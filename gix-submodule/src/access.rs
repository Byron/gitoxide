use crate::config::{Branch, FetchRecurse, Ignore, Update};
use crate::{config, File};
use bstr::BStr;
use std::borrow::Cow;
use std::path::Path;

/// Access
///
/// Note that all methods perform validation of the requested value and report issues right away.
/// If a bypass is needed, use [`config()`](File::config()) for direct access.
impl File {
    /// Return the underlying configuration file.
    ///
    /// Note that it might have been merged with values from another configuration file and may
    /// thus not be accurately reflecting that state of a `.gitmodules` file anymore.
    pub fn config(&self) -> &gix_config::File<'static> {
        &self.config
    }

    /// Return the path at which the `.gitmodules` file lives, if it is known.
    pub fn config_path(&self) -> Option<&Path> {
        self.config.sections().filter_map(|s| s.meta().path.as_deref()).next()
    }

    /// Return the unvalidated names of the submodules for which configuration is present.
    ///
    /// Note that these exact names have to be used for querying submodule values.
    pub fn names(&self) -> impl Iterator<Item = &BStr> {
        self.config
            .sections_by_name("submodule")
            .into_iter()
            .flatten()
            .filter_map(|s| s.header().subsection_name())
    }

    /// Given the `relative_path` (as seen from the root of the worktree) of a submodule with possibly platform-specific
    /// component separators, find the submodule's name associated with this path, or `None` if none was found.
    ///
    /// Note that this does a linear search and compares `relative_path` in a normalized form to the same form of the path
    /// associated with the submodule.
    pub fn name_by_path(&self, relative_path: &BStr) -> Option<&BStr> {
        self.names()
            .filter_map(|n| self.path(n).ok().map(|p| (n, p)))
            .find_map(|(n, p)| (p == relative_path).then_some(n))
    }

    /// Return the path relative to the root directory of the working tree at which the submodule is expected to be checked out.
    /// It's an error if the path doesn't exist as it's the only way to associate a path in the index with additional submodule
    /// information, like the URL to fetch from.
    ///
    /// ### Deviation
    ///
    /// Git currently allows absolute paths to be used when adding submodules, but fails later as it can't find the submodule by
    /// relative path anymore. Let's play it safe here.
    pub fn path(&self, name: &BStr) -> Result<Cow<'_, BStr>, config::path::Error> {
        let path_bstr =
            self.config
                .string("submodule", Some(name), "path")
                .ok_or_else(|| config::path::Error::Missing {
                    submodule: name.to_owned(),
                })?;
        if path_bstr.is_empty() {
            return Err(config::path::Error::Missing {
                submodule: name.to_owned(),
            });
        }
        let path = gix_path::from_bstr(path_bstr.as_ref());
        if path.is_absolute() {
            return Err(config::path::Error::Absolute {
                submodule: name.to_owned(),
                actual: path_bstr.into_owned(),
            });
        }
        if gix_path::normalize(path, "").is_none() {
            return Err(config::path::Error::OutsideOfWorktree {
                submodule: name.to_owned(),
                actual: path_bstr.into_owned(),
            });
        }
        Ok(path_bstr)
    }

    /// Retrieve the `url` field of the submodule named `name`. It's an error if it doesn't exist or is empty.
    pub fn url(&self, name: &BStr) -> Result<gix_url::Url, config::url::Error> {
        let url = self
            .config
            .string("submodule", Some(name), "url")
            .ok_or_else(|| config::url::Error::Missing {
                submodule: name.to_owned(),
            })?;

        if url.is_empty() {
            return Err(config::url::Error::Missing {
                submodule: name.to_owned(),
            });
        }
        gix_url::Url::from_bytes(url.as_ref()).map_err(|err| config::url::Error::Parse {
            submodule: name.to_owned(),
            source: err,
        })
    }

    /// Retrieve the `update` field of the submodule named `name`, if present.
    pub fn update(&self, name: &BStr) -> Result<Option<Update>, config::update::Error> {
        let value: Update = match self.config.string("submodule", Some(name), "update") {
            Some(v) => v.as_ref().try_into().map_err(|()| config::update::Error::Invalid {
                submodule: name.to_owned(),
                actual: v.into_owned(),
            })?,
            None => return Ok(None),
        };

        if let Update::Command(cmd) = &value {
            let ours = self.config.meta();
            let has_value_from_foreign_section = self
                .config
                .sections_by_name("submodule")
                .into_iter()
                .flatten()
                .any(|s| (s.header().subsection_name() == Some(name) && s.meta() as *const _ != ours as *const _));
            if !has_value_from_foreign_section {
                return Err(config::update::Error::CommandForbiddenInModulesConfiguration {
                    submodule: name.to_owned(),
                    actual: cmd.to_owned(),
                });
            }
        }
        Ok(Some(value))
    }

    /// Retrieve the `branch` field of the submodule named `name`, or `None` if unset.
    ///
    /// Note that `Default` is implemented for [`Branch`].
    pub fn branch(&self, name: &BStr) -> Result<Option<Branch>, config::branch::Error> {
        let branch = match self.config.string("submodule", Some(name), "branch") {
            Some(v) => v,
            None => return Ok(None),
        };

        Branch::try_from(branch.as_ref())
            .map(Some)
            .map_err(|err| config::branch::Error {
                submodule: name.to_owned(),
                actual: branch.into_owned(),
                source: err,
            })
    }

    /// Retrieve the `fetchRecurseSubmodules` field of the submodule named `name`, or `None` if unset.
    ///
    /// Note that if it's unset, it should be retrieved from `fetch.recurseSubmodules` in the configuration.
    pub fn fetch_recurse(&self, name: &BStr) -> Result<Option<FetchRecurse>, config::Error> {
        self.config
            .boolean("submodule", Some(name), "fetchRecurseSubmodules")
            .map(FetchRecurse::new)
            .transpose()
            .map_err(|value| config::Error {
                field: "fetchRecurseSubmodules",
                submodule: name.to_owned(),
                actual: value,
            })
    }

    /// Retrieve the `ignore` field of the submodule named `name`, or `None` if unset.
    pub fn ignore(&self, name: &BStr) -> Result<Option<Ignore>, config::Error> {
        self.config
            .string("submodule", Some(name), "ignore")
            .map(|value| {
                Ignore::try_from(value.as_ref()).map_err(|()| config::Error {
                    field: "ignore",
                    submodule: name.to_owned(),
                    actual: value.into_owned(),
                })
            })
            .transpose()
    }

    /// Retrieve the `shallow` field of the submodule named `name`, or `None` if unset.
    ///
    /// If `true`, the submodule will be checked out with `depth = 1`. If unset, `false` is assumed.
    pub fn shallow(&self, name: &BStr) -> Result<Option<bool>, gix_config::value::Error> {
        self.config.boolean("submodule", Some(name), "shallow").transpose()
    }
}
