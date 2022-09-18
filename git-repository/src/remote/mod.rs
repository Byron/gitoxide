/// The direction of an operation carried out (or to be carried out) through a remote.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum Direction {
    /// Push local changes to the remote.
    Push,
    /// Fetch changes from the remote to the local repository.
    Fetch,
}

impl Direction {
    /// Return ourselves as string suitable for use as verb in an english sentence.
    pub fn as_str(&self) -> &'static str {
        match self {
            Direction::Push => "push",
            Direction::Fetch => "fetch",
        }
    }
}

mod build;

mod errors;
pub use errors::find;

///
pub mod init;

///
#[cfg(any(feature = "async-network-client", feature = "blocking-network-client"))]
pub mod fetch {
    use crate::bstr::BString;

    /// Information about the relationship between our refspecs, and remote references with their local counterparts.
    #[derive(Debug, Clone)]
    pub struct RefMap<'spec> {
        /// A mapping between a remote reference and a local tracking branch.
        pub mappings: Vec<Mapping>,
        /// Information about the fixes applied to the `mapping` due to validation and sanitization.
        pub fixes: Vec<git_refspec::match_group::validate::Fix<'spec>>,
        /// All refs advertised by the remote.
        pub remote_refs: Vec<git_protocol::fetch::Ref>,
        /// Additional information provided by the server as part of the handshake.
        ///
        /// Note that the `refs` field is always `None` as the refs are placed in `remote_refs`.
        pub handshake: git_protocol::fetch::handshake::Outcome,
    }

    /// Either an object id that the remote has or the matched remote ref itself.
    #[derive(Debug, Clone)]
    pub enum Source {
        /// An object id, as the matched ref-spec was an object id itself.
        ObjectId(git_hash::ObjectId),
        /// The remote reference that matched the ref-specs name.
        Ref(git_protocol::fetch::Ref),
    }

    /// A mapping between a single remote reference and its advertised objects to a local destination which may or may not exist.
    #[derive(Debug, Clone)]
    pub struct Mapping {
        /// The reference on the remote side, along with information about the objects they point to as advertised by the server.
        pub remote: Source,
        /// The local tracking reference to update after fetching the object visible via `remote`.
        pub local: Option<BString>,
        /// The index into the fetch ref-specs used to produce the mapping, allowing it to be recovered.   
        pub spec_index: usize,
    }
}

///
#[cfg(any(feature = "async-network-client", feature = "blocking-network-client"))]
pub mod connect;

#[cfg(any(feature = "async-network-client", feature = "blocking-network-client"))]
mod connection;
#[cfg(any(feature = "async-network-client", feature = "blocking-network-client"))]
pub use connection::{ref_map, Connection};

mod access;
pub(crate) mod url;
