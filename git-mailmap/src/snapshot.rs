use crate::Snapshot;
use bstr::{BStr, BString, ByteSlice};
use git_actor::{Signature, SignatureRef};
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
#[derive(Clone, Copy)]
enum EncodedStringRef<'a> {
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

impl EmailEntry {
    fn merge(
        &mut self,
        crate::Entry {
            new_name,
            new_email,
            old_name,
            old_email: _,
        }: crate::Entry<'_>,
    ) {
        let new_email = new_email.map(ToOwned::to_owned);
        let new_name = new_name.map(ToOwned::to_owned);
        match old_name {
            None => {
                self.new_email = new_email;
                self.new_name = new_name;
            }
            Some(old_name) => {
                let old_name: EncodedStringRef<'_> = old_name.into();
                match self
                    .entries_by_old_name
                    .binary_search_by(|e| e.old_name.cmp_ref(old_name))
                {
                    Ok(pos) => {
                        let entry = &mut self.entries_by_old_name[pos];
                        entry.new_name = new_name;
                        entry.new_email = new_email;
                    }
                    Err(insert_pos) => self.entries_by_old_name.insert(
                        insert_pos,
                        NameEntry {
                            new_name,
                            new_email,
                            old_name: old_name.into(),
                        },
                    ),
                }
            }
        }
    }
}

impl<'a> From<crate::Entry<'a>> for EmailEntry {
    fn from(
        crate::Entry {
            new_name,
            new_email,
            old_name,
            old_email,
        }: crate::Entry<'a>,
    ) -> Self {
        let mut new_name = new_name.map(ToOwned::to_owned);
        let mut new_email = new_email.map(ToOwned::to_owned);
        let entries_by_old_name = old_name
            .map(|name| {
                vec![NameEntry {
                    new_name: new_name.take(),
                    new_email: new_email.take(),
                    old_name: name.into(),
                }]
            })
            .unwrap_or_default();
        EmailEntry {
            new_name,
            new_email,
            old_email: old_email.into(),
            entries_by_old_name,
        }
    }
}

impl Snapshot {
    pub fn from_bytes(buf: &[u8]) -> Self {
        Self::new(crate::parse_ignore_errors(buf))
    }

    pub fn new<'a>(entries: impl IntoIterator<Item = crate::Entry<'a>>) -> Self {
        let mut snapshot = Self::default();
        snapshot.merge(entries);
        snapshot
    }

    pub fn merge<'a>(&mut self, entries: impl IntoIterator<Item = crate::Entry<'a>>) -> &mut Self {
        for entry in entries {
            let old_email: EncodedStringRef<'_> = entry.old_email.into();
            assert!(
                entry.new_name.is_some() || entry.new_email.is_some(),
                "BUG: encountered entry without any mapped/new name or email."
            );
            match self
                .entries_by_old_email
                .binary_search_by(|e| e.old_email.cmp_ref(old_email))
            {
                Ok(pos) => self.entries_by_old_email[pos].merge(entry),
                Err(insert_pos) => {
                    self.entries_by_old_email.insert(insert_pos, entry.into());
                }
            };
        }
        self
    }

    pub fn try_resolve(&self, signature: &git_actor::SignatureRef<'_>) -> Option<git_actor::Signature> {
        let email: EncodedStringRef<'_> = signature.email.into();
        let pos = self
            .entries_by_old_email
            .binary_search_by(|e| e.old_email.cmp_ref(email))
            .ok()?;
        let entry = &self.entries_by_old_email[pos];

        let name: EncodedStringRef<'_> = signature.name.into();

        match entry.entries_by_old_name.binary_search_by(|e| e.old_name.cmp_ref(name)) {
            Ok(pos) => {
                let entry = &entry.entries_by_old_name[pos];
                enriched_signature(signature, entry.new_email.as_ref(), entry.new_name.as_ref())
            }
            Err(_) => enriched_signature(signature, entry.new_email.as_ref(), entry.new_name.as_ref()),
        }
    }

    pub fn resolve(&self, signature: &git_actor::SignatureRef<'_>) -> git_actor::Signature {
        self.try_resolve(signature).unwrap_or_else(|| signature.to_owned())
    }
}

fn enriched_signature(
    SignatureRef { name, email, time }: &SignatureRef<'_>,
    new_email: Option<&BString>,
    new_name: Option<&BString>,
) -> Option<Signature> {
    let time = *time;
    match (new_email, new_name) {
        (Some(new_email), Some(new_name)) => git_actor::Signature {
            email: new_email.to_owned(),
            name: new_name.to_owned(),
            time,
        }
        .into(),
        (Some(new_email), None) => git_actor::Signature {
            email: new_email.to_owned(),
            name: (*name).to_owned(),
            time,
        }
        .into(),
        (None, Some(new_name)) => git_actor::Signature {
            email: (*email).to_owned(),
            name: new_name.to_owned(),
            time,
        }
        .into(),
        (None, None) => None,
    }
}

#[cfg(test)]
mod encoded_string {
    use crate::snapshot::{EncodedString, EncodedStringRef};
    use std::cmp::Ordering;

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
