use std::borrow::Cow;

use crate::{
    bstr::BStr,
    config::{cache::interpolate_context, Snapshot},
};

/// Access configuration values, frozen in time, using a `key` which is a `.` separated string of up to
/// three tokens, namely `section_name.[subsection_name.]value_name`, like `core.bare` or `remote.origin.url`.
///
/// Note that single-value methods always return the last value found, which is the one set most recently in the
/// hierarchy of configuration files, aka 'last one wins'.
impl<'repo> Snapshot<'repo> {
    /// Return the boolean at `key`, or `None` if there is no such value or if the value can't be interpreted as
    /// boolean.
    ///
    /// For a non-degenerating version, use [`try_boolean(…)`][Self::try_boolean()].
    ///
    /// Note that this method takes the most recent value at `key` even if it is from a file with reduced trust.
    pub fn boolean(&self, key: &str) -> Option<bool> {
        self.try_boolean(key).and_then(Result::ok)
    }

    /// Like [`boolean()`][Self::boolean()], but it will report an error if the value couldn't be interpreted as boolean.
    pub fn try_boolean(&self, key: &str) -> Option<Result<bool, git_config::value::Error>> {
        let key = git_config::parse::key(key)?;
        self.repo
            .config
            .resolved
            .boolean(key.section_name, key.subsection_name, key.value_name)
    }

    /// Return the resolved integer at `key`, or `None` if there is no such value or if the value can't be interpreted as
    /// integer or exceeded the value range.
    ///
    /// For a non-degenerating version, use [`try_integer(…)`][Self::try_integer()].
    ///
    /// Note that this method takes the most recent value at `key` even if it is from a file with reduced trust.
    pub fn integer(&self, key: &str) -> Option<i64> {
        self.try_integer(key).and_then(Result::ok)
    }

    /// Like [`integer()`][Self::integer()], but it will report an error if the value couldn't be interpreted as boolean.
    pub fn try_integer(&self, key: &str) -> Option<Result<i64, git_config::value::Error>> {
        let key = git_config::parse::key(key)?;
        self.repo
            .config
            .resolved
            .integer(key.section_name, key.subsection_name, key.value_name)
    }

    /// Return the string at `key`, or `None` if there is no such value.
    ///
    /// Note that this method takes the most recent value at `key` even if it is from a file with reduced trust.
    pub fn string(&self, key: &str) -> Option<Cow<'_, BStr>> {
        let key = git_config::parse::key(key)?;
        self.repo
            .config
            .resolved
            .string(key.section_name, key.subsection_name, key.value_name)
    }

    /// Return the trusted and fully interpolated path at `key`, or `None` if there is no such value
    /// or if no value was found in a trusted file.
    /// An error occurs if the path could not be interpolated to its final value.
    pub fn trusted_path(
        &self,
        key: &str,
    ) -> Option<Result<Cow<'_, std::path::Path>, git_config::path::interpolate::Error>> {
        let key = git_config::parse::key(key)?;
        let path = self.repo.config.resolved.path_filter(
            key.section_name,
            key.subsection_name,
            key.value_name,
            &mut self.repo.filter_config_section(),
        )?;

        let install_dir = self.repo.install_dir().ok();
        let home = self.repo.config.home_dir();
        Some(path.interpolate(interpolate_context(install_dir.as_deref(), home.as_deref())))
    }
}

///
pub mod apply_cli_overrides {
    use crate::bstr::{BString, ByteSlice};
    use crate::config::SnapshotMut;
    use std::convert::TryFrom;

    /// The error returned by [SnapshotMut::apply_cli_overrides()][crate::config::SnapshotMut::apply_cli_overrides()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("{input:?} is not a valid configuration key. Examples are 'core.abbrev' or 'remote.origin.url'")]
        InvalidKey { input: BString },
        #[error("Key {key:?} could not be parsed")]
        SectionKey {
            key: BString,
            source: git_config::parse::section::key::Error,
        },
        #[error(transparent)]
        SectionHeader(#[from] git_config::parse::section::header::Error),
    }

    impl SnapshotMut<'_> {
        /// Apply configuration values of the form `core.abbrev=5` or `remote.origin.url = foo` or `core.bool-implicit-true`
        /// to the repository configuration, marked with [source CLI][git_config::Source::Cli].
        pub fn apply_cli_overrides(
            &mut self,
            values: impl IntoIterator<Item = impl Into<BString>>,
        ) -> Result<(), Error> {
            let mut file = git_config::File::new(git_config::file::Metadata::from(git_config::Source::Cli));
            for key_value in values {
                let key_value = key_value.into();
                let mut tokens = key_value.splitn(2, |b| *b == b'=').map(|v| v.trim());
                let key = tokens.next().expect("always one value").as_bstr();
                let value = tokens.next();
                let key = git_config::parse::key(key.to_str().map_err(|_| Error::InvalidKey { input: key.into() })?)
                    .ok_or_else(|| Error::InvalidKey { input: key.into() })?;
                let mut section = file.section_mut_or_create_new(key.section_name, key.subsection_name)?;
                section.push(
                    git_config::parse::section::Key::try_from(key.value_name.to_owned()).map_err(|err| {
                        Error::SectionKey {
                            source: err,
                            key: key.value_name.into(),
                        }
                    })?,
                    value.map(|v| v.as_bstr()),
                );
            }
            self.config.append(file);
            Ok(())
        }
    }
}

/// Utilities and additional access
impl<'repo> Snapshot<'repo> {
    /// Returns the underlying configuration implementation for a complete API, despite being a little less convenient.
    ///
    /// It's expected that more functionality will move up depending on demand.
    pub fn plumbing(&self) -> &git_config::File<'static> {
        &self.repo.config.resolved
    }
}

mod _impls {
    use crate::config::{Snapshot, SnapshotMut};
    use std::fmt::{Debug, Formatter};
    use std::ops::{Deref, DerefMut};

    impl Debug for Snapshot<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str(&self.repo.config.resolved.to_string())
        }
    }

    impl Debug for SnapshotMut<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str(&self.config.to_string())
        }
    }

    impl Drop for SnapshotMut<'_> {
        fn drop(&mut self) {
            self.repo.config.resolved = std::mem::take(&mut self.config).into();
        }
    }

    impl Deref for SnapshotMut<'_> {
        type Target = git_config::File<'static>;

        fn deref(&self) -> &Self::Target {
            &self.config
        }
    }

    impl DerefMut for SnapshotMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.config
        }
    }
}
