use std::{cmp::Ordering, ops::Deref};

use bstr::{BStr, BString, ByteSlice};

#[cfg_attr(test, derive(Debug))]
#[derive(Clone)]
pub enum EncodedString {
    Utf8(String),
    Unknown(BString),
}

impl EncodedString {
    pub fn as_bstr(&self) -> &BStr {
        match self {
            EncodedString::Utf8(v) => v.as_str().into(),
            EncodedString::Unknown(v) => v.as_bstr(),
        }
    }
    pub fn cmp_ref(&self, other: EncodedStringRef<'_>) -> Ordering {
        match (self, other) {
            (EncodedString::Utf8(a), EncodedStringRef::Utf8(b)) => {
                let a = a.chars().map(|c| c.to_ascii_lowercase());
                let b = b.chars().map(|c| c.to_ascii_lowercase());
                a.cmp(b)
            }
            (EncodedString::Unknown(a), EncodedStringRef::Unknown(b)) => a.deref().as_bstr().cmp(b),
            (EncodedString::Utf8(a), EncodedStringRef::Unknown(b)) => a.as_bytes().cmp(b.as_ref()),
            (EncodedString::Unknown(a), EncodedStringRef::Utf8(b)) => a.deref().as_bytes().cmp(b.as_bytes()),
        }
    }
}

#[cfg_attr(test, derive(Debug))]
#[derive(Clone, Copy)]
pub enum EncodedStringRef<'a> {
    Utf8(&'a str),
    Unknown(&'a BStr),
}

impl<'a> From<&'a BStr> for EncodedStringRef<'a> {
    fn from(v: &'a BStr) -> Self {
        match v.to_str() {
            Ok(v) => EncodedStringRef::Utf8(v),
            Err(_) => EncodedStringRef::Unknown(v),
        }
    }
}

impl<'a> From<EncodedStringRef<'a>> for EncodedString {
    fn from(v: EncodedStringRef<'a>) -> Self {
        match v {
            EncodedStringRef::Utf8(v) => EncodedString::Utf8(v.to_owned()),
            EncodedStringRef::Unknown(v) => EncodedString::Unknown(v.to_owned()),
        }
    }
}

impl<'a> From<&'a BStr> for EncodedString {
    fn from(v: &'a BStr) -> Self {
        match v.to_str() {
            Ok(v) => EncodedString::Utf8(v.to_owned()),
            Err(_) => EncodedString::Unknown(v.to_owned()),
        }
    }
}

#[cfg(test)]
mod encoded_string {
    use std::cmp::Ordering;

    use crate::snapshot::util::{EncodedString, EncodedStringRef};

    #[test]
    fn basic_ascii_case_folding() {
        assert_eq!(
            EncodedString::Utf8("FooBar".into()).cmp_ref(EncodedStringRef::Utf8("foobar")),
            Ordering::Equal
        );
    }

    #[test]
    fn no_advanced_unicode_folding() {
        assert_ne!(
            EncodedString::Utf8("Masse".into()).cmp_ref(EncodedStringRef::Utf8("Maße")),
            Ordering::Equal
        );
    }

    #[test]
    fn unknown_encoding_pairs_do_not_try_to_ignore_cases() {
        assert_ne!(
            EncodedString::Utf8("Foo".into()).cmp_ref(EncodedStringRef::Unknown("foo".into())),
            Ordering::Equal
        );
        assert_ne!(
            EncodedString::Unknown("Foo".into()).cmp_ref(EncodedStringRef::Utf8("foo")),
            Ordering::Equal
        );
        assert_ne!(
            EncodedString::Unknown("Foo".into()).cmp_ref(EncodedStringRef::Unknown("foo".into())),
            Ordering::Equal
        );
    }
}
