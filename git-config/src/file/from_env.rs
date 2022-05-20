use crate::file::{from_paths, resolve_includes};
use crate::values::path::interpolate;
use crate::File;
use std::borrow::Cow;
use std::path::PathBuf;

/// Represents the errors that may occur when calling [`File::from_env`][crate::File::from_env()].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("GIT_CONFIG_COUNT was not a positive integer: {}", .input)]
    ParseError { input: String },
    #[error("GIT_CONFIG_KEY_{} was not set", .key_id)]
    InvalidKeyId { key_id: usize },
    #[error("GIT_CONFIG_KEY_{} was set to an invalid value: {}", .key_id, .key_val)]
    InvalidKeyValue { key_id: usize, key_val: String },
    #[error("GIT_CONFIG_VALUE_{} was not set", .value_id)]
    InvalidValueId { value_id: usize },
    #[error(transparent)]
    PathInterpolationError(#[from] interpolate::Error),
    #[error(transparent)]
    FromPathsError(#[from] from_paths::Error),
}

impl<'a> File<'a> {
    /// Constructs a `git-config` from the default cascading sequence.
    /// This is neither zero-alloc nor zero-copy.
    ///
    /// See <https://git-scm.com/docs/git-config#FILES> for details.
    pub fn from_env_paths(options: from_paths::Options<'_>) -> Result<File<'static>, from_paths::Error> {
        use std::env;

        let mut paths = vec![];

        if env::var("GIT_CONFIG_NO_SYSTEM").is_err() {
            if let Some(git_config_system) = env::var_os("GIT_CONFIG_SYSTEM") {
                paths.push(PathBuf::from(git_config_system))
            } else {
                // In git the fallback is set to a build time macro which defaults to /etc/gitconfig
                paths.push(PathBuf::from("/etc/gitconfig"));
            }
        }

        if let Some(git_config_global) = env::var_os("GIT_CONFIG_GLOBAL") {
            paths.push(PathBuf::from(git_config_global));
        } else {
            // Divergence from git-config(1)
            // These two are supposed to share the same scope and override
            // rather than append according to git-config(1) documentation.
            if let Some(xdg_config_home) = env::var_os("XDG_CONFIG_HOME") {
                paths.push(PathBuf::from(xdg_config_home).join("git/config"));
            } else if let Some(home) = env::var_os("HOME") {
                paths.push(PathBuf::from(home).join(".config/git/config"));
            }

            if let Some(home) = env::var_os("HOME") {
                paths.push(PathBuf::from(home).join(".gitconfig"));
            }
        }

        if let Some(git_dir) = env::var_os("GIT_DIR") {
            paths.push(PathBuf::from(git_dir).join("config"));
        }

        File::from_paths(paths, options)
    }

    /// Generates a config from the environment variables. This is neither
    /// zero-copy nor zero-alloc. See [`git-config`'s documentation] on
    /// environment variable for more information.
    ///
    /// # Errors
    ///
    /// Returns an error if `GIT_CONFIG_COUNT` set and is not a number, or if
    /// there was an invalid key value pair.
    ///
    /// [`git-config`'s documentation]: https://git-scm.com/docs/git-config#Documentation/git-config.txt-GITCONFIGCOUNT
    pub fn from_env(options: from_paths::Options<'_>) -> Result<Option<File<'_>>, Error> {
        use std::env;
        let count: usize = match env::var("GIT_CONFIG_COUNT") {
            Ok(v) => v.parse().map_err(|_| Error::ParseError { input: v })?,
            Err(_) => return Ok(None),
        };

        let mut config = File::new();
        for i in 0..count {
            let key = env::var(format!("GIT_CONFIG_KEY_{}", i)).map_err(|_| Error::InvalidKeyId { key_id: i })?;
            let value = env::var_os(format!("GIT_CONFIG_VALUE_{}", i)).ok_or(Error::InvalidValueId { value_id: i })?;
            if let Some((section_name, maybe_subsection)) = key.split_once('.') {
                let (subsection, key) = if let Some((subsection, key)) = maybe_subsection.rsplit_once('.') {
                    (Some(subsection), key)
                } else {
                    (None, maybe_subsection)
                };

                let mut section = if let Ok(section) = config.section_mut(section_name, subsection) {
                    section
                } else {
                    // Need to have config own the section and subsection names
                    // else they get dropped at the end of the loop.
                    config.new_section(
                        section_name.to_string(),
                        subsection.map(|subsection| Cow::Owned(subsection.to_string())),
                    )
                };

                section.push(
                    Cow::<str>::Owned(key.to_string()).into(),
                    Cow::Owned(git_path::into_bstr(PathBuf::from(value)).into_owned().into()),
                );
            } else {
                return Err(Error::InvalidKeyValue {
                    key_id: i,
                    key_val: key.to_string(),
                });
            }
        }

        // This occurs when `GIT_CONFIG_COUNT` is set to zero.
        if config.is_empty() {
            Ok(None)
        } else {
            resolve_includes(&mut config, None, options)?;
            Ok(Some(config))
        }
    }
}
