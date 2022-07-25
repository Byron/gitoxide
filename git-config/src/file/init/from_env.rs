use std::{borrow::Cow, convert::TryFrom};

use crate::{file, file::init, parse, parse::section, path::interpolate, File};

/// Represents the errors that may occur when calling [`File::from_env()`].
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
    /// Generates a config from `GIT_CONFIG_*` environment variables or returns `Ok(None)` if no configuration was found.
    /// See [`git-config`'s documentation] for more information on the environment variables in question.
    ///
    /// With `options` configured, it's possible to resolve `include.path` or `includeIf.<condition>.path` directives as well.
    ///
    /// [`git-config`'s documentation]: https://git-scm.com/docs/git-config#Documentation/git-config.txt-GITCONFIGCOUNT
    pub fn from_env(options: init::Options<'_>) -> Result<Option<File<'static>>, Error> {
        use std::env;
        let count: usize = match env::var("GIT_CONFIG_COUNT") {
            Ok(v) => v.parse().map_err(|_| Error::InvalidConfigCount { input: v })?,
            Err(_) => return Ok(None),
        };

        if count == 0 {
            return Ok(None);
        }

        let meta = file::Metadata {
            path: None,
            source: crate::Source::Env,
            level: 0,
            trust: git_sec::Trust::Full,
        };
        let mut config = File::new(meta);
        for i in 0..count {
            let key = env::var(format!("GIT_CONFIG_KEY_{}", i)).map_err(|_| Error::InvalidKeyId { key_id: i })?;
            let value = env::var_os(format!("GIT_CONFIG_VALUE_{}", i)).ok_or(Error::InvalidValueId { value_id: i })?;
            let key = parse::key(&key).ok_or_else(|| Error::InvalidKeyValue {
                key_id: i,
                key_val: key.to_string(),
            })?;

            let mut section = match config.section_mut(key.section_name, key.subsection_name) {
                Ok(section) => section,
                Err(_) => config.new_section(
                    key.section_name.to_owned(),
                    key.subsection_name.map(|subsection| Cow::Owned(subsection.to_owned())),
                )?,
            };

            section.push(
                section::Key::try_from(key.value_name.to_owned())?,
                git_path::os_str_into_bstr(&value).expect("no illformed UTF-8").as_ref(),
            );
        }

        let mut buf = Vec::new();
        init::includes::resolve(&mut config, &mut buf, options)?;
        Ok(Some(config))
    }
}
