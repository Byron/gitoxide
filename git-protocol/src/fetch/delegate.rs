use crate::fetch::{Arguments, Ref, Response};
use bstr::BString;
use git_features::progress::Progress;
use git_transport::client::Capabilities;
use std::io;

/// Define what to do next.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
pub enum Action {
    /// Continue the typical flow of operations in this flow.
    Continue,
    /// Close the connection without making any further requests.
    Close,
}

/// The protocol delegate is the bare minimal interface needed to fully control the 'fetch' operation.
/// It's controlled by code with intricate knowledge about how that works in protocol version V1 and V2,
/// so you don't have to.
/// Everything is tucked away behind type-safety so nothing can go wrong. Runtime assertions assure invalid
/// features or arguments don't make it to the server in the first place.
/// Please note that this trait mostly corresponds to what V2 would look like.
pub trait Delegate {
    /// Called before invoking ls-refs to allow providing it with additional `arguments` and to enable `features`.
    /// Note that some arguments are preset based on typical use, and `features` are preset to maximize options.
    /// The `server` capabilities can be used to see which additional capabilities the server supports as per the handshake.
    /// Note that this is called only if we are using protocol version 2.
    fn prepare_ls_refs(
        &mut self,
        _server: &Capabilities,
        _arguments: &mut Vec<BString>,
        _features: &mut Vec<(&str, Option<&str>)>,
    ) {
    }

    /// Called before invoking the 'fetch' interaction, with `features` pre-filled for typical use
    /// and to maximize capabilities.
    /// `refs` is a list of known references on the remote, based on the handshake or a prior call to ls_refs.
    /// These can be used to abort early in case the refs are already known here.
    /// As there will be another call allowing to post arguments conveniently in the correct format, i.e. `want hex-oid`,
    /// there is no way to set arguments at this time.
    fn prepare_fetch(
        &mut self,
        _version: git_transport::Protocol,
        _server: &Capabilities,
        _features: &mut Vec<(&str, Option<&str>)>,
        _refs: &[Ref],
    ) -> Action {
        Action::Continue
    }

    /// ### `previous` is None
    /// Given a list of `arguments` to populate to be send to the remote to start negotiation with wants, shallows, filters and
    /// other contextual information. This method is called once.
    /// Send the objects you `have` have afterwards based on your tips, in preparation to walk down their parents with each call
    /// to `negotiate` in find the common base.
    /// Note that you should not `want` and object that you already have.
    /// `refs` are the the tips of on the server side, effectively the latest objects they have.
    ///
    /// Return `Action::Close` if you know that there are no `haves` on your end to allow the server to send all of its objects,
    /// as done during clone.
    ///
    /// ### `previous` is Some
    /// Populate `arguments` with the objects you `have` starting from the tips, taking into consideration the `previous` response of the server to see
    /// which objects they acknowledged to have. You have to maintain enough state to be able to walk down from your tips on each call,
    /// if they are not in common, and keep setting `have` for those which are in common if that helps teaching the server about our state.
    /// This method is called until the other side signals they are ready to send a pack.
    /// Return `Action::Close` if you want to give up before finding a common base. This can happen if the remote repository has radically changed
    /// so there are no bases, or they are very far in the past.
    fn negotiate(&mut self, refs: &[Ref], arguments: &mut Arguments, previous: Option<&Response>) -> Action;

    /// Receive a pack provided from the given `input`. `refs` are provided to not hide any context, along with the
    /// parsed response in case you want to check additional acks.
    fn receive_pack(
        &mut self,
        input: impl io::BufRead,
        progress: impl Progress,
        refs: &[Ref],
        previous: &Response,
    ) -> io::Result<()>;
}
