/// Provide simplified access to git configuration values
impl crate::Repository {
    /// Return the integer value at `key` (like `core.abbrev`) or use the given `default` value if it isn't present.
    // TODO: figure out how to identify sub-sections, or how to design such an API. This is really just a first test.
    // TODO: tests
    pub fn config_int(&self, key: &str, default: i64) -> i64 {
        let (section, key) = key.split_once('.').expect("valid section.key format");
        self.config
            .value::<git_config::values::Integer>(section, None, key)
            .map_or(default, |v| v.value)
    }
}
