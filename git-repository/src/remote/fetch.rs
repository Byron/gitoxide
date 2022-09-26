use crate::bstr::BString;

/// Information about the relationship between our refspecs, and remote references with their local counterparts.
#[derive(Default, Debug, Clone)]
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

impl Source {
    /// Return either the direct object id we refer to or the direct target that a reference refers to.
    pub fn as_id(&self) -> &git_hash::oid {
        match self {
            Source::ObjectId(id) => id,
            Source::Ref(r) => r.unpack().1,
        }
    }
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

/// The status of the repository after the fetch operation
#[derive(Debug, Clone)]
pub enum Status {
    /// Nothing changed as the remote didn't have anything new.
    NoChange,
    /// There was at least one tip with a new object which we received.
    Change {
        /// Information collected while writing the pack and its index.
        write_pack_bundle: git_pack::bundle::write::Outcome,
        /// Information collected while updating references.
        update_refs: refs::update::Outcome,
    },
}

/// The outcome of receiving a pack via [`Prepare::receive()`].
#[derive(Debug, Clone)]
pub struct Outcome<'spec> {
    /// The result of the initial mapping of references, the prerequisite for any fetch.
    pub ref_map: RefMap<'spec>,
    /// The status of the operation to indicate what happened.
    pub status: Status,
}

#[cfg(feature = "blocking-network-client")]
pub use super::connection::fetch::{negotiate, refs, Error, Prepare};
