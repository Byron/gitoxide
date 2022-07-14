/// Converts string to a bstr
pub fn b(s: &str) -> &bstr::BStr {
    s.into()
}

mod normalize;
