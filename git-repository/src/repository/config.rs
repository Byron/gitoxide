use crate::config;

/// Provide simplified access to git configuration values
impl crate::Repository {
    /// Return the integer value at `key` (like `core.abbrev`) or use the given `default` value if it isn't present.
    pub fn config_int(_key: &str, _default: i64) -> Result<i64, config::query::Error> {
        todo!()
    }
}
