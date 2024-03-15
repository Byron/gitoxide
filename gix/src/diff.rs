pub use gix_diff::*;

///
#[allow(clippy::empty_docs)]
pub mod rename {
    /// Determine how to do rename tracking.
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub enum Tracking {
        /// Do not track renames at all, the fastest option.
        Disabled,
        /// Track renames.
        Renames,
        /// Track renames and copies.
        ///
        /// This is the most expensive option.
        RenamesAndCopies,
    }
}

///
#[cfg(feature = "blob-diff")]
mod utils {
    use gix_diff::{rewrites::Copies, Rewrites};

    use crate::{
        config::{cache::util::ApplyLeniency, tree::Diff},
        diff::rename::Tracking,
        Repository,
    };

    ///
    #[allow(clippy::empty_docs)]
    pub mod new_rewrites {
        /// The error returned by [`new_rewrites()`](super::new_rewrites()).
        #[derive(Debug, thiserror::Error)]
        #[allow(missing_docs)]
        pub enum Error {
            #[error(transparent)]
            ConfigDiffRenames(#[from] crate::config::key::GenericError),
            #[error(transparent)]
            ConfigDiffRenameLimit(#[from] crate::config::unsigned_integer::Error),
        }
    }

    ///
    #[allow(clippy::empty_docs)]
    pub mod resource_cache {
        /// The error returned by [`resource_cache()`](super::resource_cache()).
        #[derive(Debug, thiserror::Error)]
        #[allow(missing_docs)]
        pub enum Error {
            #[error(transparent)]
            DiffAlgorithm(#[from] crate::config::diff::algorithm::Error),
            #[error(transparent)]
            WorktreeFilterOptions(#[from] crate::filter::pipeline::options::Error),
            #[error(transparent)]
            DiffDrivers(#[from] crate::config::diff::drivers::Error),
            #[error(transparent)]
            DiffPipelineOptions(#[from] crate::config::diff::pipeline_options::Error),
            #[error(transparent)]
            CommandContext(#[from] crate::config::command_context::Error),
        }
    }

    /// Create an instance by reading all relevant information from the `config`uration, while being `lenient` or not.
    /// Returns `Ok(None)` if nothing is configured.
    ///
    /// Note that missing values will be defaulted similar to what git does.
    #[allow(clippy::result_large_err)]
    pub fn new_rewrites(
        config: &gix_config::File<'static>,
        lenient: bool,
    ) -> Result<Option<Rewrites>, new_rewrites::Error> {
        let key = "diff.renames";
        let copies = match config
            .boolean_by_key(key)
            .map(|value| Diff::RENAMES.try_into_renames(value))
            .transpose()
            .with_leniency(lenient)?
        {
            Some(renames) => match renames {
                Tracking::Disabled => return Ok(None),
                Tracking::Renames => None,
                Tracking::RenamesAndCopies => Some(Copies::default()),
            },
            None => return Ok(None),
        };

        let default = Rewrites::default();
        Ok(Rewrites {
            copies,
            limit: config
                .integer_by_key("diff.renameLimit")
                .map(|value| Diff::RENAME_LIMIT.try_into_usize(value))
                .transpose()
                .with_leniency(lenient)?
                .unwrap_or(default.limit),
            ..default
        }
        .into())
    }

    /// Return a low-level utility to efficiently prepare a blob-level diff operation between two resources,
    /// and cache these diffable versions so that matrix-like MxN diffs are efficient.
    ///
    /// `repo` is used to obtain the needed configuration values.
    /// `mode` determines how the diffable files will look like, and also how fast, in average, these conversions are.
    /// `attr_stack` is for accessing `.gitattributes` for knowing how to apply filters. Know that it's typically adjusted based on the
    /// `roots` - if there are no worktree roots, `.gitattributes` are also not usually read from worktrees.
    /// `roots` provide information about where to get diffable data from, so source and destination can either be sourced from
    /// a worktree, or from the object database, or both.
    pub fn resource_cache(
        repo: &Repository,
        mode: gix_diff::blob::pipeline::Mode,
        attr_stack: gix_worktree::Stack,
        roots: gix_diff::blob::pipeline::WorktreeRoots,
    ) -> Result<gix_diff::blob::Platform, resource_cache::Error> {
        let diff_algo = repo.config.diff_algorithm()?;
        let diff_cache = gix_diff::blob::Platform::new(
            gix_diff::blob::platform::Options {
                algorithm: Some(diff_algo),
                skip_internal_diff_if_external_is_configured: false,
            },
            gix_diff::blob::Pipeline::new(
                roots,
                gix_filter::Pipeline::new(repo.command_context()?, crate::filter::Pipeline::options(repo)?),
                repo.config.diff_drivers()?,
                repo.config.diff_pipeline_options()?,
            ),
            mode,
            attr_stack,
        );
        Ok(diff_cache)
    }
}
#[cfg(feature = "blob-diff")]
pub use utils::{new_rewrites, resource_cache};
