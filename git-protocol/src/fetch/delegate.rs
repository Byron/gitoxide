use crate::fetch::{Arguments, Ref, Response};
use bstr::BString;
use git_transport::client::Capabilities;
use std::{
    io,
    ops::{Deref, DerefMut},
};

/// Defines what to do next after certain [`Delegate`] operations.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
pub enum Action {
    /// Continue the typical flow of operations in this flow.
    Continue,
    /// Return at the next possible opportunity without making further requests, possibly after closing the connection.
    Cancel,
}

/// What to do after [`DelegateBlocking::prepare_ls_refs`].
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub enum LsRefsAction {
    /// Continue by sending a 'ls-refs' command.
    Continue,
    /// Skip 'ls-refs' entirely.
    ///
    /// This is valid if the 'ref-in-want' capability is taken advantage of. The delegate must then send 'want-ref's in
    /// [`DelegateBlocking::negotiate`].
    Skip,
}

/// The non-IO protocol delegate is the bare minimal interface needed to fully control the [`fetch`][crate::fetch()] operation, sparing
/// the IO parts.
/// Async implementations must treat it as blocking and unblock it by evaluating it elsewhere.
///
/// See [Delegate] for the complete trait.
pub trait DelegateBlocking {
    /// Return extra parameters to be provided during the handshake.
    ///
    /// Note that this method is only called once and the result is reused during subsequent handshakes which may happen
    /// if there is an authentication failure.
    fn handshake_extra_parameters(&self) -> Vec<(String, Option<String>)> {
        Vec::new()
    }
    /// Called before invoking 'ls-refs' on the server to allow providing it with additional `arguments` and to enable `features`.
    /// If the server `capabilities` don't match the requirements abort with an error to abort the entire fetch operation.
    ///
    /// Note that some arguments are preset based on typical use, and `features` are preset to maximize options.
    /// The `server` capabilities can be used to see which additional capabilities the server supports as per the handshake which happened prior.
    ///
    /// If the delegate returns [`LsRefsAction::Skip`], no 'ls-refs` command is sent to the server. This is valid if the
    /// 'ref-in-want' capability is supported by the server, and the client takes advantage of that by sending 'want-ref's
    /// in [`DelegateBlocking::negotiate`]. The delegate must check for the presence of 'ref-in-want' in `features`, and
    /// otherwise ensure that 'ls-refs` is executed.
    ///
    /// Note that this is called only if we are using protocol version 2.
    fn prepare_ls_refs(
        &mut self,
        _server: &Capabilities,
        _arguments: &mut Vec<BString>,
        _features: &mut Vec<(&str, Option<&str>)>,
    ) -> std::io::Result<LsRefsAction> {
        Ok(LsRefsAction::Continue)
    }

    /// Called before invoking the 'fetch' interaction with `features` pre-filled for typical use
    /// and to maximize capabilities to allow aborting an interaction early.
    ///
    /// `refs` is a list of known references on the remote based on the handshake or a prior call to ls_refs.
    /// These can be used to abort early in case the refs are already known here.
    ///
    /// As there will be another call allowing to post arguments conveniently in the correct format, i.e. `want hex-oid`,
    /// there is no way to set arguments at this time.
    ///
    /// `version` is the actually supported version as reported by the server, which is relevant in case the server requested a downgrade.
    /// `server` capabilities is a list of features the server supports for your information, along with enabled `features` that the server knows about.
    fn prepare_fetch(
        &mut self,
        _version: git_transport::Protocol,
        _server: &Capabilities,
        _features: &mut Vec<(&str, Option<&str>)>,
        _refs: &[Ref],
    ) -> std::io::Result<Action> {
        Ok(Action::Continue)
    }

    /// A method called repeatedly to negotiate the objects to receive in [`receive_pack(…)`][Delegate::receive_pack()].
    ///
    /// The first call has `previous` set to `None` as there was no previous response. Every call that follows `previous`
    /// will be set to `Some`.
    ///
    /// ### If `previous` is `None`…
    ///
    /// Given a list of `arguments` to populate with wants, want-refs, shallows, filters and other contextual information to be
    /// sent to the server. This method is called once.
    /// Send the objects you `have` have afterwards based on the tips of your refs, in preparation to walk down their parents
    /// with each call to `negotiate` to find the common base(s).
    ///
    /// Note that you should not `want` and object that you already have.
    /// `refs` are the the tips of on the server side, effectively the latest objects _they_ have.
    ///
    /// Return `Action::Close` if you know that there are no `haves` on your end to allow the server to send all of its objects
    /// as is the case during initial clones.
    ///
    /// ### If `previous` is `Some`…
    ///
    /// Populate `arguments` with the objects you `have` starting from the tips of _your_ refs, taking into consideration
    /// the `previous` response of the server to see which objects they acknowledged to have. You have to maintain
    /// enough state to be able to walk down from your tips on each call, if they are not in common, and keep setting `have`
    /// for those which are in common if that helps teaching the server about our state and to acknowledge their existence on _their_ end.
    /// This method is called until the other side signals they are ready to send a pack.
    /// Return `Action::Close` if you want to give up before finding a common base. This can happen if the remote repository
    /// has radically changed so there are no bases, or they are very far in the past, causing all objects to be sent.
    ///
    /// ### 'ref-in-want'
    ///
    /// The 'ref-in-want' feature requires special attention: 'want-refs' need to be
    /// [added to the arguments][Arguments::want_ref()] on the **first** call of this method (when `previous` is
    /// `None`). The `Action` must in this case be [`Action::Continue`], as the server's 'wanted-refs' response will be
    /// available only on the next turn.
    fn negotiate(&mut self, refs: &[Ref], arguments: &mut Arguments, previous: Option<&Response>)
        -> io::Result<Action>;
}

impl<T: DelegateBlocking> DelegateBlocking for Box<T> {
    fn handshake_extra_parameters(&self) -> Vec<(String, Option<String>)> {
        self.deref().handshake_extra_parameters()
    }

    fn prepare_ls_refs(
        &mut self,
        _server: &Capabilities,
        _arguments: &mut Vec<BString>,
        _features: &mut Vec<(&str, Option<&str>)>,
    ) -> io::Result<LsRefsAction> {
        self.deref_mut().prepare_ls_refs(_server, _arguments, _features)
    }

    fn prepare_fetch(
        &mut self,
        _version: git_transport::Protocol,
        _server: &Capabilities,
        _features: &mut Vec<(&str, Option<&str>)>,
        _refs: &[Ref],
    ) -> io::Result<Action> {
        self.deref_mut().prepare_fetch(_version, _server, _features, _refs)
    }

    fn negotiate(
        &mut self,
        refs: &[Ref],
        arguments: &mut Arguments,
        previous: Option<&Response>,
    ) -> io::Result<Action> {
        self.deref_mut().negotiate(refs, arguments, previous)
    }
}

impl<T: DelegateBlocking> DelegateBlocking for &mut T {
    fn handshake_extra_parameters(&self) -> Vec<(String, Option<String>)> {
        self.deref().handshake_extra_parameters()
    }

    fn prepare_ls_refs(
        &mut self,
        _server: &Capabilities,
        _arguments: &mut Vec<BString>,
        _features: &mut Vec<(&str, Option<&str>)>,
    ) -> io::Result<LsRefsAction> {
        self.deref_mut().prepare_ls_refs(_server, _arguments, _features)
    }

    fn prepare_fetch(
        &mut self,
        _version: git_transport::Protocol,
        _server: &Capabilities,
        _features: &mut Vec<(&str, Option<&str>)>,
        _refs: &[Ref],
    ) -> io::Result<Action> {
        self.deref_mut().prepare_fetch(_version, _server, _features, _refs)
    }

    fn negotiate(
        &mut self,
        refs: &[Ref],
        arguments: &mut Arguments,
        previous: Option<&Response>,
    ) -> io::Result<Action> {
        self.deref_mut().negotiate(refs, arguments, previous)
    }
}

#[cfg(feature = "blocking-client")]
mod blocking_io {
    use crate::fetch::{DelegateBlocking, Ref, Response};
    use git_features::progress::Progress;
    use std::{
        io::{self, BufRead},
        ops::DerefMut,
    };

    /// The protocol delegate is the bare minimal interface needed to fully control the [`fetch`][crate::fetch()] operation.
    ///
    /// Implementations of this trait are controlled by code with intricate knowledge about how fetching works in protocol version V1 and V2,
    /// so you don't have to.
    /// Everything is tucked away behind type-safety so 'nothing can go wrong'©. Runtime assertions assure invalid
    /// features or arguments don't make it to the server in the first place.
    /// Please note that this trait mostly corresponds to what V2 would look like, even though V1 is supported as well.
    pub trait Delegate: DelegateBlocking {
        /// Receive a pack provided from the given `input`.
        ///
        /// Use `progress` to emit your own progress messages when decoding the pack.
        ///
        /// `refs` of the remote side are provided for convenience, along with the parsed `previous` response in case you want
        /// to check additional acks.
        fn receive_pack(
            &mut self,
            input: impl io::BufRead,
            progress: impl Progress,
            refs: &[Ref],
            previous: &Response,
        ) -> io::Result<()>;
    }

    impl<T: Delegate> Delegate for Box<T> {
        fn receive_pack(
            &mut self,
            input: impl BufRead,
            progress: impl Progress,
            refs: &[Ref],
            previous: &Response,
        ) -> io::Result<()> {
            self.deref_mut().receive_pack(input, progress, refs, previous)
        }
    }

    impl<T: Delegate> Delegate for &mut T {
        fn receive_pack(
            &mut self,
            input: impl BufRead,
            progress: impl Progress,
            refs: &[Ref],
            previous: &Response,
        ) -> io::Result<()> {
            self.deref_mut().receive_pack(input, progress, refs, previous)
        }
    }
}
#[cfg(feature = "blocking-client")]
pub use blocking_io::Delegate;

#[cfg(feature = "async-client")]
mod async_io {
    use crate::fetch::{DelegateBlocking, Ref, Response};
    use async_trait::async_trait;
    use futures_io::AsyncBufRead;
    use git_features::progress::Progress;
    use std::{io, ops::DerefMut};

    /// The protocol delegate is the bare minimal interface needed to fully control the [`fetch`][crate::fetch()] operation.
    ///
    /// Implementations of this trait are controlled by code with intricate knowledge about how fetching works in protocol version V1 and V2,
    /// so you don't have to.
    /// Everything is tucked away behind type-safety so 'nothing can go wrong'©. Runtime assertions assure invalid
    /// features or arguments don't make it to the server in the first place.
    /// Please note that this trait mostly corresponds to what V2 would look like, even though V1 is supported as well.
    #[async_trait(?Send)]
    pub trait Delegate: DelegateBlocking {
        /// Receive a pack provided from the given `input`, and the caller should consider it to be blocking as
        /// most operations on the received pack are implemented in a blocking fashion.
        ///
        /// Use `progress` to emit your own progress messages when decoding the pack.
        ///
        /// `refs` of the remote side are provided for convenience, along with the parsed `previous` response in case you want
        /// to check additional acks.
        async fn receive_pack(
            &mut self,
            input: impl AsyncBufRead + Unpin + 'async_trait,
            progress: impl Progress,
            refs: &[Ref],
            previous: &Response,
        ) -> io::Result<()>;
    }
    #[async_trait(?Send)]
    impl<T: Delegate> Delegate for Box<T> {
        async fn receive_pack(
            &mut self,
            input: impl AsyncBufRead + Unpin + 'async_trait,
            progress: impl Progress,
            refs: &[Ref],
            previous: &Response,
        ) -> io::Result<()> {
            self.deref_mut().receive_pack(input, progress, refs, previous).await
        }
    }

    #[async_trait(?Send)]
    impl<T: Delegate> Delegate for &mut T {
        async fn receive_pack(
            &mut self,
            input: impl AsyncBufRead + Unpin + 'async_trait,
            progress: impl Progress,
            refs: &[Ref],
            previous: &Response,
        ) -> io::Result<()> {
            self.deref_mut().receive_pack(input, progress, refs, previous).await
        }
    }
}
#[cfg(feature = "async-client")]
pub use async_io::Delegate;
