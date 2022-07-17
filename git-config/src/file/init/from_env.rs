use git_features::threading::OwnShared;
use std::convert::TryFrom;
use std::{borrow::Cow, path::PathBuf};

use crate::file::{init, Metadata};
use crate::{file, parse::section, path::interpolate, File, Source};

/// Represents the errors that may occur when calling [`File::from_env`][crate::File::from_env()].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("GIT_CONFIG_COUNT was not a positive integer: {}", .input)]
    InvalidConfigCount { input: String },
    #[error("GIT_CONFIG_KEY_{} was not set", .key_id)]
    InvalidKeyId { key_id: usize },
    #[error("GIT_CONFIG_KEY_{} was set to an invalid value: {}", .key_id, .key_val)]
    InvalidKeyValue { key_id: usize, key_val: String },
    #[error("GIT_CONFIG_VALUE_{} was not set", .value_id)]
    InvalidValueId { value_id: usize },
    #[error(transparent)]
    PathInterpolationError(#[from] interpolate::Error),
    #[error(transparent)]
    Includes(#[from] init::includes::Error),
    #[error(transparent)]
    Section(#[from] section::header::Error),
    #[error(transparent)]
    Key(#[from] section::key::Error),
}

/// Instantiation from environment variables
impl File<'static> {
    /// Constructs a `git-config` from the default cascading sequence of global configuration files,
    /// excluding any repository-local configuration.
    ///
    /// See <https://git-scm.com/docs/git-config#FILES> for details.
    // TODO: how does this relate to the `fs` module? Have a feeling options should contain instructions on which files to use.
    pub fn from_env_paths(options: init::Options<'_>) -> Result<File<'static>, init::from_paths::Error> {
        use std::env;

        let mut metas = vec![];
        let mut push_path = |path: PathBuf, source: Source, trust: Option<git_sec::Trust>| {
            if let Some(meta) = trust
                .or_else(|| git_sec::Trust::from_path_ownership(&path).ok())
                .map(|trust| Metadata {
                    path: Some(path),
                    trust,
                    level: 0,
                    source,
                })
            {
                metas.push(meta)
            }
        };

        if env::var("GIT_CONFIG_NO_SYSTEM").is_err() {
            let git_config_system_path = env::var_os("GIT_CONFIG_SYSTEM").unwrap_or_else(|| "/etc/gitconfig".into());
            push_path(
                PathBuf::from(git_config_system_path),
                Source::System,
                git_sec::Trust::Full.into(),
            );
        }

        if let Some(git_config_global) = env::var_os("GIT_CONFIG_GLOBAL") {
            push_path(
                PathBuf::from(git_config_global),
                Source::Global,
                git_sec::Trust::Full.into(),
            );
        } else {
            // Divergence from git-config(1)
            // These two are supposed to share the same scope and override
            // rather than append according to git-config(1) documentation.
            if let Some(xdg_config_home) = env::var_os("XDG_CONFIG_HOME") {
                push_path(
                    PathBuf::from(xdg_config_home).join("git/config"),
                    Source::User,
                    git_sec::Trust::Full.into(),
                );
            } else if let Some(home) = env::var_os("HOME") {
                push_path(
                    PathBuf::from(home).join(".config/git/config"),
                    Source::User,
                    git_sec::Trust::Full.into(),
                );
            }

            if let Some(home) = env::var_os("HOME") {
                push_path(
                    PathBuf::from(home).join(".gitconfig"),
                    Source::User,
                    git_sec::Trust::Full.into(),
                );
            }
        }

        // TODO: remove this in favor of a clear non-local approach to integrate better with git-repository
        if let Some(git_dir) = env::var_os("GIT_DIR") {
            push_path(PathBuf::from(git_dir).join("config"), Source::Local, None);
        }

        File::from_paths_metadata(metas, options)
    }

    /// Generates a config from the environment variables. This is neither
    /// zero-copy nor zero-alloc. See [`git-config`'s documentation] on
    /// environment variable for more information.
    ///
    /// [`git-config`'s documentation]: https://git-scm.com/docs/git-config#Documentation/git-config.txt-GITCONFIGCOUNT
    // TODO: use `init::Options` instead for lossy support.
    pub fn from_env(options: init::Options<'_>) -> Result<Option<File<'static>>, Error> {
        use std::env;
        let count: usize = match env::var("GIT_CONFIG_COUNT") {
            Ok(v) => v.parse().map_err(|_| Error::InvalidConfigCount { input: v })?,
            Err(_) => return Ok(None),
        };

        if count == 0 {
            return Ok(None);
        }

        let meta = OwnShared::new(file::Metadata {
            path: None,
            source: crate::Source::Env,
            level: 0,
            trust: git_sec::Trust::Full,
        });
        let mut config = File::new(OwnShared::clone(&meta));
        for i in 0..count {
            let key = env::var(format!("GIT_CONFIG_KEY_{}", i)).map_err(|_| Error::InvalidKeyId { key_id: i })?;
            let value = env::var_os(format!("GIT_CONFIG_VALUE_{}", i)).ok_or(Error::InvalidValueId { value_id: i })?;
            match key.split_once('.') {
                Some((section_name, maybe_subsection)) => {
                    let (subsection, key) = match maybe_subsection.rsplit_once('.') {
                        Some((subsection, key)) => (Some(subsection), key),
                        None => (None, maybe_subsection),
                    };

                    let mut section = match config.section_mut(section_name, subsection) {
                        Ok(section) => section,
                        Err(_) => config.new_section(
                            section_name.to_string(),
                            subsection.map(|subsection| Cow::Owned(subsection.to_string())),
                        )?,
                    };

                    section.push(
                        section::Key::try_from(key.to_owned())?,
                        git_path::os_str_into_bstr(&value).expect("no illformed UTF-8").as_ref(),
                    );
                }
                None => {
                    return Err(Error::InvalidKeyValue {
                        key_id: i,
                        key_val: key.to_string(),
                    })
                }
            }
        }

        let mut buf = Vec::new();
        init::includes::resolve(&mut config, meta, &mut buf, options)?;
        Ok(Some(config))
    }
}
