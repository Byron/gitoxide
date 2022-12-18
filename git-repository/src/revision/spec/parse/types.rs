use crate::{bstr::BString, object, reference};

/// A hint to know what to do if refs and object names are equal.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RefsHint {
    /// This is the default, and leads to specs that look like objects identified by full hex sha and are objets to be used
    /// instead of similarly named references. The latter is not typical but can absolutely happen by accident.
    /// If the object prefix is shorter than the maximum hash length of the repository, use the reference instead, which is
    /// preferred as there are many valid object names like `beef` and `cafe` that are short and both valid and typical prefixes
    /// for objects.
    /// Git chooses this as default as well, even though it means that every object prefix is also looked up as ref.
    PreferObjectOnFullLengthHexShaUseRefOtherwise,
    /// No matter what, if it looks like an object prefix and has an object, use it.
    /// Note that no ref-lookup is made here which is the fastest option.
    PreferObject,
    /// When an object is found for a given prefix, also check if a reference exists with that name and if it does,
    /// use that moving forward.
    PreferRef,
    /// If there is an ambiguous situation, instead of silently choosing one over the other, fail instead.
    Fail,
}

/// A hint to know which object kind to prefer if multiple objects match a prefix.
///
/// This disambiguation mechanism is applied only if there is no disambiguation hints in the spec itself.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ObjectKindHint {
    /// Pick objects that are commits themselves.
    Commit,
    /// Pick objects that can be peeled into a commit, i.e. commits themselves or tags which are peeled until a commit is found.
    Committish,
    /// Pick objects that are trees themselves.
    Tree,
    /// Pick objects that can be peeled into a tree, i.e. trees themselves or tags which are peeled until a tree is found or commits
    /// whose tree is chosen.
    Treeish,
    /// Pick objects that are blobs.
    Blob,
}

impl Default for RefsHint {
    fn default() -> Self {
        RefsHint::PreferObjectOnFullLengthHexShaUseRefOtherwise
    }
}

/// Options for use in [`revision::Spec::from_bstr()`][crate::revision::Spec::from_bstr()].
#[derive(Debug, Default, Copy, Clone)]
pub struct Options {
    /// What to do if both refs and object names match the same input.
    pub refs_hint: RefsHint,
    /// The hint to use when encountering multiple object matching a prefix.
    ///
    /// If `None`, the rev-spec itself must disambiguate the object by drilling down to desired kinds or applying
    /// other disambiguating transformations.
    pub object_kind_hint: Option<ObjectKindHint>,
}

/// The error returned by [`crate::Repository::rev_parse()`].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Unborn heads do not have a reflog yet")]
    UnbornHeadsHaveNoRefLog,
    #[error("This feature will be implemented once {dependency}")]
    Planned { dependency: &'static str },
    #[error("Reference {reference:?} does not have a reference log, cannot {action}")]
    MissingRefLog { reference: BString, action: &'static str },
    #[error("HEAD has {available} prior checkouts and checkout number {desired} is out of range")]
    PriorCheckoutOutOfRange { desired: usize, available: usize },
    #[error("Reference {:?} has {available} ref-log entries and entry number {desired} is out of range", reference.name.as_bstr())]
    RefLogEntryOutOfRange {
        reference: git_ref::Reference,
        desired: usize,
        available: usize,
    },
    #[error(
        "Commit {oid} has {available} ancestors along the first parent and ancestor number {desired} is out of range"
    )]
    AncestorOutOfRange {
        oid: git_hash::Prefix,
        desired: usize,
        available: usize,
    },
    #[error("Commit {oid} has {available} parents and parent number {desired} is out of range")]
    ParentOutOfRange {
        oid: git_hash::Prefix,
        desired: usize,
        available: usize,
    },
    #[error("Path {desired_path:?} did not exist in index at stage {desired_stage}{}{}", stage_hint.map(|actual|format!(". It does exist at stage {actual}")).unwrap_or_default(), exists.then(|| ". It exists on disk").unwrap_or(". It does not exist on disk"))]
    IndexLookup {
        desired_path: BString,
        desired_stage: git_index::entry::Stage,
        stage_hint: Option<git_index::entry::Stage>,
        exists: bool,
    },
    #[error(transparent)]
    FindHead(#[from] reference::find::existing::Error),
    #[error(transparent)]
    Index(#[from] crate::worktree::open_index::Error),
    #[error(transparent)]
    RevWalkIterInit(#[from] crate::reference::iter::init::Error),
    #[error(transparent)]
    RevWalkAllReferences(#[from] git_ref::packed::buffer::open::Error),
    #[cfg(feature = "regex")]
    #[error(transparent)]
    InvalidRegex(#[from] regex::Error),
    #[cfg_attr(
        feature = "regex",
        error("None of {commits_searched} commits from {oid} matched regex {regex:?}")
    )]
    #[cfg_attr(
        not(feature = "regex"),
        error("None of {commits_searched} commits from {oid} matched text {regex:?}")
    )]
    NoRegexMatch {
        regex: BString,
        oid: git_hash::Prefix,
        commits_searched: usize,
    },
    #[cfg_attr(
        feature = "regex",
        error("None of {commits_searched} commits reached from all references matched regex {regex:?}")
    )]
    #[cfg_attr(
        not(feature = "regex"),
        error("None of {commits_searched} commits reached from all references matched text {regex:?}")
    )]
    NoRegexMatchAllRefs { regex: BString, commits_searched: usize },
    #[error(
    "The short hash {prefix} matched both the reference {} and at least one object", reference.name)]
    AmbiguousRefAndObject {
        /// The prefix to look for.
        prefix: git_hash::Prefix,
        /// The reference matching the prefix.
        reference: git_ref::Reference,
    },
    #[error(transparent)]
    IdFromHex(#[from] git_hash::decode::Error),
    #[error(transparent)]
    FindReference(#[from] git_ref::file::find::existing::Error),
    #[error(transparent)]
    FindObject(#[from] object::find::existing::Error),
    #[error(transparent)]
    LookupPrefix(#[from] git_odb::store::prefix::lookup::Error),
    #[error(transparent)]
    PeelToKind(#[from] object::peel::to_kind::Error),
    #[error("Object {oid} was a {actual}, but needed it to be a {expected}")]
    ObjectKind {
        oid: git_hash::Prefix,
        actual: git_object::Kind,
        expected: git_object::Kind,
    },
    #[error(transparent)]
    Parse(#[from] git_revision::spec::parse::Error),
    #[error("An object prefixed {prefix} could not be found")]
    PrefixNotFound { prefix: git_hash::Prefix },
    #[error("Short id {prefix} is ambiguous. Candidates are:\n{}", info.iter().map(|(oid, info)| format!("\t{oid} {info}")).collect::<Vec<_>>().join("\n"))]
    AmbiguousPrefix {
        prefix: git_hash::Prefix,
        info: Vec<(git_hash::Prefix, super::error::CandidateInfo)>,
    },
    #[error("Could not find path {path:?} in tree {tree} of parent object {object}")]
    PathNotFound {
        object: git_hash::Prefix,
        tree: git_hash::Prefix,
        path: BString,
    },
    #[error("{current}")]
    Multi {
        current: Box<dyn std::error::Error + Send + Sync + 'static>,
        #[source]
        next: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    },
    #[error(transparent)]
    Traverse(#[from] git_traverse::commit::ancestors::Error),
    #[error("Spec does not contain a single object id")]
    SingleNotFound,
}
