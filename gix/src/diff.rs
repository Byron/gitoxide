pub use gix_diff::*;

///
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
    use crate::config::cache::util::ApplyLeniency;
    use crate::config::tree::Diff;
    use crate::diff::rename::Tracking;
    use gix_diff::rewrites::Copies;
    use gix_diff::Rewrites;

    ///
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
}
#[cfg(feature = "blob-diff")]
pub use utils::new_rewrites;
