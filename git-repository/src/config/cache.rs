use std::{convert::TryFrom, path::PathBuf};

use git_config::{Boolean, Integer};

use super::{Cache, Error};
use crate::rev_spec::parse::ObjectKindHint;
use crate::{bstr::ByteSlice, repository, repository::identity};

/// A utility to deal with the cyclic dependency between the ref store and the configuration. The ref-store needs the
/// object hash kind, and the configuration needs the current branch name to resolve conditional includes with `onbranch`.
#[allow(dead_code)]
pub(crate) struct StageOne {
    git_dir_config: git_config::File<'static>,
    buf: Vec<u8>,

    is_bare: bool,
    lossy: Option<bool>,
    pub object_hash: git_hash::Kind,
    pub reflog: Option<git_ref::store::WriteReflog>,
}

/// Initialization
impl StageOne {
    pub fn new(git_dir: &std::path::Path, git_dir_trust: git_sec::Trust, lossy: Option<bool>) -> Result<Self, Error> {
        let mut buf = Vec::with_capacity(512);
        let config = {
            let config_path = git_dir.join("config");
            std::io::copy(&mut std::fs::File::open(&config_path)?, &mut buf)?;

            git_config::File::from_bytes_owned(
                &mut buf,
                git_config::file::Metadata::from(git_config::Source::Local)
                    .at(config_path)
                    .with(git_dir_trust),
                git_config::file::init::Options {
                    includes: git_config::file::includes::Options::no_follow(),
                    ..base_options(lossy)
                },
            )?
        };

        let is_bare = config_bool(&config, "core.bare", false)?;
        let repo_format_version = config
            .value::<Integer>("core", None, "repositoryFormatVersion")
            .map_or(0, |v| v.to_decimal().unwrap_or_default());
        let object_hash = (repo_format_version != 1)
            .then(|| Ok(git_hash::Kind::Sha1))
            .or_else(|| {
                config.string("extensions", None, "objectFormat").map(|format| {
                    if format.as_ref().eq_ignore_ascii_case(b"sha1") {
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

        let reflog = query_refupdates(&config);
        Ok(StageOne {
            git_dir_config: config,
            buf,
            is_bare,
            lossy,
            object_hash,
            reflog,
        })
    }
}

/// Initialization
impl Cache {
    #[allow(clippy::too_many_arguments)]
    pub fn from_stage_one(
        StageOne {
            git_dir_config,
            mut buf,
            lossy,
            is_bare,
            object_hash,
            reflog: _,
        }: StageOne,
        git_dir: &std::path::Path,
        branch_name: Option<&git_ref::FullNameRef>,
        mut filter_config_section: fn(&git_config::file::Metadata) -> bool,
        git_install_dir: Option<&std::path::Path>,
        home: Option<&std::path::Path>,
        repository::permissions::Environment {
            git_prefix,
            home: home_env,
            xdg_config_home: xdg_config_home_env,
        }: repository::permissions::Environment,
        repository::permissions::Config {
            system: use_system,
            git: use_git,
            user: use_user,
            env: use_env,
            includes: use_includes,
        }: repository::permissions::Config,
    ) -> Result<Self, Error> {
        let options = git_config::file::init::Options {
            includes: if use_includes {
                git_config::file::includes::Options::follow(
                    interpolate_context(git_install_dir, home),
                    git_config::file::includes::conditional::Context {
                        git_dir: git_dir.into(),
                        branch_name,
                    },
                )
            } else {
                git_config::file::includes::Options::no_follow()
            },
            ..base_options(lossy)
        };

        let config = {
            let home_env = &home_env;
            let xdg_config_home_env = &xdg_config_home_env;
            let git_prefix = &git_prefix;
            let metas = [git_config::source::Kind::System, git_config::source::Kind::Global]
                .iter()
                .flat_map(|kind| kind.sources())
                .filter_map(|source| {
                    match source {
                        git_config::Source::System if !use_system => return None,
                        git_config::Source::Git if !use_git => return None,
                        git_config::Source::User if !use_user => return None,
                        _ => {}
                    }
                    let path = source
                        .storage_location(&mut |name| {
                            match name {
                                git_ if git_.starts_with("GIT_") => Some(git_prefix),
                                "XDG_CONFIG_HOME" => Some(xdg_config_home_env),
                                "HOME" => Some(home_env),
                                _ => None,
                            }
                            .and_then(|perm| std::env::var_os(name).and_then(|val| perm.check(val).ok().flatten()))
                        })
                        .map(|p| p.into_owned());

                    git_config::file::Metadata {
                        path,
                        source: *source,
                        level: 0,
                        trust: git_sec::Trust::Full,
                    }
                    .into()
                });

            let err_on_nonexisting_paths = false;
            let mut globals = git_config::File::from_paths_metadata_buf(
                metas,
                &mut buf,
                err_on_nonexisting_paths,
                git_config::file::init::Options {
                    includes: git_config::file::includes::Options::no_follow(),
                    ..options
                },
            )
            .map_err(|err| match err {
                git_config::file::init::from_paths::Error::Init(err) => Error::from(err),
                git_config::file::init::from_paths::Error::Io(err) => err.into(),
            })?
            .unwrap_or_default();

            globals.append(git_dir_config);
            globals.resolve_includes(options)?;
            if use_env {
                globals.append(git_config::File::from_env(options)?.unwrap_or_default());
            }
            globals
        };

        let excludes_file = config
            .path_filter("core", None, "excludesFile", &mut filter_config_section)
            .map(|p| p.interpolate(options.includes.interpolate).map(|p| p.into_owned()))
            .transpose()?;

        let mut hex_len = None;
        if let Some(hex_len_str) = config.string("core", None, "abbrev") {
            if hex_len_str.trim().is_empty() {
                return Err(Error::EmptyValue { key: "core.abbrev" });
            }
            if !hex_len_str.eq_ignore_ascii_case(b"auto") {
                let value_bytes = hex_len_str.as_ref();
                if let Ok(false) = Boolean::try_from(value_bytes).map(Into::into) {
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

        let reflog = query_refupdates(&config);
        let ignore_case = config_bool(&config, "core.ignoreCase", false)?;
        let use_multi_pack_index = config_bool(&config, "core.multiPackIndex", true)?;
        let object_kind_hint = config.string("core", None, "disambiguate").and_then(|value| {
            Some(match value.as_ref().as_ref() {
                b"commit" => ObjectKindHint::Commit,
                b"committish" => ObjectKindHint::Committish,
                b"tree" => ObjectKindHint::Tree,
                b"treeish" => ObjectKindHint::Treeish,
                b"blob" => ObjectKindHint::Blob,
                _ => return None,
            })
        });
        Ok(Cache {
            resolved: config.into(),
            use_multi_pack_index,
            object_hash,
            object_kind_hint,
            reflog,
            is_bare,
            ignore_case,
            hex_len,
            excludes_file,
            xdg_config_home_env,
            home_env,
            personas: Default::default(),
            git_prefix,
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
    pub fn home_dir(&self) -> Option<PathBuf> {
        std::env::var_os("HOME")
            .map(PathBuf::from)
            .and_then(|path| self.home_env.check(path).ok().flatten())
    }
}

/// Access
impl Cache {
    pub fn personas(&self) -> &identity::Personas {
        self.personas
            .get_or_init(|| identity::Personas::from_config_and_env(&self.resolved, &self.git_prefix))
    }
}

pub(crate) fn interpolate_context<'a>(
    git_install_dir: Option<&'a std::path::Path>,
    home_dir: Option<&'a std::path::Path>,
) -> git_config::path::interpolate::Context<'a> {
    git_config::path::interpolate::Context {
        git_install_dir,
        home_dir,
        home_for_user: Some(git_config::path::interpolate::home_for_user), // TODO: figure out how to configure this
    }
}

fn base_options(lossy: Option<bool>) -> git_config::file::init::Options<'static> {
    git_config::file::init::Options {
        lossy: lossy.unwrap_or(!cfg!(debug_assertions)),
        ..Default::default()
    }
}

fn config_bool(config: &git_config::File<'_>, key: &str, default: bool) -> Result<bool, Error> {
    let (section, key) = key.split_once('.').expect("valid section.key format");
    config
        .boolean(section, None, key)
        .unwrap_or(Ok(default))
        .map_err(|err| Error::DecodeBoolean {
            value: err.input,
            key: key.into(),
        })
}

fn query_refupdates(config: &git_config::File<'static>) -> Option<git_ref::store::WriteReflog> {
    config.string("core", None, "logallrefupdates").map(|val| {
        (val.eq_ignore_ascii_case(b"always"))
            .then(|| git_ref::store::WriteReflog::Always)
            .or_else(|| {
                git_config::Boolean::try_from(val)
                    .ok()
                    .and_then(|b| b.is_true().then(|| git_ref::store::WriteReflog::Normal))
            })
            .unwrap_or(git_ref::store::WriteReflog::Disable)
    })
}
