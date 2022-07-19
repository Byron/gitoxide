use crate::bstr::BStr;
use crate::config::cache::interpolate_context;
use crate::config::Snapshot;
use std::borrow::Cow;
use std::fmt::{Debug, Formatter};

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
    /// An error occours if the path could not be interpolated to its final value.
    pub fn trusted_path(
        &self,
        key: &str,
    ) -> Option<Result<Cow<'_, std::path::Path>, git_config::path::interpolate::Error>> {
        let key = git_config::parse::key(key)?;
        let path = self.repo.config.resolved.path_filter(
            key.section_name,
            key.subsection_name,
            key.value_name,
            &mut self
                .repo
                .options
                .filter_config_section
                .unwrap_or(crate::config::section::is_trusted),
        )?;

        let install_dir = self.repo.install_dir().ok();
        let home = self.repo.config.home_dir();
        Some(path.interpolate(interpolate_context(install_dir.as_deref(), home.as_deref())))
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

impl Debug for Snapshot<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if cfg!(debug_assertions) {
            f.write_str(&self.repo.config.resolved.to_string())
        } else {
            Debug::fmt(&self.repo.config.resolved, f)
        }
    }
}
