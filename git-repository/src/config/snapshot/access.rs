use std::borrow::Cow;

use git_features::threading::OwnShared;

use crate::{
    bstr::BStr,
    config::{CommitAutoRollback, Snapshot, SnapshotMut},
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
    pub fn boolean<'a>(&self, key: impl Into<&'a BStr>) -> Option<bool> {
        self.try_boolean(key).and_then(Result::ok)
    }

    /// Like [`boolean()`][Self::boolean()], but it will report an error if the value couldn't be interpreted as boolean.
    pub fn try_boolean<'a>(&self, key: impl Into<&'a BStr>) -> Option<Result<bool, git_config::value::Error>> {
        self.repo.config.resolved.boolean_by_key(key)
    }

    /// Return the resolved integer at `key`, or `None` if there is no such value or if the value can't be interpreted as
    /// integer or exceeded the value range.
    ///
    /// For a non-degenerating version, use [`try_integer(…)`][Self::try_integer()].
    ///
    /// Note that this method takes the most recent value at `key` even if it is from a file with reduced trust.
    pub fn integer<'a>(&self, key: impl Into<&'a BStr>) -> Option<i64> {
        self.try_integer(key).and_then(Result::ok)
    }

    /// Like [`integer()`][Self::integer()], but it will report an error if the value couldn't be interpreted as boolean.
    pub fn try_integer<'a>(&self, key: impl Into<&'a BStr>) -> Option<Result<i64, git_config::value::Error>> {
        self.repo.config.resolved.integer_by_key(key)
    }

    /// Return the string at `key`, or `None` if there is no such value.
    ///
    /// Note that this method takes the most recent value at `key` even if it is from a file with reduced trust.
    pub fn string<'a>(&self, key: impl Into<&'a BStr>) -> Option<Cow<'_, BStr>> {
        self.repo.config.resolved.string_by_key(key)
    }

    /// Return the trusted and fully interpolated path at `key`, or `None` if there is no such value
    /// or if no value was found in a trusted file.
    /// An error occurs if the path could not be interpolated to its final value.
    pub fn trusted_path<'a>(
        &self,
        key: impl Into<&'a BStr>,
    ) -> Option<Result<Cow<'_, std::path::Path>, git_config::path::interpolate::Error>> {
        let key = git_config::parse::key(key)?;
        self.repo
            .config
            .trusted_file_path(key.section_name, key.subsection_name, key.value_name)
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
    /// Append configuration values of the form `core.abbrev=5` or `remote.origin.url = foo` or `core.bool-implicit-true`
    /// to the end of the repository configuration, with each section marked with the given `source`.
    ///
    /// Note that doing so applies the configuration at the very end, so it will always override what came before it
    /// even though the `source` is of lower priority as what's there.
    pub fn append_config(
        &mut self,
        values: impl IntoIterator<Item = impl AsRef<BStr>>,
        source: git_config::Source,
    ) -> Result<&mut Self, crate::config::overrides::Error> {
        crate::config::overrides::append(&mut self.config, values, source, |v| Some(format!("-c {v}").into()))?;
        Ok(self)
    }
    /// Apply all changes made to this instance.
    ///
    /// Note that this would also happen once this instance is dropped, but using this method may be more intuitive and won't squelch errors
    /// in case the new configuration is partially invalid.
    pub fn commit(mut self) -> Result<&'repo mut crate::Repository, crate::config::Error> {
        let repo = self.repo.take().expect("always present here");
        self.commit_inner(repo)
    }

    pub(crate) fn commit_inner(
        &mut self,
        repo: &'repo mut crate::Repository,
    ) -> Result<&'repo mut crate::Repository, crate::config::Error> {
        repo.reread_values_and_clear_caches_replacing_config(std::mem::take(&mut self.config).into())?;
        Ok(repo)
    }

    /// Create a structure the temporarily commits the changes, but rolls them back when dropped.
    pub fn commit_auto_rollback(mut self) -> Result<CommitAutoRollback<'repo>, crate::config::Error> {
        let repo = self.repo.take().expect("this only runs once on consumption");
        let prev_config = OwnShared::clone(&repo.config.resolved);

        Ok(CommitAutoRollback {
            repo: self.commit_inner(repo)?.into(),
            prev_config,
        })
    }

    /// Don't apply any of the changes after consuming this instance, effectively forgetting them, returning the changed configuration.
    pub fn forget(mut self) -> git_config::File<'static> {
        self.repo.take();
        std::mem::take(&mut self.config)
    }
}

/// Utilities
impl<'repo> CommitAutoRollback<'repo> {
    /// Rollback the changes previously applied and all values before the change.
    pub fn rollback(mut self) -> Result<&'repo mut crate::Repository, crate::config::Error> {
        let repo = self.repo.take().expect("still present, consumed only once");
        self.rollback_inner(repo)
    }

    pub(crate) fn rollback_inner(
        &mut self,
        repo: &'repo mut crate::Repository,
    ) -> Result<&'repo mut crate::Repository, crate::config::Error> {
        repo.reread_values_and_clear_caches_replacing_config(OwnShared::clone(&self.prev_config))?;
        Ok(repo)
    }
}
