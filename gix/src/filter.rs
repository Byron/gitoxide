//! lower-level access to filters which are applied to create working tree checkouts or to 'clean' working tree contents for storage in git.
use std::borrow::Cow;

pub use gix_filter as plumbing;
use gix_odb::{Find, FindExt};

use crate::{
    bstr::BStr,
    config::{
        cache::util::{ApplyLeniency, ApplyLeniencyDefaultValue},
        tree::Core,
    },
    Repository,
};

///
pub mod pipeline {
    ///
    pub mod options {
        use crate::{bstr::BString, config};

        /// The error returned by [Pipeline::options()][crate::filter::Pipeline::options()].
        #[derive(Debug, thiserror::Error)]
        #[allow(missing_docs)]
        pub enum Error {
            #[error(transparent)]
            CheckRoundTripEncodings(#[from] config::encoding::Error),
            #[error(transparent)]
            SafeCrlf(#[from] config::key::GenericErrorWithValue),
            #[error("Could not interpret 'filter.{name}.required' configuration")]
            Driver {
                name: BString,
                source: gix_config::value::Error,
            },
        }
    }

    ///
    pub mod convert_to_git {
        /// The error returned by [Pipeline::convert_to_git()][crate::filter::Pipeline::convert_to_git()].
        #[derive(Debug, thiserror::Error)]
        #[allow(missing_docs)]
        pub enum Error {
            #[error("Failed to prime attributes to the path at which the data resides")]
            WorktreeCacheAtPath(#[from] std::io::Error),
            #[error(transparent)]
            Convert(#[from] gix_filter::pipeline::convert::to_git::Error),
        }
    }

    ///
    pub mod convert_to_worktree {
        /// The error returned by [Pipeline::convert_to_worktree()][crate::filter::Pipeline::convert_to_worktree()].
        #[derive(Debug, thiserror::Error)]
        #[allow(missing_docs)]
        pub enum Error {
            #[error("Failed to prime attributes to the path at which the data resides")]
            WorktreeCacheAtPath(#[from] std::io::Error),
            #[error(transparent)]
            Convert(#[from] gix_filter::pipeline::convert::to_worktree::Error),
        }
    }
}

/// A git pipeline for transforming data *to-git* and *to-worktree*, based
/// [on git configuration and attributes](https://git-scm.com/docs/gitattributes).
#[derive(Clone)]
pub struct Pipeline<'repo> {
    inner: gix_filter::Pipeline,
    cache: gix_worktree::Stack,
    repo: &'repo Repository,
}

/// Lifecycle
impl<'repo> Pipeline<'repo> {
    /// Extract options from `repo` that are needed to properly drive a standard git filter pipeline.
    pub fn options(repo: &'repo Repository) -> Result<gix_filter::pipeline::Options, pipeline::options::Error> {
        let config = &repo.config.resolved;
        let encodings =
            Core::CHECK_ROUND_TRIP_ENCODING.try_into_encodings(config.string_by_key("core.checkRoundtripEncoding"))?;
        let safe_crlf = config
            .string_by_key("core.safecrlf")
            .map(|value| Core::SAFE_CRLF.try_into_safecrlf(value))
            .transpose()
            .map(Option::unwrap_or_default)
            .with_lenient_default_value(
                repo.config.lenient_config,
                // in lenient mode, we prefer the safe option, instead of just (trying) to output warnings.
                gix_filter::pipeline::CrlfRoundTripCheck::Fail,
            )?;
        let auto_crlf = config
            .string_by_key("core.autocrlf")
            .map(|value| Core::AUTO_CRLF.try_into_autocrlf(value))
            .transpose()
            .with_leniency(repo.config.lenient_config)?
            .unwrap_or_default();
        let eol = config
            .string_by_key("core.eol")
            .map(|value| Core::EOL.try_into_eol(value))
            .transpose()?;
        let drivers = extract_drivers(repo)?;
        Ok(gix_filter::pipeline::Options {
            drivers,
            eol_config: gix_filter::eol::Configuration { auto_crlf, eol },
            encodings_with_roundtrip_check: encodings,
            crlf_roundtrip_check: safe_crlf,
            object_hash: repo.object_hash(),
        })
    }

    /// Create a new instance by extracting all necessary information and configuration from a `repo` along with `cache` for accessing
    /// attributes. The `index` is used for some filters which may access it under very specific circumstances.
    pub fn new(repo: &'repo Repository, cache: gix_worktree::Stack) -> Result<Self, pipeline::options::Error> {
        let pipeline = gix_filter::Pipeline::new(cache.attributes_collection(), Self::options(repo)?);
        Ok(Pipeline {
            inner: pipeline,
            cache,
            repo,
        })
    }

    /// Detach the repository and obtain the individual functional parts.
    pub fn into_parts(self) -> (gix_filter::Pipeline, gix_worktree::Stack) {
        (self.inner, self.cache)
    }
}

/// Conversions
impl<'repo> Pipeline<'repo> {
    /// Convert a `src` stream (to be found at `rela_path`, a repo-relative path) to a representation suitable for storage in `git`
    /// by using all attributes at `rela_path` and configuration of the repository to know exactly which filters apply.
    /// `index` is used in particularly rare cases where the CRLF filter in auto-mode tries to determine whether or not to apply itself,
    /// and it should match the state used when [instantiating this instance][Self::new()].
    /// Note that the return-type implements [`std::io::Read`].
    pub fn convert_to_git<R>(
        &mut self,
        src: R,
        rela_path: &std::path::Path,
        index: &gix_index::State,
    ) -> Result<gix_filter::pipeline::convert::ToGitOutcome<'_, R>, pipeline::convert_to_git::Error>
    where
        R: std::io::Read,
    {
        let entry = self
            .cache
            .at_path(rela_path, Some(false), |id, buf| self.repo.objects.find_blob(id, buf))?;
        Ok(self.inner.convert_to_git(
            src,
            rela_path,
            &mut |_, attrs| {
                entry.matching_attributes(attrs);
            },
            &mut |buf| -> Result<_, gix_odb::find::Error> {
                let entry = match index.entry_by_path(gix_path::into_bstr(rela_path).as_ref()) {
                    None => return Ok(None),
                    Some(entry) => entry,
                };
                let obj = self.repo.objects.try_find(&entry.id, buf)?;
                Ok(obj.filter(|obj| obj.kind == gix_object::Kind::Blob).map(|_| ()))
            },
        )?)
    }

    /// Convert a `src` buffer located at `rela_path` (in the index) from what's in `git` to the worktree representation.
    /// This method will obtain all attributes and configuration necessary to know exactly which filters to apply.
    /// Note that the return-type implements [`std::io::Read`].
    ///
    /// Use `can_delay` to tell driver processes that they may delay the return of data. Doing this will require the caller to specifically
    /// handle delayed files by keeping state and using [`Self::into_parts()`] to get access to the driver state to follow the delayed-files
    /// protocol. For simplicity, most will want to disallow delayed processing.
    pub fn convert_to_worktree<'input>(
        &mut self,
        src: &'input [u8],
        rela_path: &BStr,
        can_delay: gix_filter::driver::apply::Delay,
    ) -> Result<gix_filter::pipeline::convert::ToWorktreeOutcome<'input, '_>, pipeline::convert_to_worktree::Error>
    {
        let entry = self
            .cache
            .at_entry(rela_path, Some(false), |id, buf| self.repo.objects.find_blob(id, buf))?;
        Ok(self.inner.convert_to_worktree(
            src,
            rela_path,
            &mut |_, attrs| {
                entry.matching_attributes(attrs);
            },
            can_delay,
        )?)
    }

    /// Retrieve the static context that is made available to the process filters.
    ///
    /// The context set here is relevant for the [`convert_to_git()`][Self::convert_to_git()] and
    /// [`convert_to_worktree()`][Self::convert_to_worktree()] methods.
    pub fn driver_context_mut(&mut self) -> &mut gix_filter::pipeline::Context {
        self.inner.driver_context_mut()
    }
}

/// Obtain a list of all configured driver, but ignore those in sections that we don't trust enough.
fn extract_drivers(repo: &Repository) -> Result<Vec<gix_filter::Driver>, pipeline::options::Error> {
    repo.config
        .resolved
        .sections_by_name("filter")
        .into_iter()
        .flatten()
        .filter(|s| repo.filter_config_section()(s.meta()))
        .filter_map(|s| {
            s.header().subsection_name().map(|name| {
                Ok(gix_filter::Driver {
                    name: name.to_owned(),
                    clean: s.value("clean").map(Cow::into_owned),
                    smudge: s.value("smudge").map(Cow::into_owned),
                    process: s.value("process").map(Cow::into_owned),
                    required: s
                        .value("required")
                        .map(|value| gix_config::Boolean::try_from(value.as_ref()))
                        .transpose()
                        .map_err(|err| pipeline::options::Error::Driver {
                            name: name.to_owned(),
                            source: err,
                        })?
                        .unwrap_or_default()
                        .into(),
                })
            })
        })
        .collect::<Result<Vec<_>, pipeline::options::Error>>()
}
