use bstr::BStr;

/// TODO
pub trait Key {
    /// TODO
    fn name(&self) -> &str;
    /// TODO
    fn section_name(&self) -> &str;
    /// TODO
    fn subsection_name(&self) -> Option<&BStr>;
}
