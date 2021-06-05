use crate::fetch::{Arguments, Ref, Response};
use bstr::BString;
use git_features::progress::Progress;
use git_transport::client::Capabilities;
use std::io;

/// Defines what to do next after certain [`Delegate`] operations.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
pub enum Action {
    /// Continue the typical flow of operations in this flow.
    Continue,
    /// Close the connection without making any further requests.
    Close,
}

/// The protocol delegate is the bare minimal interface needed to fully control the [`fetch`][crate::fetch()] operation.
///
/// Implementations of this trait are controlled by code with intricate knowledge about how fetching works in protocol version V1 and V2,
/// so you don't have to.
/// Everything is tucked away behind type-safety so 'nothing can go wrong'©. Runtime assertions assure invalid
/// features or arguments don't make it to the server in the first place.
/// Please note that this trait mostly corresponds to what V2 would look like, even though V1 is supported as well.
pub trait Delegate {
    /// Called before invoking 'ls-refs' on the server to allow providing it with additional `arguments` and to enable `features`.
    ///
    /// Note that some arguments are preset based on typical use, and `features` are preset to maximize options.
    /// The `server` capabilities can be used to see which additional capabilities the server supports as per the handshake which happened prior.
    /// Note that this is called only if we are using protocol version 2.
    fn prepare_ls_refs(
        &mut self,
        _server: &Capabilities,
        _arguments: &mut Vec<BString>,
        _features: &mut Vec<(&str, Option<&str>)>,
    ) {
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
    ) -> Action {
        Action::Continue
    }

    /// A method called repeatedly to negotiate the objects to receive in [`receive_pack(…)`][Delegate::receive_pack()].
    ///
    /// The first call has `previous` set to `None` as there was no previous response. Every call that follows `previous`
    /// will be set to `Some`.
    ///
    /// ### If `previous` is `None`…
    ///
    /// Given a list of `arguments` to populate with wants, shallows, filters and other contextual information to be
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
    fn negotiate(&mut self, refs: &[Ref], arguments: &mut Arguments, previous: Option<&Response>) -> Action;

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
