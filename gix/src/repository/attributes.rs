//! exclude information
use crate::Repository;

impl Repository {
    /// Configure a file-system cache for accessing git attributes *and* excludes on a per-path basis.
    ///
    /// Use `attribute_source` to specify where to read attributes from. Also note that exclude information will
    /// always try to read `.gitignore` files from disk before trying to read it from the `index`.
    ///
    /// Note that no worktree is required for this to work, even though access to in-tree `.gitattributes` and `.gitignore` files
    /// would require a non-empty `index` that represents a git tree.
    ///
    /// This takes into consideration all the usual repository configuration, namely:
    ///
    /// * `$XDG_CONFIG_HOME/…/ignore|attributes` if `core.excludesFile|attributesFile` is *not* set, otherwise use the configured file.
    /// * `$GIT_DIR/info/exclude|attributes` if present.
    // TODO: test, provide higher-level custom Cache wrapper that is much easier to use and doesn't panic when accessing entries
    //       by non-relative path.
    pub fn attributes(
        &self,
        index: &gix_index::State,
        attributes_source: gix_worktree::cache::state::attributes::Source,
        ignore_source: gix_worktree::cache::state::ignore::Source,
        exclude_overrides: Option<gix_ignore::Search>,
    ) -> Result<gix_worktree::Cache, crate::attributes::Error> {
        let case = if self.config.ignore_case {
            gix_glob::pattern::Case::Fold
        } else {
            gix_glob::pattern::Case::Sensitive
        };
        let (attributes, mut buf) = self.config.assemble_attribute_globals(
            self.git_dir(),
            attributes_source,
            self.options.permissions.attributes,
        )?;
        let ignore =
            self.config
                .assemble_exclude_globals(self.git_dir(), exclude_overrides, ignore_source, &mut buf)?;
        let state = gix_worktree::cache::State::AttributesAndIgnoreStack { attributes, ignore };
        let attribute_list = state.id_mappings_from_index(index, index.path_backing(), ignore_source, case);
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
