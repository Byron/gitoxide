use crate::Remote;

pub(crate) struct HandshakeWithRefs {
    outcome: git_protocol::fetch::handshake::Outcome,
    refs: Vec<git_protocol::fetch::Ref>,
}

/// A function that performs a given credential action.
pub type CredentialsFn<'a> = Box<dyn FnMut(git_credentials::helper::Action) -> git_credentials::protocol::Result + 'a>;

/// A type to represent an ongoing connection to a remote host, typically with the connection already established.
///
/// It can be used to perform a variety of operations with the remote without worrying about protocol details,
/// much like a remote procedure call.
pub struct Connection<'a, 'repo, T, P> {
    pub(crate) remote: &'a Remote<'repo>,
    pub(crate) credentials: Option<CredentialsFn<'a>>,
    pub(crate) transport: T,
    pub(crate) progress: P,
}

mod access;

///
pub mod ref_map;

///
#[cfg(feature = "blocking-network-client")]
pub mod fetch;
