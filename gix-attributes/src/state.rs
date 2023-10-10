use bstr::{BStr, ByteSlice};
use kstring::{KString, KStringRef};

use crate::{State, StateRef};

/// A container to encapsulate a tightly packed and typically unallocated byte value that isn't necessarily UTF8 encoded.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Value(KString);

/// A reference container to encapsulate a tightly packed and typically unallocated byte value that isn't necessarily UTF8 encoded.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ValueRef<'a>(#[cfg_attr(feature = "serde", serde(borrow))] KStringRef<'a>);

/// Lifecycle
impl<'a> ValueRef<'a> {
    /// Keep `input` as our value.
    pub fn from_bytes(input: &'a [u8]) -> Self {
        Self(KStringRef::from_ref(
            // SAFETY: our API makes accessing that value as `str` impossible, so illformed UTF8 is never exposed as such.
            #[allow(unsafe_code)]
            unsafe {
                std::str::from_utf8_unchecked(input)
            },
        ))
    }
}

/// Access and conversions
impl ValueRef<'_> {
    /// Access this value as byte string.
    pub fn as_bstr(&self) -> &BStr {
        self.0.as_bytes().as_bstr()
    }

    /// Convert this instance into its owned form.
    pub fn to_owned(self) -> Value {
        self.into()
    }
}

impl<'a> From<&'a str> for ValueRef<'a> {
    fn from(v: &'a str) -> Self {
        ValueRef(v.into())
    }
}

impl<'a> From<ValueRef<'a>> for Value {
    fn from(v: ValueRef<'a>) -> Self {
        Value(v.0.into())
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Value(KString::from_ref(v))
    }
}

/// Access
impl Value {
    /// Return ourselves as reference.
    pub fn as_ref(&self) -> ValueRef<'_> {
        ValueRef(self.0.as_ref())
    }
}

/// Access
impl StateRef<'_> {
    /// Return `true` if the associated attribute was set to be unspecified using the `!attr` prefix or it wasn't mentioned.
    pub fn is_unspecified(&self) -> bool {
        matches!(self, StateRef::Unspecified)
    }

    /// Return `true` if the associated attribute was set with `attr`. Note that this will also be `true` if a value is assigned.
    pub fn is_set(&self) -> bool {
        matches!(self, StateRef::Set | StateRef::Value(_))
    }

    /// Return `true` if the associated attribute was set with `-attr` to specifically remove it.
    pub fn is_unset(&self) -> bool {
        matches!(self, StateRef::Unset)
    }

    /// Attempt to obtain the string value of this state, or return `None` if there is no such value.
    pub fn as_bstr(&self) -> Option<&BStr> {
        match self {
            StateRef::Value(v) => Some(v.as_bstr()),
            _ => None,
        }
    }
}

/// Initialization
impl<'a> StateRef<'a> {
    /// Keep `input` in one of our enums.
    pub fn from_bytes(input: &'a [u8]) -> Self {
        Self::Value(ValueRef::from_bytes(input))
    }
}

/// Access
impl<'a> StateRef<'a> {
    /// Turn ourselves into our owned counterpart.
    pub fn to_owned(self) -> State {
        self.into()
    }
}

impl<'a> State {
    /// Turn ourselves into our ref-type.
    pub fn as_ref(&'a self) -> StateRef<'a> {
        match self {
            State::Value(v) => StateRef::Value(v.as_ref()),
            State::Set => StateRef::Set,
            State::Unset => StateRef::Unset,
            State::Unspecified => StateRef::Unspecified,
        }
    }
}

impl<'a> From<StateRef<'a>> for State {
    fn from(s: StateRef<'a>) -> Self {
        match s {
            StateRef::Value(v) => State::Value(v.into()),
            StateRef::Set => State::Set,
            StateRef::Unset => State::Unset,
            StateRef::Unspecified => State::Unspecified,
        }
    }
}
