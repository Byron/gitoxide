use crate::status::{index_worktree, Platform, Submodule};

/// Builder
impl<'repo, Progress> Platform<'repo, Progress>
where
    Progress: gix_features::progress::Progress,
{
    /// Call `cb` on dirwalk options if these are set (which is the default). The directory walk is used to find
    /// untracked files or ignored files.
    /// `cb` will be able to run builder-methods on the passed dirwalk options.
    pub fn dirwalk_options(mut self, cb: impl FnOnce(crate::dirwalk::Options) -> crate::dirwalk::Options) -> Self {
        if let Some(opts) = self.index_worktree_options.dirwalk_options.take() {
            self.index_worktree_options.dirwalk_options = Some(cb(opts));
        }
        self
    }

    /// Configure how the `submodule_status` is obtained when looking at submodules that are still mentioned in the index.
    // If `None` is given, no submodule status check is performed.
    pub fn index_worktree_submodules(mut self, submodules: impl Into<Option<Submodule>>) -> Self {
        let submodules = submodules.into();
        self.submodules = match submodules {
            None => Submodule::Given {
                ignore: crate::submodule::config::Ignore::All,
                check_dirty: false,
            },
            Some(status) => status,
        };
        self
    }

    /// Set the `index` to use when making comparisons to the worktree and the head revision.
    ///
    /// Defaults to the current index, or an empty one if it doesn't exist (yet).
    pub fn index(mut self, index: crate::worktree::IndexPersistedOrInMemory) -> Self {
        self.index = Some(index);
        self
    }

    /// Configure the index-to-worktree rename tracking with `rewrites`, which is `None` by default.
    ///
    /// Note that Git does not have configuration related to rename tracking of changes between the index
    /// and the worktree. The closest there is can be obtained using [`crate::diff::new_rewrites()`], which refers
    /// to rename tracking between trees.
    ///
    /// Also note that if `rewrites` are `Some()`, [`sorting`](index_worktree::Options::sorting) will automatically be
    /// configured to assure deterministic outcomes for rewrite solutions.
    pub fn index_worktree_rewrites(mut self, rewrites: impl Into<Option<gix_diff::Rewrites>>) -> Self {
        let rewrites = rewrites.into();
        self.index_worktree_options.rewrites = rewrites;
        if rewrites.is_some() && self.index_worktree_options.sorting.is_none() {
            self.index_worktree_options.sorting =
                Some(gix_status::index_as_worktree_with_renames::Sorting::ByPathCaseSensitive);
        }
        self
    }

    /// Adjust all options related to the index-worktree status.
    /// This is a catch-all in case there are no more specific methods that could be used instead to change
    /// the respective option.
    pub fn index_worktree_options_mut(mut self, cb: impl FnOnce(&mut index_worktree::Options)) -> Self {
        cb(&mut self.index_worktree_options);
        self
    }
}
