use crate::bstr::BStr;
use crate::{Pathspec, Repository};

impl Repository {
    /// Create a new pathspec abstraction that allows to conduct searches using `patterns`. `index` may be needed to load attributes
    /// which is required only if `patterns` refer to attributes via `:(attr:…)` syntax.
    /// `inherit_ignore_case` should be `true` if `patterns` will match against files on disk, or `false` otherwise.
    ///
    /// It will be initialized exactly how it would, and attribute matching will be conducted by reading the worktree first if available.
    /// If that is not desirable, consider calling [`Pathspec::new()`] directly.
    pub fn pathspec(
        &self,
        patterns: impl IntoIterator<Item = impl AsRef<BStr>>,
        inherit_ignore_case: bool,
        index: &gix_index::State,
    ) -> Result<Pathspec<'_>, crate::pathspec::init::Error> {
        Pathspec::new(self, patterns, inherit_ignore_case, || {
            self.attributes_only(
                index,
                gix_worktree::cache::state::attributes::Source::WorktreeThenIdMapping,
            )
            .map_err(Into::into)
        })
    }

    /// Return default settings that are required when [parsing pathspecs](gix_pathspec::parse()) by hand.
    ///
    /// These are stemming from environment variables which have been converted to [config settings](crate::config::tree::gitoxide::Pathspec),
    /// which now serve as authority for configuration.
    pub fn pathspec_defaults(&self) -> Result<gix_pathspec::Defaults, gix_pathspec::defaults::from_environment::Error> {
        self.config.pathspec_defaults()
    }
}
