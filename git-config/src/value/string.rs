use bstr::BStr;
use std::borrow::Cow;

/// Any string value
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct String<'a> {
    /// The string value
    pub value: Cow<'a, BStr>,
}

impl<'a> From<Cow<'a, BStr>> for String<'a> {
    fn from(c: Cow<'a, BStr>) -> Self {
        String {
            value: crate::values::normalize(c),
        }
    }
}
