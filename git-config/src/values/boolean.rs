use std::{borrow::Cow, convert::TryFrom, fmt::Display};

use bstr::{BStr, BString, ByteSlice};

use crate::{value, Boolean};

fn bool_err(input: impl Into<BString>) -> value::Error {
    value::Error::new(
        "Booleans need to be 'no', 'off', 'false', '' or 'yes', 'on', 'true' or any number",
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
            use std::str::FromStr;
            if let Some(integer) = value.to_str().ok().and_then(|s| i64::from_str(s).ok()) {
                Ok(Boolean(integer != 0))
            } else {
                Err(bool_err(value))
            }
        }
    }
}

impl Boolean {
    /// Return true if the boolean is a true value.
    ///
    /// Note that the inner value is accessible directly as well.
    pub fn is_true(self) -> bool {
        self.0
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
    value.eq_ignore_ascii_case(b"yes") || value.eq_ignore_ascii_case(b"on") || value.eq_ignore_ascii_case(b"true")
}

fn parse_false(value: &BStr) -> bool {
    value.eq_ignore_ascii_case(b"no")
        || value.eq_ignore_ascii_case(b"off")
        || value.eq_ignore_ascii_case(b"false")
        || value.is_empty()
}
