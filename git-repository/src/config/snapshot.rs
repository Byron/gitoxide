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
    /// Note that this method takes the most recent value at `key` even if it is from a file with reduced trust.
    /// For a non-degenerating version, use [`try_boolean(…)`][Self::try_boolean()]
    pub fn boolean(&self, key: &str) -> Option<bool> {
        self.try_boolean(key).map(Result::ok).flatten()
    }

    /// Like [`boolean()`][Self::boolean()], but it will report an error if the value couldn't be interpreted as boolean.
    pub fn try_boolean(&self, key: &str) -> Option<Result<bool, git_config::value::Error>> {
        let key = git_config::parse::key(key)?;
        self.repo
            .config
            .resolved
            .boolean(key.section_name, key.subsection_name, key.value_name)
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
            &mut crate::config::section::is_trusted,
        )?;

        let install_dir = self.repo.install_dir().ok();
        let home = self.repo.config.home_dir();
        Some(path.interpolate(interpolate_context(install_dir.as_deref(), home.as_deref())))
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
