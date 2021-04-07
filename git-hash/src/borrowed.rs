use crate::SIZE_OF_SHA1_DIGEST;
use bstr::ByteSlice;
use std::{
    convert::{TryFrom, TryInto},
    fmt,
};

/// A borrowed reference to a hash identifying objects.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize))]
pub struct Id<'a>(&'a [u8; SIZE_OF_SHA1_DIGEST]);

/// A borrowed reference to a hash identifying objects.
#[derive(Hash)]
#[repr(transparent)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize))]
pub struct Id2 {
    bytes: [u8],
}

use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        UnknownDigestLength(length: usize) {
            display("Cannot instantiate git hash from a digest of length {}", length)
        }
    }
}

impl Id2 {
    pub fn try_from(value: &[u8]) -> Result<&Self, Error> {
        match value.len() {
            20 => Ok(
                #[allow(unsafe_code)]
                unsafe {
                    &*(value as *const [u8] as *const Id2)
                },
            ),
            len => Err(Error::UnknownDigestLength(len)),
        }
    }
}

/// Access
impl Id2 {
    /// The kind of hash used for this Digest
    pub fn kind(&self) -> crate::Kind {
        match self.bytes.len() {
            20 => crate::Kind::Sha1,
            _ => unreachable!("creating this instance is checked and fails on unknown lengths"),
        }
    }

    /// The first byte of the hash, commonly used to partition a set of `Id`s
    pub fn first_byte(&self) -> u8 {
        self.bytes[0]
    }
}

impl AsRef<Id2> for &Id2 {
    fn as_ref(&self) -> &Id2 {
        self
    }
}

impl ToOwned for Id2 {
    type Owned = crate::Id;

    fn to_owned(&self) -> Self::Owned {
        match self.kind() {
            crate::Kind::Sha1 => crate::Id(self.bytes.try_into().expect("no bug in hash detection")),
        }
    }
}

/// Access
impl<'a> Id<'a> {
    /// The kind of hash used for this Digest
    pub fn kind(&self) -> crate::Kind {
        crate::Kind::Sha1
    }
    /// The first byte of the hash, commonly used to partition a set of `Id`s
    pub fn first_byte(&self) -> u8 {
        self.0[0]
    }
}

/// Sha1 specific methods
impl<'a> Id<'a> {
    /// Returns an array with a hexadecimal encoded version of the Sha1 hash this `Id` represents.
    ///
    /// **Panics** if this is not a Sha1 hash, as identifiable by [`Id::kind()`].
    pub fn to_sha1_hex(&self) -> [u8; SIZE_OF_SHA1_DIGEST * 2] {
        let mut buf = [0u8; SIZE_OF_SHA1_DIGEST * 2];
        hex::encode_to_slice(self.0, &mut buf).expect("to count correctly");
        buf
    }

    /// Returns the bytes making up the Sha1.
    ///
    /// **Panics** if this is not a Sha1 hash, as identifiable by [`Id::kind()`].
    pub fn sha1(&self) -> &[u8; SIZE_OF_SHA1_DIGEST] {
        self.0
    }

    /// Returns a Sha1 digest with all bytes being initialized to zero.
    pub fn null_sha1() -> Self {
        Id(&[0u8; SIZE_OF_SHA1_DIGEST])
    }
}

impl<'a> From<&'a [u8; SIZE_OF_SHA1_DIGEST]> for Id<'a> {
    fn from(v: &'a [u8; SIZE_OF_SHA1_DIGEST]) -> Self {
        Id(v)
    }
}

impl<'a> TryFrom<&'a [u8]> for Id<'a> {
    type Error = std::array::TryFromSliceError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(Id(value.try_into()?))
    }
}

impl fmt::Display for Id<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.to_sha1_hex().as_bstr())
    }
}

/// Manually created from a version that uses a slice, and we forcefully try to convert it into a borrowed array of the desired size
/// Could be improved by fitting this into serde
/// Unfortunately the serde::Deserialize derive wouldn't work for borrowed arrays.
#[cfg(feature = "serde1")]
impl<'de: 'a, 'a> serde::Deserialize<'de> for Id<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as serde::Deserializer<'de>>::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct __Visitor<'de: 'a, 'a> {
            marker: std::marker::PhantomData<Id<'a>>,
            lifetime: std::marker::PhantomData<&'de ()>,
        }
        impl<'de: 'a, 'a> serde::de::Visitor<'de> for __Visitor<'de, 'a> {
            type Value = Id<'a>;
            fn expecting(&self, __formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Formatter::write_str(__formatter, "tuple struct Digest")
            }
            #[inline]
            fn visit_newtype_struct<__E>(self, __e: __E) -> std::result::Result<Self::Value, __E::Error>
            where
                __E: serde::Deserializer<'de>,
            {
                let __field0: &'a [u8] = match <&'a [u8] as serde::Deserialize>::deserialize(__e) {
                    Ok(__val) => __val,
                    Err(__err) => {
                        return Err(__err);
                    }
                };
                Ok(Id(__field0.try_into().expect("exactly 20 bytes")))
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> std::result::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match match serde::de::SeqAccess::next_element::<&'a [u8]>(&mut __seq) {
                    Ok(__val) => __val,
                    Err(__err) => {
                        return Err(__err);
                    }
                } {
                    Some(__value) => __value,
                    None => {
                        return Err(serde::de::Error::invalid_length(
                            0usize,
                            &"tuple struct Digest with 1 element",
                        ));
                    }
                };
                Ok(Id(__field0.try_into().expect("exactly 20 bytes")))
            }
        }
        serde::Deserializer::deserialize_newtype_struct(
            deserializer,
            "Digest",
            __Visitor {
                marker: std::marker::PhantomData::<Id<'a>>,
                lifetime: std::marker::PhantomData,
            },
        )
    }
}

/// Manually created from a version that uses a slice, and we forcefully try to convert it into a borrowed array of the desired size
/// Could be improved by fitting this into serde
/// Unfortunately the serde::Deserialize derive wouldn't work for borrowed arrays.
#[cfg(feature = "serde1")]
impl<'de: 'a, 'a> serde::Deserialize<'de> for &'a Id2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as serde::Deserializer<'de>>::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct __Visitor<'de: 'a, 'a> {
            marker: std::marker::PhantomData<&'a Id2>,
            lifetime: std::marker::PhantomData<&'de ()>,
        }
        impl<'de: 'a, 'a> serde::de::Visitor<'de> for __Visitor<'de, 'a> {
            type Value = &'a Id2;
            fn expecting(&self, __formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Formatter::write_str(__formatter, "tuple struct Digest")
            }
            #[inline]
            fn visit_newtype_struct<__E>(self, __e: __E) -> std::result::Result<Self::Value, __E::Error>
            where
                __E: serde::Deserializer<'de>,
            {
                let __field0: &'a [u8] = match <&'a [u8] as serde::Deserialize>::deserialize(__e) {
                    Ok(__val) => __val,
                    Err(__err) => {
                        return Err(__err);
                    }
                };
                Ok(Id2::try_from(__field0).expect("exactly 20 bytes"))
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> std::result::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match match serde::de::SeqAccess::next_element::<&'a [u8]>(&mut __seq) {
                    Ok(__val) => __val,
                    Err(__err) => {
                        return Err(__err);
                    }
                } {
                    Some(__value) => __value,
                    None => {
                        return Err(serde::de::Error::invalid_length(
                            0usize,
                            &"tuple struct Digest with 1 element",
                        ));
                    }
                };
                Ok(Id2::try_from(__field0).expect("exactly 20 bytes"))
            }
        }
        serde::Deserializer::deserialize_newtype_struct(
            deserializer,
            "Digest",
            __Visitor {
                marker: std::marker::PhantomData::<&'a Id2>,
                lifetime: std::marker::PhantomData,
            },
        )
    }
}
