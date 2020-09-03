use crate::fetch::{Capabilities, Ref};
use bstr::BString;

/// Define what to do next.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
pub enum Action {
    /// Continue the typical flow of operations in this flow.
    Continue,
    /// Close the connection without making any further requests.
    Close,
}

pub trait Delegate {
    /// A chance to inspect or adjust the Capabilities returned after handshake with the server.
    /// They will be used in subsequent calls to the server, but the client is free to cache information as they see fit.
    fn adjust_capabilities(&mut self, _version: git_transport::Protocol, _capabilities: &mut Capabilities) {}

    /// Called before invoking ls-refs to allow providing it with additional `arguments` and to enable `features`.
    /// Note that some arguments are preset based on typical usage.
    /// The `server` capabilities can be used to see which additional capabilities the server supports as per the handshake.
    /// Note that this is called only if we are using protocol version 2.
    fn prepare_ls_refs(
        &mut self,
        _server: &Capabilities,
        _arguments: &mut Vec<BString>,
        _features: &mut Vec<(&str, Option<&str>)>,
    ) {
    }

    /// Called before invoking the 'fetch' interaction, with `arguments` and `features` pre-filled for typical use.
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
