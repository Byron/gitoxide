use crate::fetch::Ref;
use bstr::BString;
use git_transport::client::Capabilities;

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
}
