#![allow(unused)]
#![allow(clippy::result_unit_err)]

use std::{
    borrow::Cow,
    convert::TryFrom,
    path::{Path, PathBuf},
};

use crate::file::from_env;
use crate::{lookup, File};

use crate::file::from_paths;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ConfigSource {
    /// System-wide configuration path. This is defined as
    /// `$(prefix)/etc/gitconfig`.
    System,
    /// Also known as the user configuration path. This is usually `~/.gitconfig`.
    Global,
    /// Second user-specific configuration path; if `$XDG_CONFIG_HOME` is not
    /// set or empty, `$HOME/.config/git/config` will be used. Any single-valued
    /// variable set in this file will be overridden by whatever is in the
    /// Global configuration file.
    User,

    Repository,
    // Worktree(&'a Path),
    /// Config<'_> values parsed from the environment.
    Env,
    Cli,
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, Default)]
pub struct ConfigBuilder {
    no_system: bool,
    load_env_conf: bool,
    override_system_config: Option<PathBuf>,
    override_global_config: Option<PathBuf>,
    override_repo_config: Option<PathBuf>,
}

impl ConfigBuilder {
    /// Constructs a new builder that finds the default location
    #[must_use]
    pub fn new() -> Self {
        Self {
            load_env_conf: true,
            ..Self::default()
        }
    }

    /// Whether or not to skip reading settings from the system-wide
    /// `$(prefix)/etc/gitconfig` file. This corresponds to setting the
    /// `GIT_CONFIG_NOSYSTEM` environment variable.
    #[must_use]
    pub fn no_system(&mut self, no_system: bool) -> &mut Self {
        self.no_system = no_system;
        self
    }

    /// Whether or not to respect `GIT_CONFIG_COUNT`, `GIT_CONFIG_KEY_<n>`, and
    /// `GIT_CONFIG_VALUE_<n>` environment variables. By default, this is true.
    #[must_use]
    pub fn load_environment_entries(&mut self, load_conf: bool) -> &mut Self {
        self.load_env_conf = load_conf;
        self
    }

    /// Override the system-wide configuration file location. Providing [`None`]
    /// or not calling this method will use the default location.
    #[must_use]
    pub fn system_config_path(&mut self, path: Option<PathBuf>) -> &mut Self {
        self.override_system_config = path;
        self
    }

    /// Override the global (user) configuration file location. Providing
    /// [`None`] or not calling this method will use the default location.
    #[must_use]
    pub fn global_config_path(&mut self, path: Option<PathBuf>) -> &mut Self {
        self.override_global_config = path;
        self
    }

    /// Sets where to read the repository-specific configuration file. This
    /// is equivalent to setting `GIT_CONFIG`. If none is provided, then the
    /// builder will look in the default location, `.git/config`.
    #[must_use]
    pub fn repository_config_path(&mut self, path: Option<PathBuf>) -> &mut Self {
        self.override_repo_config = path;
        self
    }

    /// Builds a config, ignoring any failed configuration files.
    #[must_use]
    pub fn build(&self) -> Config<'_> {
        let system_conf = if self.no_system { None } else { todo!() };

        let global_conf = {
            let path = self
                .override_global_config
                .as_ref()
                .map_or_else(|| Path::new(".git/config"), PathBuf::as_path);

            File::open(path).ok()
        };

        let env_conf = if self.load_env_conf {
            // TODO: when bringing up the system, make sure options can be passed. Have to review this entire module first though.
            from_env(from_paths::Options::default()).ok().flatten()
        } else {
            None
        };

        // let user_conf = if self.

        Config {
            system_conf,
            global_conf,
            user_conf: todo!(),
            repository_conf: todo!(),
            worktree_conf: todo!(),
            env_conf,
            cli_conf: todo!(),
        }
    }

    /// Attempts to build a config, returning error if the environment variable
    /// is invalid, if a config file is invalid, or if an overridden config file
    /// does not exist. This is only recommended when you have a very controlled
    /// system state. Otherwise, this will likely fail more often than you'd
    /// like.
    pub fn try_build(&self) -> Result<Config<'_>, ()> {
        todo!()
    }
}

pub struct Config<'a> {
    system_conf: Option<File<'a>>,
    global_conf: Option<File<'a>>,
    user_conf: Option<File<'a>>,
    repository_conf: Option<File<'a>>,
    worktree_conf: Option<File<'a>>,
    env_conf: Option<File<'a>>,
    cli_conf: Option<File<'a>>,
}

impl<'a> Config<'a> {
    #[must_use]
    pub fn value<T: TryFrom<Cow<'a, [u8]>>>(
        &'a self,
        section_name: &str,
        subsection_name: Option<&str>,
        key: &str,
    ) -> Option<T> {
        self.value_with_source(section_name, subsection_name, key)
            .map(|(value, _)| value)
    }

    fn value_with_source<T: TryFrom<Cow<'a, [u8]>>>(
        &'a self,
        section_name: &str,
        subsection_name: Option<&str>,
        key: &str,
    ) -> Option<(T, ConfigSource)> {
        let mapping = self.mapping();

        for (conf, source) in mapping.iter() {
            if let Some(conf) = conf {
                if let Ok(v) = conf.value(section_name, subsection_name, key) {
                    return Some((v, *source));
                }
            }
        }

        None
    }

    pub fn try_value<'lookup, T: TryFrom<Cow<'a, [u8]>>>(
        &'a self,
        section_name: &'lookup str,
        subsection_name: Option<&'lookup str>,
        key: &'lookup str,
    ) -> Result<Option<T>, lookup::Error<T::Error>> {
        self.try_value_with_source(section_name, subsection_name, key)
            .map(|res| res.map(|(value, _)| value))
    }

    /// Tries to retrieve the value, returning an error if the parsing fails or
    /// if the key was not found. On a successful parse, the value will be
    /// returned as well as the source location. This respects the priority of
    /// the various configuration files.
    pub fn try_value_with_source<'lookup, T: TryFrom<Cow<'a, [u8]>>>(
        &'a self,
        section_name: &'lookup str,
        subsection_name: Option<&'lookup str>,
        key: &'lookup str,
    ) -> Result<Option<(T, ConfigSource)>, lookup::Error<T::Error>> {
        let mapping = self.mapping();

        for (conf, source) in mapping.iter() {
            if let Some(conf) = conf {
                return Ok(Some((conf.value(section_name, subsection_name, key)?, *source)));
            }
        }

        Ok(None)
    }

    /// Returns a mapping from [`File`] to [`ConfigSource`]
    const fn mapping(&self) -> [(&Option<File<'_>>, ConfigSource); 6] {
        [
            (&self.cli_conf, ConfigSource::Cli),
            (&self.env_conf, ConfigSource::Env),
            (&self.repository_conf, ConfigSource::Repository),
            (&self.user_conf, ConfigSource::User),
            (&self.global_conf, ConfigSource::Global),
            (&self.system_conf, ConfigSource::System),
        ]
    }
}

/// Lower-level interface for directly accessing a
impl<'a> Config<'a> {
    /// Retrieves the underlying [`File`] object, if one was found during
    /// initialization.
    #[must_use]
    pub fn config(&self, source: ConfigSource) -> Option<&File<'a>> {
        match source {
            ConfigSource::System => self.system_conf.as_ref(),
            ConfigSource::Global => self.global_conf.as_ref(),
            ConfigSource::User => self.user_conf.as_ref(),
            ConfigSource::Repository => self.repository_conf.as_ref(),
            ConfigSource::Env => self.env_conf.as_ref(),
            ConfigSource::Cli => self.cli_conf.as_ref(),
        }
    }

    /// Retrieves the underlying [`File`] object as a mutable reference,
    /// if one was found during initialization.
    #[must_use]
    pub fn config_mut(&mut self, source: ConfigSource) -> Option<&mut File<'a>> {
        match source {
            ConfigSource::System => self.system_conf.as_mut(),
            ConfigSource::Global => self.global_conf.as_mut(),
            ConfigSource::User => self.user_conf.as_mut(),
            ConfigSource::Repository => self.repository_conf.as_mut(),
            ConfigSource::Env => self.env_conf.as_mut(),
            ConfigSource::Cli => self.cli_conf.as_mut(),
        }
    }
}
