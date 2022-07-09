use crate::value;
use crate::Boolean;
use bstr::{BStr, BString, ByteSlice};
use std::borrow::Cow;
use std::convert::TryFrom;
use std::fmt::Display;

impl Boolean {
    /// Generates a byte representation of the value. This should be used when
    /// non-UTF-8 sequences are present or a UTF-8 representation can't be
    /// guaranteed.
    #[must_use]
    pub fn to_bstring(self) -> BString {
        self.to_string().into()
    }
}

fn bool_err(input: impl Into<BString>) -> value::Error {
    value::Error::new(
        "Booleans need to be 'no', 'off', 'false', 'zero' or 'yes', 'on', 'true', 'one'",
        input,
    )
}

impl TryFrom<&BStr> for Boolean {
    type Error = value::Error;

    fn try_from(value: &BStr) -> Result<Self, Self::Error> {
        if parse_true(value) {
            Ok(Boolean(true))
        } else if parse_false(value) {
            Ok(Boolean(false))
        } else {
            Err(bool_err(value))
        }
    }
}

impl TryFrom<BString> for Boolean {
    type Error = value::Error;

    fn try_from(value: BString) -> Result<Self, Self::Error> {
        Self::try_from(value.as_bstr())
    }
}

impl TryFrom<Cow<'_, BStr>> for Boolean {
    type Error = value::Error;
    fn try_from(c: Cow<'_, BStr>) -> Result<Self, Self::Error> {
        Self::try_from(c.as_ref())
    }
}

impl Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<Boolean> for bool {
    fn from(b: Boolean) -> Self {
        b.0
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Boolean {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bool(self.0)
    }
}

fn parse_true(value: &BStr) -> bool {
    value.eq_ignore_ascii_case(b"yes")
        || value.eq_ignore_ascii_case(b"on")
        || value.eq_ignore_ascii_case(b"true")
        || value.eq_ignore_ascii_case(b"one")
        || value.is_empty()
}

fn parse_false(value: &BStr) -> bool {
    value.eq_ignore_ascii_case(b"no")
        || value.eq_ignore_ascii_case(b"off")
        || value.eq_ignore_ascii_case(b"false")
        || value.eq_ignore_ascii_case(b"zero")
        || value == "\"\""
}
