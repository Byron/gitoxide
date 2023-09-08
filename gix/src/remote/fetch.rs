///
pub mod negotiate {
    #[cfg(feature = "credentials")]
    pub use gix_negotiate::Algorithm;

    #[cfg(any(feature = "blocking-network-client", feature = "async-network-client"))]
    pub use super::super::connection::fetch::negotiate::Error;
    #[cfg(any(feature = "blocking-network-client", feature = "async-network-client"))]
    pub(crate) use super::super::connection::fetch::negotiate::{
        add_wants, make_refmapping_ignore_predicate, mark_complete_and_common_ref, one_round, Action,
    };
}

#[cfg(any(feature = "blocking-network-client", feature = "async-network-client"))]
pub use super::connection::fetch::{
    outcome, prepare, refs, Error, Outcome, Prepare, ProgressId, RefLogMessage, Status,
};

/// If `Yes`, don't really make changes but do as much as possible to get an idea of what would be done.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg(any(feature = "blocking-network-client", feature = "async-network-client"))]
pub(crate) enum DryRun {
    /// Enable dry-run mode and don't actually change the underlying repository in any way.
    Yes,
    /// Run the operation like normal, making changes to the underlying repository.
    No,
}

/// How to deal with refs when cloning or fetching.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg(any(feature = "blocking-network-client", feature = "async-network-client"))]
pub(crate) enum WritePackedRefs {
    /// Normal operation, i.e. don't use packed-refs at all for writing.
    Never,
    /// Put ref updates straight into the `packed-refs` file, without creating loose refs first or dealing with them in any way.
    Only,
}

/// Describe how to handle tags when fetching
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum Tags {
    /// Fetch all tags from the remote, even if these are not reachable from objects referred to by our refspecs.
    All,
    /// Fetch only the tags that point to the objects being sent.
    /// That way, annotated tags that point to an object we receive are automatically transmitted and their refs are created.
    /// The same goes for lightweight tags.
    #[default]
    Included,
    /// Do not fetch any tags.
    None,
}

impl Tags {
    /// Obtain a refspec that determines whether or not to fetch all tags, depending on this variant.
    ///
    /// The returned refspec is the default refspec for tags, but won't overwrite local tags ever.
    pub fn to_refspec(&self) -> Option<gix_refspec::RefSpecRef<'static>> {
        match self {
            Tags::All | Tags::Included => Some(
                gix_refspec::parse("refs/tags/*:refs/tags/*".into(), gix_refspec::parse::Operation::Fetch)
                    .expect("valid"),
            ),
            Tags::None => None,
        }
    }
}

/// Describe how shallow clones are handled when fetching, with variants defining how the *shallow boundary* is handled.
///
/// The *shallow boundary* is a set of commits whose parents are not present in the repository.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum Shallow {
    /// Fetch all changes from the remote without affecting the shallow boundary at all.
    ///
    /// This also means that repositories that aren't shallow will remain like that.
    #[default]
    NoChange,
    /// Receive update to `depth` commits in the history of the refs to fetch (from the viewpoint of the remote),
    /// with the value of `1` meaning to receive only the commit a ref is pointing to.
    ///
    /// This may update the shallow boundary to increase or decrease the amount of available history.
    DepthAtRemote(std::num::NonZeroU32),
    /// Increase the number of commits and thus expand the shallow boundary by `depth` commits as seen from our local
    /// shallow boundary, with a value of `0` having no effect.
    Deepen(u32),
    /// Set the shallow boundary at the `cutoff` time, meaning that there will be no commits beyond that time.
    Since {
        /// The date beyond which there will be no history.
        cutoff: gix_date::Time,
    },
    /// Receive all history excluding all commits reachable from `remote_refs`. These can be long or short
    /// ref names or tag names.
    Exclude {
        /// The ref names to exclude, short or long. Note that ambiguous short names will cause the remote to abort
        /// without an error message being transferred (because the protocol does not support it)
        remote_refs: Vec<gix_ref::PartialName>,
        /// If some, this field has the same meaning as [`Shallow::Since`] which can be used in combination
        /// with excluded references.
        since_cutoff: Option<gix_date::Time>,
    },
}

impl Shallow {
    /// Produce a variant that causes the repository to loose its shallow boundary, effectively by extending it
    /// beyond all limits.
    pub fn undo() -> Self {
        Shallow::DepthAtRemote((i32::MAX as u32).try_into().expect("valid at compile time"))
    }
}

/// Information about the relationship between our refspecs, and remote references with their local counterparts.
#[derive(Default, Debug, Clone)]
#[cfg(any(feature = "blocking-network-client", feature = "async-network-client"))]
pub struct RefMap {
    /// A mapping between a remote reference and a local tracking branch.
    pub mappings: Vec<Mapping>,
    /// Refspecs which have been added implicitly due to settings of the `remote`, possibly pre-initialized from
    /// [`extra_refspecs` in RefMap options][crate::remote::ref_map::Options::extra_refspecs].
    ///
    /// They are never persisted nor are they typically presented to the user.
    pub extra_refspecs: Vec<gix_refspec::RefSpec>,
    /// Information about the fixes applied to the `mapping` due to validation and sanitization.
    pub fixes: Vec<gix_refspec::match_group::validate::Fix>,
    /// All refs advertised by the remote.
    pub remote_refs: Vec<gix_protocol::handshake::Ref>,
    /// Additional information provided by the server as part of the handshake.
    ///
    /// Note that the `refs` field is always `None` as the refs are placed in `remote_refs`.
    pub handshake: gix_protocol::handshake::Outcome,
    /// The kind of hash used for all data sent by the server, if understood by this client implementation.
    ///
    /// It was extracted from the `handshake` as advertised by the server.
    pub object_hash: gix_hash::Kind,
}

/// Either an object id that the remote has or the matched remote ref itself.
#[derive(Debug, Clone)]
#[cfg(any(feature = "blocking-network-client", feature = "async-network-client"))]
pub enum Source {
    /// An object id, as the matched ref-spec was an object id itself.
    ObjectId(gix_hash::ObjectId),
    /// The remote reference that matched the ref-specs name.
    Ref(gix_protocol::handshake::Ref),
}

#[cfg(any(feature = "blocking-network-client", feature = "async-network-client"))]
impl Source {
    /// Return either the direct object id we refer to or the direct target that a reference refers to.
    /// The latter may be a direct or a symbolic reference.
    /// If unborn, `None` is returned.
    pub fn as_id(&self) -> Option<&gix_hash::oid> {
        match self {
            Source::ObjectId(id) => Some(id),
            Source::Ref(r) => r.unpack().1,
        }
    }

    /// Return the target that this symbolic ref is pointing to, or `None` if it is no symbolic ref.
    pub fn as_target(&self) -> Option<&crate::bstr::BStr> {
        match self {
            Source::ObjectId(_) => None,
            Source::Ref(r) => match r {
                gix_protocol::handshake::Ref::Peeled { .. } | gix_protocol::handshake::Ref::Direct { .. } => None,
                gix_protocol::handshake::Ref::Symbolic { target, .. }
                | gix_protocol::handshake::Ref::Unborn { target, .. } => Some(target.as_ref()),
            },
        }
    }

    /// Returns the peeled id of this instance, that is the object that can't be de-referenced anymore.
    pub fn peeled_id(&self) -> Option<&gix_hash::oid> {
        match self {
            Source::ObjectId(id) => Some(id),
            Source::Ref(r) => {
                let (_name, target, peeled) = r.unpack();
                peeled.or(target)
            }
        }
    }

    /// Return ourselves as the full name of the reference we represent, or `None` if this source isn't a reference but an object.
    pub fn as_name(&self) -> Option<&crate::bstr::BStr> {
        match self {
            Source::ObjectId(_) => None,
            Source::Ref(r) => match r {
                gix_protocol::handshake::Ref::Unborn { full_ref_name, .. }
                | gix_protocol::handshake::Ref::Symbolic { full_ref_name, .. }
                | gix_protocol::handshake::Ref::Direct { full_ref_name, .. }
                | gix_protocol::handshake::Ref::Peeled { full_ref_name, .. } => Some(full_ref_name.as_ref()),
            },
        }
    }
}

/// An index into various lists of refspecs that have been used in a [Mapping] of remote references to local ones.
#[cfg(any(feature = "blocking-network-client", feature = "async-network-client"))]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum SpecIndex {
    /// An index into the _refspecs of the remote_ that triggered a fetch operation.
    /// These refspecs are explicit and visible to the user.
    ExplicitInRemote(usize),
    /// An index into the list of [extra refspecs][crate::remote::fetch::RefMap::extra_refspecs] that are implicit
    /// to a particular fetch operation.
    Implicit(usize),
}

#[cfg(any(feature = "blocking-network-client", feature = "async-network-client"))]
impl SpecIndex {
    /// Depending on our index variant, get the index either from `refspecs` or from `extra_refspecs` for `Implicit` variants.
    pub fn get<'a>(
        self,
        refspecs: &'a [gix_refspec::RefSpec],
        extra_refspecs: &'a [gix_refspec::RefSpec],
    ) -> Option<&'a gix_refspec::RefSpec> {
        match self {
            SpecIndex::ExplicitInRemote(idx) => refspecs.get(idx),
            SpecIndex::Implicit(idx) => extra_refspecs.get(idx),
        }
    }

    /// If this is an `Implicit` variant, return its index.
    pub fn implicit_index(self) -> Option<usize> {
        match self {
            SpecIndex::Implicit(idx) => Some(idx),
            SpecIndex::ExplicitInRemote(_) => None,
        }
    }
}

/// A mapping between a single remote reference and its advertised objects to a local destination which may or may not exist.
#[derive(Debug, Clone)]
#[cfg(any(feature = "blocking-network-client", feature = "async-network-client"))]
pub struct Mapping {
    /// The reference on the remote side, along with information about the objects they point to as advertised by the server.
    pub remote: Source,
    /// The local tracking reference to update after fetching the object visible via `remote`.
    pub local: Option<crate::bstr::BString>,
    /// The index into the fetch ref-specs used to produce the mapping, allowing it to be recovered.   
    pub spec_index: SpecIndex,
}
