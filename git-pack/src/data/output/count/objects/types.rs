/// Information gathered during the run of [`iter_from_objects()`][super::objects()].
#[derive(Default, PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Outcome {
    /// The amount of objects provided to start the iteration.
    pub input_objects: usize,
    /// The amount of objects that have been expanded from the input source.
    /// It's desirable to do that as expansion happens on multiple threads, allowing the amount of input objects to be small.
    /// `expanded_objects - decoded_objects` is the 'cheap' object we found without decoding the object itself.
    pub expanded_objects: usize,
    /// The amount of fully decoded objects. These are the most expensive as they are fully decoded
    pub decoded_objects: usize,
    /// The total amount of encountered objects. Should be `expanded_objects + input_objects`.
    pub total_objects: usize,
}

impl Outcome {
    pub(in crate::data::output::count) fn aggregate(
        &mut self,
        Outcome {
            input_objects,
            decoded_objects,
            expanded_objects,
            total_objects,
        }: Self,
    ) {
        self.input_objects += input_objects;
        self.decoded_objects += decoded_objects;
        self.expanded_objects += expanded_objects;
        self.total_objects += total_objects;
    }
}

/// The way input objects are handled
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum ObjectExpansion {
    /// Don't do anything with the input objects except for transforming them into pack entries
    AsIs,
    /// If the input object is a Commit then turn it into a pack entry. Additionally obtain its tree, turn it into a pack entry
    /// along with all of its contents, that is nested trees, and any other objects reachable from it.
    /// Otherwise, the same as [`AsIs`][ObjectExpansion::AsIs].
    ///
    /// This mode is useful if all reachable objects should be added, as in cloning a repository.
    TreeContents,
    /// If the input is a commit, obtain its ancestors and turn them into pack entries. Obtain the ancestor trees along with the commits
    /// tree and turn them into pack entries. Finally obtain the added/changed objects when comparing the ancestor trees with the
    /// current tree and turn them into entries as well.
    /// Otherwise, the same as [`AsIs`][ObjectExpansion::AsIs].
    ///
    /// This mode is useful to build a pack containing only new objects compared to a previous state.
    TreeAdditionsComparedToAncestor,
}

impl Default for ObjectExpansion {
    fn default() -> Self {
        ObjectExpansion::AsIs
    }
}

/// Configuration options for the pack generation functions provied in [this module][crate::data::output].
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Options {
    /// The amount of threads to use at most when resolving the pack. If `None`, all logical cores are used.
    /// If more than one thread is used, the order of returned [counts][crate::data::output::Count] is not deterministic anymore
    /// especially when tree traversal is involved. Thus deterministic ordering requires `Some(1)` to be set.
    pub thread_limit: Option<usize>,
    /// The amount of objects per chunk or unit of work to be sent to threads for processing
    pub chunk_size: usize,
    /// The way input objects are handled
    pub input_object_expansion: ObjectExpansion,
    /// The size of a per-thread object cache in bytes to accelerate tree diffs in conjunction
    /// with [ObjectExpansion::TreeAdditionsComparedToAncestor].
    ///
    /// If zero, the cache is disabled but in a costly way. Consider using a low value instead.
    ///
    /// Defaults to 10 megabytes which usually leads to 2.5x speedups.
    #[cfg(feature = "object-cache-dynamic")]
    pub object_cache_size_in_bytes: usize,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            thread_limit: None,
            chunk_size: 10,
            input_object_expansion: Default::default(),
            #[cfg(feature = "object-cache-dynamic")]
            object_cache_size_in_bytes: 10 * 1024 * 1024,
        }
    }
}

/// The error returned by the pack generation iterator [bytes::FromEntriesIter][crate::data::output::bytes::FromEntriesIter].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error<FindErr, IterErr>
where
    FindErr: std::error::Error + 'static,
    IterErr: std::error::Error + 'static,
{
    #[error(transparent)]
    CommitDecode(git_object::decode::Error),
    #[error(transparent)]
    FindExisting(#[from] FindErr),
    #[error(transparent)]
    InputIteration(IterErr),
    #[error(transparent)]
    TreeTraverse(git_traverse::tree::breadthfirst::Error),
    #[error(transparent)]
    TreeChanges(git_diff::tree::changes::Error),
    #[error("Operation interrupted")]
    Interrupted,
}
