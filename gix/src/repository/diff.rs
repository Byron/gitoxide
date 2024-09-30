use crate::repository::diff_resource_cache;
use crate::Repository;

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
    ) -> Result<gix_diff::blob::Platform, diff_resource_cache::Error> {
        let index = self.index_or_load_from_head()?;
        Ok(crate::diff::resource_cache(
            self,
            mode,
            self.attributes_only(
                &index,
                if worktree_roots.is_unset() {
                    gix_worktree::stack::state::attributes::Source::IdMapping
                } else {
                    gix_worktree::stack::state::attributes::Source::WorktreeThenIdMapping
                },
            )?
            .inner,
            worktree_roots,
        )?)
    }

    /// Return a resource cache suitable for diffing blobs from trees directly, where no worktree checkout exists.
    ///
    /// For more control, see [`diff_resource_cache()`](Self::diff_resource_cache).
    pub fn diff_resource_cache_for_tree_diff(&self) -> Result<gix_diff::blob::Platform, diff_resource_cache::Error> {
        self.diff_resource_cache(
            gix_diff::blob::pipeline::Mode::ToGit,
            gix_diff::blob::pipeline::WorktreeRoots::default(),
        )
    }
}
