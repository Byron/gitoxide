use crate::bstr::{BStr, BString};

/// If `Yes`, don't really make changes but do as much as possible to get an idea of what would be done.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg(feature = "blocking-network-client")]
pub(crate) enum DryRun {
    /// Enable dry-run mode and don't actually change the underlying repository in any way.
    Yes,
    /// Run the operation like normal, making changes to the underlying repository.
    No,
}

/// Information about the relationship between our refspecs, and remote references with their local counterparts.
#[derive(Default, Debug, Clone)]
pub struct RefMap {
    /// A mapping between a remote reference and a local tracking branch.
    pub mappings: Vec<Mapping>,
    /// Information about the fixes applied to the `mapping` due to validation and sanitization.
    pub fixes: Vec<git_refspec::match_group::validate::Fix>,
    /// All refs advertised by the remote.
    pub remote_refs: Vec<git_protocol::fetch::Ref>,
    /// Additional information provided by the server as part of the handshake.
    ///
    /// Note that the `refs` field is always `None` as the refs are placed in `remote_refs`.
    pub handshake: git_protocol::fetch::handshake::Outcome,
    /// The kind of hash used for all data sent by the server, if understood by this client implementation.
    ///
    /// It was extracted from the `handshake` as advertised by the server.
    pub object_hash: git_hash::Kind,
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
    /// The latter may be a direct or a symbolic reference, and we degenerate this to the peeled object id.
    pub fn as_id(&self) -> &git_hash::oid {
        match self {
            Source::ObjectId(id) => id,
            Source::Ref(r) => r.unpack().1,
        }
    }

    /// Return ourselves as the full name of the reference we represent, or `None` if this source isn't a reference but an object.
    pub fn as_name(&self) -> Option<&BStr> {
        match self {
            Source::ObjectId(_) => None,
            Source::Ref(r) => match r {
                git_protocol::fetch::Ref::Unborn { target: _ } => Some("HEAD".into()),
                git_protocol::fetch::Ref::Symbolic { full_ref_name, .. }
                | git_protocol::fetch::Ref::Direct { full_ref_name, .. }
                | git_protocol::fetch::Ref::Peeled { full_ref_name, .. } => Some(full_ref_name.as_ref()),
            },
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

#[cfg(feature = "blocking-network-client")]
pub use super::connection::fetch::{negotiate, prepare, refs, Error, Outcome, Prepare, RefLogMessage, Status};
