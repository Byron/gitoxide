use crate::Snapshot;
use bstr::{BStr, BString, ByteSlice};
use std::cmp::Ordering;
use std::ops::Deref;

#[cfg_attr(test, derive(Debug))]
#[derive(Clone)]
enum EncodedString {
    Utf8(String),
    Unknown(BString),
}

impl EncodedString {
    fn cmp_ref(&self, other: EncodedStringRef<'_>) -> Ordering {
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
enum EncodedStringRef<'a> {
    Utf8(&'a str),
    Unknown(&'a BStr),
}

impl EncodedString {
    fn to_ref(&self) -> EncodedStringRef<'_> {
        match self {
            EncodedString::Unknown(v) => EncodedStringRef::Unknown(v.deref().as_bstr()),
            EncodedString::Utf8(v) => EncodedStringRef::Utf8(v),
        }
    }
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
        self.cmp_ref(other.to_ref())
    }
}

#[derive(Clone)]
struct NameEntry {
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

impl Snapshot {
    pub fn from_bytes(buf: &[u8]) -> Self {
        Self::new(crate::parse_ignore_errors(buf))
    }

    pub fn new<'a>(entries: impl IntoIterator<Item = crate::Entry<'a>>) -> Self {
        let mut snapshot = Self::default();
        snapshot.extend(entries);
        snapshot
    }

    pub fn extend<'a>(&mut self, entries: impl IntoIterator<Item = crate::Entry<'a>>) -> &mut Self {
        todo!()
    }

    pub fn try_resolve(&self, signature: &git_actor::SignatureRef<'_>) -> Option<git_actor::Signature> {
        todo!()
    }

    pub fn resolve(&self, signature: &git_actor::SignatureRef<'_>) -> git_actor::Signature {
        self.try_resolve(signature).unwrap_or_else(|| signature.to_owned())
    }
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
