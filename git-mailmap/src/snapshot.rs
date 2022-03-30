use std::{cmp::Ordering, ops::Deref};

use bstr::{BStr, BString, ByteSlice};
use git_actor::SignatureRef;

use crate::Snapshot;

/// A resolved signature with borrowed fields for a mapped `name` and/or `email`.
pub struct ResolvedSignature<'a> {
    /// The mapped name.
    pub name: Option<&'a BStr>,
    /// The mapped email.
    pub email: Option<&'a BStr>,
}

impl<'a> ResolvedSignature<'a> {
    fn try_new(new_email: Option<&'a BString>, new_name: Option<&'a BString>) -> Option<Self> {
        match (new_email, new_name) {
            (None, None) => None,
            (new_email, new_name) => Some(ResolvedSignature {
                email: new_email.map(|v| v.as_bstr()),
                name: new_name.map(|v| v.as_bstr()),
            }),
        }
    }
}

#[cfg_attr(test, derive(Debug))]
#[derive(Clone)]
enum EncodedString {
    Utf8(String),
    Unknown(BString),
}

impl EncodedString {
    fn as_bstr(&self) -> &BStr {
        match self {
            EncodedString::Utf8(v) => v.as_str().into(),
            EncodedString::Unknown(v) => v.as_bstr(),
        }
    }
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
    /// Create a new snapshot from the given bytes buffer, ignoring all parse errors that may occour on a line-by-line basis.
    ///
    /// This is similar to what git does.
    pub fn from_bytes(buf: &[u8]) -> Self {
        Self::new(crate::parse_ignore_errors(buf))
    }

    /// Create a new instance from `entries`.
    ///
    /// These can be obtained using [crate::parse()].
    pub fn new<'a>(entries: impl IntoIterator<Item = crate::Entry<'a>>) -> Self {
        let mut snapshot = Self::default();
        snapshot.merge(entries);
        snapshot
    }

    /// Merge the given `entries` into this instance, possibly overwriting existing mappings with
    /// new ones should they collide.
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

    /// Transform our acceleration structure into a list of entries.
    ///
    /// Note that the order is different from how they were obtained initially, and are explicitly ordered by
    /// (old_email, old_name).
    pub fn entries(&self) -> Vec<crate::Entry<'_>> {
        let mut out = Vec::with_capacity(self.entries_by_old_email.len());
        for entry in &self.entries_by_old_email {
            if entry.new_email.is_some() || entry.new_name.is_some() {
                out.push(crate::Entry {
                    new_name: entry.new_name.as_ref().map(|b| b.as_bstr()),
                    new_email: entry.new_email.as_ref().map(|b| b.as_bstr()),
                    old_name: None,
                    old_email: entry.old_email.as_bstr(),
                });
            }

            for name_entry in &entry.entries_by_old_name {
                out.push(crate::Entry {
                    new_name: name_entry.new_name.as_ref().map(|b| b.as_bstr()),
                    new_email: name_entry.new_email.as_ref().map(|b| b.as_bstr()),
                    old_name: name_entry.old_name.as_bstr().into(),
                    old_email: entry.old_email.as_bstr(),
                });
            }
        }
        out
    }

    /// Try to resolve `signature` by its contained email and name and provide resolved/mapped names as reference.
    /// Return `None` if no such mapping was found.
    ///
    /// This is the fastest possible lookup as there is no allocation.
    pub fn try_resolve_ref<'a>(&'a self, signature: git_actor::SignatureRef<'_>) -> Option<ResolvedSignature<'a>> {
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
                ResolvedSignature::try_new(entry.new_email.as_ref(), entry.new_name.as_ref())
            }
            Err(_) => ResolvedSignature::try_new(entry.new_email.as_ref(), entry.new_name.as_ref()),
        }
    }

    /// Try to resolve `signature` by its contained email and name and provide resolved/mapped names as owned signature,
    /// with the mapped name and/or email replaced accordingly.
    ///
    /// Return `None` if no such mapping was found.
    pub fn try_resolve(&self, signature: git_actor::SignatureRef<'_>) -> Option<git_actor::Signature> {
        let new = self.try_resolve_ref(signature)?;
        enriched_signature(signature, new)
    }

    /// Like [`try_resolve()`][Snapshot::try_resolve()], but always returns an owned signature, which might be a copy
    /// of `signature` if no mapping was found.
    ///
    /// Note that this method will always allocate.
    pub fn resolve(&self, signature: git_actor::SignatureRef<'_>) -> git_actor::Signature {
        self.try_resolve(signature).unwrap_or_else(|| signature.to_owned())
    }
}

fn enriched_signature(
    SignatureRef { name, email, time }: SignatureRef<'_>,
    new: ResolvedSignature<'_>,
) -> Option<git_actor::Signature> {
    match (new.email, new.name) {
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
        (None, None) => unreachable!("BUG: ResolvedSignatures don't exist here when nothing is set"),
    }
}

#[cfg(test)]
mod encoded_string {
    use std::cmp::Ordering;

    use crate::snapshot::{EncodedString, EncodedStringRef};

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
