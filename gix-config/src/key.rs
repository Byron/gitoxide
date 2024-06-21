use bstr::BStr;
use bstr::ByteSlice;

/// Parse parts of a Git configuration key, like `remote.origin.url` or `core.bare`.
pub trait AsKey {
    /// Return a parsed key reference, containing all relevant parts of a key.
    /// For instance, `remote.origin.url` such key would yield access to `("remote", Some("origin"), "url")`
    /// while `user.name` would yield `("user", None, "name")`.
    ///
    /// # Panic
    ///
    /// If there is no valid `KeyRef` representation.
    fn as_key(&self) -> KeyRef<'_>;

    /// Return a parsed key reference, containing all relevant parts of a key.
    /// For instance, `remote.origin.url` such key would yield access to `("remote", Some("origin"), "url")`
    /// while `user.name` would yield `("user", None, "name")`.
    fn try_as_key(&self) -> Option<KeyRef<'_>>;
}

mod impls {
    use bstr::{BStr, BString, ByteSlice};

    use crate::key::{AsKey, KeyRef};

    impl AsKey for String {
        fn as_key(&self) -> KeyRef<'_> {
            self.try_as_key()
                .unwrap_or_else(|| panic!("'{self}' is not a valid configuration key"))
        }

        fn try_as_key(&self) -> Option<KeyRef<'_>> {
            KeyRef::parse_unvalidated(self.as_str().into())
        }
    }

    impl AsKey for &str {
        fn as_key(&self) -> KeyRef<'_> {
            self.try_as_key()
                .unwrap_or_else(|| panic!("'{self}' is not a valid configuration key"))
        }

        fn try_as_key(&self) -> Option<KeyRef<'_>> {
            KeyRef::parse_unvalidated((*self).into())
        }
    }

    impl AsKey for BString {
        fn as_key(&self) -> KeyRef<'_> {
            self.try_as_key()
                .unwrap_or_else(|| panic!("'{self}' is not a valid configuration key"))
        }

        fn try_as_key(&self) -> Option<KeyRef<'_>> {
            KeyRef::parse_unvalidated(self.as_bstr())
        }
    }

    impl AsKey for &BStr {
        fn as_key(&self) -> KeyRef<'_> {
            self.try_as_key()
                .unwrap_or_else(|| panic!("'{self}' is not a valid configuration key"))
        }

        fn try_as_key(&self) -> Option<KeyRef<'_>> {
            KeyRef::parse_unvalidated(self)
        }
    }

    impl<T> AsKey for &T
    where
        T: AsKey,
    {
        fn as_key(&self) -> KeyRef<'_> {
            (*self).as_key()
        }

        fn try_as_key(&self) -> Option<KeyRef<'_>> {
            (*self).try_as_key()
        }
    }

    impl AsKey for KeyRef<'_> {
        fn as_key(&self) -> KeyRef<'_> {
            *self
        }

        fn try_as_key(&self) -> Option<KeyRef<'_>> {
            Some(*self)
        }
    }
}

/// An unvalidated parse result of parsing input like `remote.origin.url` or `core.bare`.
#[derive(Debug, PartialEq, Ord, PartialOrd, Eq, Hash, Clone, Copy)]
pub struct KeyRef<'a> {
    /// The name of the section, like `core` in `core.bare`.
    pub section_name: &'a str,
    /// The name of the subsection, like `origin` in `remote.origin.url`.
    pub subsection_name: Option<&'a BStr>,
    /// The name of the section key, like `url` in `remote.origin.url`.
    pub value_name: &'a str,
}

/// Lifecycle
impl KeyRef<'_> {
    /// Parse `input` like `core.bare` or `remote.origin.url` as a `Key` to make its fields available,
    /// or `None` if there were not at least 2 tokens separated by `.`.
    /// Note that `input` isn't validated, and is `str` as ascii is a subset of UTF-8 which is required for any valid keys.
    pub fn parse_unvalidated(input: &BStr) -> Option<KeyRef<'_>> {
        let mut tokens = input.splitn(2, |b| *b == b'.');
        let section_name = tokens.next()?;
        let subsection_or_key = tokens.next()?;
        let mut tokens = subsection_or_key.rsplitn(2, |b| *b == b'.');
        let (subsection_name, value_name) = match (tokens.next(), tokens.next()) {
            (Some(key), Some(subsection)) => (Some(subsection.into()), key),
            (Some(key), None) => (None, key),
            (None, Some(_)) => unreachable!("iterator can't restart producing items"),
            (None, None) => return None,
        };

        Some(KeyRef {
            section_name: section_name.to_str().ok()?,
            subsection_name,
            value_name: value_name.to_str().ok()?,
        })
    }
}
