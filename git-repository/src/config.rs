use crate::{bstr::BString, permission};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not open repository conifguration file")]
    Open(#[from] git_config::parser::ParserOrIoError<'static>),
    #[error("Cannot handle objects formatted as {:?}", .name)]
    UnsupportedObjectFormat { name: crate::bstr::BString },
    #[error("The value for '{}' cannot be empty", .key)]
    EmptyValue { key: &'static str },
    #[error("Invalid value for 'core.abbrev' = '{}'. It must be between 4 and {}", .value, .max)]
    CoreAbbrev { value: BString, max: u8 },
    #[error("Value '{}' at key '{}' could not be decoded as boolean", .value, .key)]
    DecodeBoolean { key: String, value: BString },
    #[error(transparent)]
    PathInterpolation(#[from] git_config::values::path::interpolate::Error),
}

/// Utility type to keep pre-obtained configuration values.
#[derive(Debug, Clone)]
pub(crate) struct Cache {
    pub resolved: crate::Config,
    /// The hex-length to assume when shortening object ids. If `None`, it should be computed based on the approximate object count.
    pub hex_len: Option<usize>,
    /// true if the repository is designated as 'bare', without work tree.
    pub is_bare: bool,
    /// The type of hash to use.
    pub object_hash: git_hash::Kind,
    /// If true, multi-pack indices, whether present or not, may be used by the object database.
    pub use_multi_pack_index: bool,
    /// If true, we are on a case-insensitive file system.
    #[cfg_attr(not(feature = "git-index"), allow(dead_code))]
    pub ignore_case: bool,
    /// The path to the user-level excludes file to ignore certain files in the worktree.
    #[cfg_attr(not(feature = "git-index"), allow(dead_code))]
    pub excludes_file: Option<std::path::PathBuf>,
    /// Define how we can use values obtained with `xdg_config(…)` and its `XDG_CONFIG_HOME` variable.
    #[cfg_attr(not(feature = "git-index"), allow(dead_code))]
    xdg_config_home_env: permission::env_var::Resource,
    /// Define how we can use values obtained with `xdg_config(…)`. and its `HOME` variable.
    #[cfg_attr(not(feature = "git-index"), allow(dead_code))]
    home_env: permission::env_var::Resource,
    // TODO: make core.precomposeUnicode available as well.
}

mod cache {
    use std::{convert::TryFrom, path::PathBuf};

    use git_config::{
        values::{Boolean, Integer},
        File,
    };

    use super::{Cache, Error};
    use crate::{bstr::ByteSlice, permission};

    impl Cache {
        pub fn new(
            git_dir: &std::path::Path,
            xdg_config_home_env: permission::env_var::Resource,
            home_env: permission::env_var::Resource,
            git_install_dir: Option<&std::path::Path>,
        ) -> Result<Self, Error> {
            let home = std::env::var_os("HOME")
                .map(PathBuf::from)
                .and_then(|home| home_env.check(home).ok().flatten());
            // TODO: don't forget to use the canonicalized home for initializing the stacked config.
            //       like git here: https://github.com/git/git/blob/master/config.c#L208:L208
            let config = File::at(git_dir.join("config"))?;

            let is_bare = config_bool(&config, "core.bare", false)?;
            let use_multi_pack_index = config_bool(&config, "core.multiPackIndex", true)?;
            let ignore_case = config_bool(&config, "core.ignorecase", false)?;
            let excludes_file = config
                .path("core", None, "excludesFile")
                .map(|p| p.interpolate(git_install_dir, home.as_deref()).map(|p| p.into_owned()))
                .transpose()?;
            let repo_format_version = config
                .value::<Integer>("core", None, "repositoryFormatVersion")
                .map_or(0, |v| v.value);
            let object_hash = (repo_format_version != 1)
                .then(|| Ok(git_hash::Kind::Sha1))
                .or_else(|| {
                    config.string("extensions", None, "objectFormat").map(|format| {
                        if format.as_ref() == "sha1" {
                            Ok(git_hash::Kind::Sha1)
                        } else {
                            Err(Error::UnsupportedObjectFormat {
                                name: format.to_vec().into(),
                            })
                        }
                    })
                })
                .transpose()?
                .unwrap_or(git_hash::Kind::Sha1);

            let mut hex_len = None;
            if let Some(hex_len_str) = config.string("core", None, "abbrev") {
                if hex_len_str.trim().is_empty() {
                    return Err(Error::EmptyValue { key: "core.abbrev" });
                }
                if hex_len_str.as_ref() != "auto" {
                    let value_bytes = hex_len_str.as_ref();
                    if let Ok(Boolean::False(_)) = Boolean::try_from(value_bytes) {
                        hex_len = object_hash.len_in_hex().into();
                    } else {
                        let value = Integer::try_from(value_bytes)
                            .map_err(|_| Error::CoreAbbrev {
                                value: hex_len_str.clone().into_owned(),
                                max: object_hash.len_in_hex() as u8,
                            })?
                            .to_decimal()
                            .ok_or_else(|| Error::CoreAbbrev {
                                value: hex_len_str.clone().into_owned(),
                                max: object_hash.len_in_hex() as u8,
                            })?;
                        if value < 4 || value as usize > object_hash.len_in_hex() {
                            return Err(Error::CoreAbbrev {
                                value: hex_len_str.clone().into_owned(),
                                max: object_hash.len_in_hex() as u8,
                            });
                        }
                        hex_len = Some(value as usize);
                    }
                }
            }

            Ok(Cache {
                resolved: config.into(),
                use_multi_pack_index,
                object_hash,
                is_bare,
                ignore_case,
                hex_len,
                excludes_file,
                xdg_config_home_env,
                home_env,
            })
        }

        /// Return a path by using the `$XDF_CONFIG_HOME` or `$HOME/.config/…` environment variables locations.
        #[cfg_attr(not(feature = "git-index"), allow(dead_code))]
        pub fn xdg_config_path(
            &self,
            resource_file_name: &str,
        ) -> Result<Option<PathBuf>, git_sec::permission::Error<PathBuf, git_sec::Permission>> {
            std::env::var_os("XDG_CONFIG_HOME")
                .map(|path| (path, &self.xdg_config_home_env))
                .or_else(|| std::env::var_os("HOME").map(|path| (path, &self.home_env)))
                .and_then(|(base, permission)| {
                    let resource = std::path::PathBuf::from(base).join("git").join(resource_file_name);
                    permission.check(resource).transpose()
                })
                .transpose()
        }

        /// Return the home directory if we are allowed to read it and if it is set in the environment.
        ///
        /// We never fail for here even if the permission is set to deny as we `git-config` will fail later
        /// if it actually wants to use the home directory - we don't want to fail prematurely.
        #[cfg(feature = "git-mailmap")]
        pub fn home_dir(&self) -> Option<PathBuf> {
            std::env::var_os("HOME")
                .map(PathBuf::from)
                .and_then(|path| self.home_env.check(path).ok().flatten())
        }
    }

    fn config_bool(config: &File<'_>, key: &str, default: bool) -> Result<bool, Error> {
        let (section, key) = key.split_once('.').expect("valid section.key format");
        config
            .boolean(section, None, key)
            .unwrap_or(Ok(default))
            .map_err(|err| Error::DecodeBoolean {
                value: err.input,
                key: key.into(),
            })
    }
}
