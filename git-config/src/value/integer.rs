use crate::value;
use bstr::{BStr, BString};
use std::borrow::Cow;
use std::convert::TryFrom;
use std::fmt::Display;
use std::str::FromStr;

/// Any value that can be interpreted as an integer.
///
/// This supports any numeric value that can fit in a [`i64`], excluding the
/// suffix. The suffix is parsed separately from the value itself, so if you
/// wish to obtain the true value of the integer, you must account for the
/// suffix after fetching the value. [`IntegerSuffix`] provides
/// [`bitwise_offset`] to help with the math, but do be warned that if the value
/// is very large, you may run into overflows.
///
/// [`BStr`]: bstr::BStr
/// [`bitwise_offset`]: IntegerSuffix::bitwise_offset
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Integer {
    /// The value, without any suffix modification
    pub value: i64,
    /// A provided suffix, if any.
    pub suffix: Option<IntegerSuffix>,
}

impl Integer {
    /// Generates a byte representation of the value. This should be used when
    /// non-UTF-8 sequences are present or a UTF-8 representation can't be
    /// guaranteed.
    #[must_use]
    pub fn to_bstring(self) -> BString {
        self.into()
    }

    /// Canonicalize values as simple decimal numbers.
    /// An optional suffix of k, m, or g (case-insensitive), upon creation, will cause the value to be multiplied by
    /// 1024 (k), 1048576 (m), or 1073741824 (g) respectively.
    ///
    /// Returns the result if no multiplication overflow.
    pub fn to_decimal(&self) -> Option<i64> {
        match self.suffix {
            None => Some(self.value),
            Some(suffix) => match suffix {
                IntegerSuffix::Kibi => self.value.checked_mul(1024),
                IntegerSuffix::Mebi => self.value.checked_mul(1024 * 1024),
                IntegerSuffix::Gibi => self.value.checked_mul(1024 * 1024 * 1024),
            },
        }
    }
}

impl Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)?;
        if let Some(suffix) = self.suffix {
            write!(f, "{}", suffix)
        } else {
            Ok(())
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Integer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(suffix) = self.suffix {
            serializer.serialize_i64(self.value << suffix.bitwise_offset())
        } else {
            serializer.serialize_i64(self.value)
        }
    }
}

fn int_err(input: impl Into<BString>) -> value::parse::Error {
    value::parse::Error::new(
        "Integers needs to be positive or negative numbers which may have a suffix like 1k, 42, or 50G",
        input,
    )
}

impl TryFrom<&BStr> for Integer {
    type Error = value::parse::Error;

    fn try_from(s: &BStr) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(s).map_err(|err| int_err(s).with_err(err))?;
        if let Ok(value) = s.parse() {
            return Ok(Self { value, suffix: None });
        }

        // Assume we have a prefix at this point.

        if s.len() <= 1 {
            return Err(int_err(s));
        }

        let (number, suffix) = s.split_at(s.len() - 1);
        if let (Ok(value), Ok(suffix)) = (number.parse(), suffix.parse()) {
            Ok(Self {
                value,
                suffix: Some(suffix),
            })
        } else {
            Err(int_err(s))
        }
    }
}

impl TryFrom<BString> for Integer {
    type Error = value::parse::Error;

    fn try_from(value: BString) -> Result<Self, Self::Error> {
        Self::try_from(value.as_ref())
    }
}

impl TryFrom<Cow<'_, BStr>> for Integer {
    type Error = value::parse::Error;

    fn try_from(c: Cow<'_, BStr>) -> Result<Self, Self::Error> {
        match c {
            Cow::Borrowed(c) => Self::try_from(c),
            Cow::Owned(c) => Self::try_from(c),
        }
    }
}

impl From<Integer> for BString {
    fn from(i: Integer) -> Self {
        i.into()
    }
}

impl From<&Integer> for BString {
    fn from(i: &Integer) -> Self {
        i.to_string().into()
    }
}

/// Integer prefixes that are supported by `git-config`.
///
/// These values are base-2 unit of measurements, not the base-10 variants.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[allow(missing_docs)]
pub enum IntegerSuffix {
    Kibi,
    Mebi,
    Gibi,
}

impl IntegerSuffix {
    /// Returns the number of bits that the suffix shifts left by.
    #[must_use]
    pub const fn bitwise_offset(self) -> usize {
        match self {
            Self::Kibi => 10,
            Self::Mebi => 20,
            Self::Gibi => 30,
        }
    }
}

impl Display for IntegerSuffix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Kibi => write!(f, "k"),
            Self::Mebi => write!(f, "m"),
            Self::Gibi => write!(f, "g"),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for IntegerSuffix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match self {
            Self::Kibi => "k",
            Self::Mebi => "m",
            Self::Gibi => "g",
        })
    }
}

impl FromStr for IntegerSuffix {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "k" | "K" => Ok(Self::Kibi),
            "m" | "M" => Ok(Self::Mebi),
            "g" | "G" => Ok(Self::Gibi),
            _ => Err(()),
        }
    }
}

impl TryFrom<&[u8]> for IntegerSuffix {
    type Error = ();

    fn try_from(s: &[u8]) -> Result<Self, Self::Error> {
        Self::from_str(std::str::from_utf8(s).map_err(|_| ())?)
    }
}

impl TryFrom<BString> for IntegerSuffix {
    type Error = ();

    fn try_from(value: BString) -> Result<Self, Self::Error> {
        Self::try_from(value.as_ref())
    }
}
