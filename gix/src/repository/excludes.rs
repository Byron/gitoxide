//! exclude information
use crate::{config, Repository};
impl Repository {
    /// Configure a file-system cache checking if files below the repository are excluded, reading `.gitignore` files from
    /// the specified `source`.
    ///
    /// Note that no worktree is required for this to work, even though access to in-tree `.gitignore` files would require
    /// a non-empty `index` that represents a tree with `.gitignore` files.
    ///
    /// This takes into consideration all the usual repository configuration, namely:
    ///
    /// * `$XDG_CONFIG_HOME/…/ignore` if `core.excludesFile` is *not* set, otherwise use the configured file.
    /// * `$GIT_DIR/info/exclude` if present.
    ///
    /// When only excludes are desired, this is the most efficient way to obtain them. Otherwise use
    /// [`Repository::attributes()`] for accessing both attributes and excludes.
    // TODO: test, provide higher-level custom Cache wrapper that is much easier to use and doesn't panic when accessing entries
    //       by non-relative path.
    pub fn excludes(
        &self,
        index: &gix_index::State,
        overrides: Option<gix_ignore::Search>,
        source: gix_worktree::cache::state::ignore::Source,
    ) -> Result<gix_worktree::Cache, config::exclude_stack::Error> {
        let case = if self.config.ignore_case {
            gix_glob::pattern::Case::Fold
        } else {
            gix_glob::pattern::Case::Sensitive
        };
        let mut buf = Vec::with_capacity(512);
        let ignore = self
            .config
            .assemble_exclude_globals(self.git_dir(), overrides, source, &mut buf)?;
        let state = gix_worktree::cache::State::IgnoreStack(ignore);
        let attribute_list = state.id_mappings_from_index(index, index.path_backing(), source, case);
        Ok(gix_worktree::Cache::new(
            // this is alright as we don't cause mutation of that directory, it's virtual.
            self.work_dir().unwrap_or(self.git_dir()),
            state,
            case,
            buf,
            attribute_list,
        ))
    }
}
