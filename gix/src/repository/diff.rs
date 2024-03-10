use crate::Repository;

///
#[allow(clippy::empty_docs)]
pub mod resource_cache {
    /// The error returned by [Repository::diff_resource_cache()](super::Repository::diff_resource_cache()).
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("Could not obtain resource cache for diffing")]
        ResourceCache(#[from] crate::diff::resource_cache::Error),
        #[error(transparent)]
        Index(#[from] crate::repository::index_or_load_from_head::Error),
        #[error(transparent)]
        AttributeStack(#[from] crate::config::attribute_stack::Error),
    }
}

/// Diff-utilities
impl Repository {
    /// Create a resource cache for diffable objects, and configured with everything it needs to know to perform diffs
    /// faithfully just like `git` would.
    /// `mode` controls what version of a resource should be diffed.
    /// `worktree_roots` determine if files can be read from the worktree, where each side of the diff operation can
    /// be represented by its own worktree root. `.gitattributes` are automatically read from the worktree if at least
    /// one worktree is present.
    ///
    /// Note that attributes will always be obtained from the current `HEAD` index even if the resources being diffed
    /// might live in another tree. Further, if one of the `worktree_roots` are set, attributes will also be read from
    /// the worktree. Otherwise, it will be skipped and attributes are read from the index tree instead.
    pub fn diff_resource_cache(
        &self,
        mode: gix_diff::blob::pipeline::Mode,
        worktree_roots: gix_diff::blob::pipeline::WorktreeRoots,
    ) -> Result<gix_diff::blob::Platform, resource_cache::Error> {
        let index = self.index_or_load_from_head()?;
        Ok(crate::diff::resource_cache(
            self,
            mode,
            self.attributes_only(
                &index,
                if worktree_roots.new_root.is_some() || worktree_roots.old_root.is_some() {
                    gix_worktree::stack::state::attributes::Source::WorktreeThenIdMapping
                } else {
                    gix_worktree::stack::state::attributes::Source::IdMapping
                },
            )?
            .inner,
            worktree_roots,
        )?)
    }
}
