use crate::config::cache::util::ApplyLeniencyDefault;
use crate::config::tree;
use crate::repository::merge_resource_cache;
use crate::Repository;

/// Merge-utilities
impl Repository {
    /// Create a resource cache that can hold the three resources needed for a three-way merge. `worktree_roots`
    /// determines which side of the merge is read from the worktree, or from which worktree.
    ///
    /// The platform can be used to setup resources and finally perform a merge.
    ///
    /// Note that the current index is used for attribute queries.
    pub fn merge_resource_cache(
        &self,
        worktree_roots: gix_merge::blob::pipeline::WorktreeRoots,
    ) -> Result<gix_merge::blob::Platform, merge_resource_cache::Error> {
        let index = self.index_or_load_from_head()?;
        let mode = {
            let renormalize = self
                .config
                .resolved
                .boolean(&tree::Merge::RENORMALIZE)
                .map(|res| {
                    tree::Merge::RENORMALIZE
                        .enrich_error(res)
                        .with_lenient_default(self.config.lenient_config)
                })
                .transpose()?
                .unwrap_or_default();
            if renormalize {
                gix_merge::blob::pipeline::Mode::Renormalize
            } else {
                gix_merge::blob::pipeline::Mode::ToGit
            }
        };
        let attrs = self
            .attributes_only(
                &index,
                if worktree_roots.is_unset() {
                    gix_worktree::stack::state::attributes::Source::IdMapping
                } else {
                    gix_worktree::stack::state::attributes::Source::WorktreeThenIdMapping
                },
            )?
            .inner;
        let filter = gix_filter::Pipeline::new(self.command_context()?, crate::filter::Pipeline::options(self)?);
        let filter = gix_merge::blob::Pipeline::new(worktree_roots, filter, self.config.merge_pipeline_options()?);
        let options = gix_merge::blob::platform::Options {
            default_driver: self
                .config
                .resolved
                .string(&tree::Merge::DEFAULT)
                .map(|name| name.into_owned()),
        };
        let drivers = self.config.merge_drivers()?;
        Ok(gix_merge::blob::Platform::new(filter, mode, attrs, drivers, options))
    }
}
