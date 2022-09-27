use git_features::threading::OwnShared;
use std::borrow::Cow;

use crate::config::{CommitAndRollback, SnapshotMut};
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

/// Utilities and additional access
impl<'repo> Snapshot<'repo> {
    /// Returns the underlying configuration implementation for a complete API, despite being a little less convenient.
    ///
    /// It's expected that more functionality will move up depending on demand.
    pub fn plumbing(&self) -> &git_config::File<'static> {
        &self.repo.config.resolved
    }
}

/// Utilities
impl<'repo> SnapshotMut<'repo> {
    /// Apply all changes made to this instance.
    ///
    /// Note that this would also happen once this instance is dropped, but using this method may be more intuitive.
    pub fn commit(self) {}

    /// Create a structure the temporarily commits the changes, but rolls them back when dropped.
    pub fn commit_and_rollback(mut self) -> CommitAndRollback<'repo> {
        let new_config = std::mem::take(&mut self.config);
        let repo = self.repo.take().expect("this only runs once on consumption");
        repo.config.resolved = new_config.into();
        let prev_config = OwnShared::clone(&repo.config.resolved);
        CommitAndRollback { repo, prev_config }
    }

    /// Don't apply any of the changes after consuming this instance, effectively forgetting them, returning the changed configuration.
    pub fn forget(mut self) -> git_config::File<'static> {
        std::mem::take(&mut self.config)
    }
}
