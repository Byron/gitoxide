use crate::Trust;

impl Trust {
    /// Derive `Full` trust if `path` is owned by the user executing the current process, or `Reduced` trust otherwise.
    pub fn from_path_ownership(path: &std::path::Path) -> std::io::Result<Self> {
        Ok(if crate::identity::is_path_owned_by_current_user(path)? {
            Trust::Full
        } else {
            Trust::Reduced
        })
    }
}

/// A trait to help creating default values based on a trust level.
pub trait DefaultForLevel {
    /// Produce a default value for the given trust `level`.
    fn default_for_level(level: Trust) -> Self;
}

/// Associate instructions for how to deal with various `Trust` levels as they are encountered in the wild.
pub struct Mapping<T> {
    /// The value for fully trusted resources.
    pub full: T,
    /// The value for resources with reduced trust.
    pub reduced: T,
}

impl<T> Default for Mapping<T>
where
    T: DefaultForLevel,
{
    fn default() -> Self {
        Mapping {
            full: T::default_for_level(Trust::Full),
            reduced: T::default_for_level(Trust::Reduced),
        }
    }
}

impl<T> Mapping<T> {
    /// Obtain the value for the given trust `level`.
    pub fn by_level(&self, level: Trust) -> &T {
        match level {
            Trust::Full => &self.full,
            Trust::Reduced => &self.reduced,
        }
    }

    /// Obtain the value for the given `level` once.
    pub fn into_value_by_level(self, level: Trust) -> T {
        match level {
            Trust::Full => self.full,
            Trust::Reduced => self.reduced,
        }
    }
}
