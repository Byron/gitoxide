use crate::value;
use bstr::{BStr, BString, ByteSlice};
use std::borrow::Cow;
use std::convert::TryFrom;
use std::fmt::Display;

/// Any value that can be interpreted as a boolean.
///
/// Note that while values can effectively be any byte string, the `git-config`
/// documentation has a strict subset of values that may be interpreted as a
/// boolean value, all of which are ASCII and thus UTF-8 representable.
/// Consequently, variants hold [`str`]s rather than [`[u8]`]s.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[allow(missing_docs)]
pub enum Boolean<'a> {
    True(TrueVariant<'a>),
    False(Cow<'a, BStr>),
}

impl Boolean<'_> {
    /// Return ourselves as plain bool.
    pub fn to_bool(&self) -> bool {
        match self {
            Boolean::True(_) => true,
            Boolean::False(_) => false,
        }
    }
    /// Generates a byte representation of the value. This should be used when
    /// non-UTF-8 sequences are present or a UTF-8 representation can't be
    /// guaranteed.
    #[must_use]
    pub fn to_bstring(&self) -> BString {
        self.into()
    }
}

fn bool_err(input: impl Into<BString>) -> value::parse::Error {
    value::parse::Error::new(
        "Booleans need to be 'no', 'off', 'false', 'zero' or 'yes', 'on', 'true', 'one'",
        input,
    )
}

impl<'a> TryFrom<&'a BStr> for Boolean<'a> {
    type Error = value::parse::Error;

    fn try_from(value: &'a BStr) -> Result<Self, Self::Error> {
        if let Ok(v) = TrueVariant::try_from(value) {
            return Ok(Self::True(v));
        }

        if value.eq_ignore_ascii_case(b"no")
            || value.eq_ignore_ascii_case(b"off")
            || value.eq_ignore_ascii_case(b"false")
            || value.eq_ignore_ascii_case(b"zero")
            || value == "\"\""
        {
            return Ok(Self::False(value.as_bstr().into()));
        }

        Err(bool_err(value))
    }
}

impl TryFrom<BString> for Boolean<'_> {
    type Error = value::parse::Error;

    fn try_from(value: BString) -> Result<Self, Self::Error> {
        if value.eq_ignore_ascii_case(b"no")
            || value.eq_ignore_ascii_case(b"off")
            || value.eq_ignore_ascii_case(b"false")
            || value.eq_ignore_ascii_case(b"zero")
            || value == "\"\""
        {
            return Ok(Self::False(Cow::Owned(value)));
        }

        TrueVariant::try_from(value).map(Self::True)
    }
}

impl<'a> TryFrom<Cow<'a, BStr>> for Boolean<'a> {
    type Error = value::parse::Error;
    fn try_from(c: Cow<'a, BStr>) -> Result<Self, Self::Error> {
        match c {
            Cow::Borrowed(c) => Self::try_from(c),
            Cow::Owned(c) => Self::try_from(c),
        }
    }
}

impl Display for Boolean<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Boolean::True(v) => v.fmt(f),
            Boolean::False(v) => write!(f, "{}", v),
        }
    }
}

impl From<Boolean<'_>> for bool {
    fn from(b: Boolean<'_>) -> Self {
        match b {
            Boolean::True(_) => true,
            Boolean::False(_) => false,
        }
    }
}

impl<'a, 'b: 'a> From<&'b Boolean<'a>> for &'a BStr {
    fn from(b: &'b Boolean<'_>) -> Self {
        match b {
            Boolean::True(t) => t.into(),
            Boolean::False(f) => f.as_ref(),
        }
    }
}

impl From<Boolean<'_>> for BString {
    fn from(b: Boolean<'_>) -> Self {
        b.into()
    }
}

impl From<&Boolean<'_>> for BString {
    fn from(b: &Boolean<'_>) -> Self {
        b.to_string().into()
    }
}

#[cfg(feature = "serde")]
impl Serialize for Boolean<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Boolean::True(_) => serializer.serialize_bool(true),
            Boolean::False(_) => serializer.serialize_bool(false),
        }
    }
}

/// Discriminating enum between implicit and explicit truthy values.
///
/// This enum is part of the [`Boolean`] struct.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[allow(missing_docs)]
pub enum TrueVariant<'a> {
    Explicit(Cow<'a, BStr>),
    /// For values defined without a `= <value>`.
    Implicit,
}

impl<'a> TryFrom<&'a BStr> for TrueVariant<'a> {
    type Error = value::parse::Error;

    fn try_from(value: &'a BStr) -> Result<Self, Self::Error> {
        if value.eq_ignore_ascii_case(b"yes")
            || value.eq_ignore_ascii_case(b"on")
            || value.eq_ignore_ascii_case(b"true")
            || value.eq_ignore_ascii_case(b"one")
        {
            Ok(Self::Explicit(value.as_bstr().into()))
        } else if value.is_empty() {
            Ok(Self::Implicit)
        } else {
            Err(bool_err(value))
        }
    }
}

impl TryFrom<BString> for TrueVariant<'_> {
    type Error = value::parse::Error;

    fn try_from(value: BString) -> Result<Self, Self::Error> {
        if value.eq_ignore_ascii_case(b"yes")
            || value.eq_ignore_ascii_case(b"on")
            || value.eq_ignore_ascii_case(b"true")
            || value.eq_ignore_ascii_case(b"one")
        {
            Ok(Self::Explicit(Cow::Owned(value)))
        } else if value.is_empty() {
            Ok(Self::Implicit)
        } else {
            Err(bool_err(value))
        }
    }
}

impl Display for TrueVariant<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Self::Explicit(v) = self {
            write!(f, "{}", v)
        } else {
            Ok(())
        }
    }
}

impl<'a, 'b: 'a> From<&'b TrueVariant<'a>> for &'a BStr {
    fn from(t: &'b TrueVariant<'a>) -> Self {
        match t {
            TrueVariant::Explicit(e) => e.as_ref(),
            TrueVariant::Implicit => "".into(),
        }
    }
}

#[cfg(feature = "serde")]
impl Serialize for TrueVariant<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bool(true)
    }
}
