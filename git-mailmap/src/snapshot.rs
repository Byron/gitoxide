use bstr::{BString, ByteSlice};
use std::cmp::Ordering;
use std::ops::Deref;

#[cfg_attr(test, derive(Debug))]
#[derive(Clone)]
enum EncodedString {
    Utf8(String),
    Unknown(BString),
}

impl Eq for EncodedString {}

impl PartialEq<Self> for EncodedString {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd<Self> for EncodedString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cmp(other).into()
    }
}

impl Ord for EncodedString {
    fn cmp(&self, other: &Self) -> Ordering {
        use EncodedString::*;
        match (self, other) {
            (Utf8(a), Utf8(b)) => {
                let a = a.chars().map(|c| c.to_ascii_lowercase());
                let b = b.chars().map(|c| c.to_ascii_lowercase());
                a.cmp(b)
            }
            (Unknown(a), Unknown(b)) => a.cmp(b),
            (Utf8(a), Unknown(b)) => a.as_bytes().cmp(b.as_ref()),
            (Unknown(a), Utf8(b)) => a.deref().as_bytes().cmp(b.as_bytes()),
        }
    }
}

#[derive(Clone)]
pub(crate) struct NameEntry {
    new_name: Option<BString>,
    new_email: Option<BString>,
    old_name: EncodedString,
}

#[derive(Clone)]
pub(crate) struct EmailEntry {
    new_name: Option<BString>,
    new_email: Option<BString>,
    old_email: EncodedString,

    entries_by_old_name: Vec<NameEntry>,
}

#[cfg(test)]
mod encoded_string {
    use crate::snapshot::EncodedString;

    #[test]
    fn basic_ascii_case_folding() {
        assert_eq!(
            EncodedString::Utf8("FooBar".into()),
            EncodedString::Utf8("foobar".into())
        )
    }

    #[test]
    fn no_advanced_unicode_folding() {
        assert_ne!(EncodedString::Utf8("Masse".into()), EncodedString::Utf8("Maße".into()))
    }

    #[test]
    fn unknown_encoding_pairs_do_not_try_to_ignore_cases() {
        assert_ne!(EncodedString::Utf8("Foo".into()), EncodedString::Unknown("foo".into()));
        assert_ne!(EncodedString::Unknown("Foo".into()), EncodedString::Utf8("foo".into()));
        assert_ne!(
            EncodedString::Unknown("Foo".into()),
            EncodedString::Unknown("foo".into())
        );
    }
}
