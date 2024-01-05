use bstr::BStr;

/// Parsed elements of a Git config key, like `remote.origin.url` or `core.bare`.
pub trait Key {
    /// The name of the section key, like `url` in `remote.origin.url`.
    fn name(&self) -> &str;
    /// The name of the section, like `core` in `core.bare`.
    fn section_name(&self) -> &str;
    /// The name of the sub-section, like `origin` in `remote.origin.url`.
    fn subsection_name(&self) -> Option<&BStr>;
}
