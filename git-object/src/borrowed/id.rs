use crate::SHA1_SIZE;
use std::convert::{TryFrom, TryInto};

/// A reference to a SHA1 identifying objects
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize))]
pub struct Id<'a>(&'a [u8; SHA1_SIZE]);

impl<'a> Id<'a> {
    pub fn encode_to_40_bytes_slice(&self, out: &mut [u8]) -> Result<(), hex::FromHexError> {
        hex::encode_to_slice(self.0, out)
    }
    pub fn first_byte(&self) -> u8 {
        self.0[0]
    }
}

impl<'a> From<&'a [u8; SHA1_SIZE]> for Id<'a> {
    fn from(v: &'a [u8; SHA1_SIZE]) -> Self {
        Id(v)
    }
}

impl<'a> TryFrom<&'a [u8]> for Id<'a> {
    type Error = std::array::TryFromSliceError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(Id(value.try_into()?))
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
            marker: serde::export::PhantomData<Id<'a>>,
            lifetime: serde::export::PhantomData<&'de ()>,
        }
        impl<'de: 'a, 'a> serde::de::Visitor<'de> for __Visitor<'de, 'a> {
            type Value = Id<'a>;
            fn expecting(&self, __formatter: &mut serde::export::Formatter) -> serde::export::fmt::Result {
                serde::export::Formatter::write_str(__formatter, "tuple struct Id")
            }
            #[inline]
            fn visit_newtype_struct<__E>(self, __e: __E) -> serde::export::Result<Self::Value, __E::Error>
            where
                __E: serde::Deserializer<'de>,
            {
                let __field0: &'a [u8] = match <&'a [u8] as serde::Deserialize>::deserialize(__e) {
                    serde::export::Ok(__val) => __val,
                    serde::export::Err(__err) => {
                        return serde::export::Err(__err);
                    }
                };
                serde::export::Ok(Id(__field0.try_into().expect("exactly 20 bytes")))
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::export::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match match serde::de::SeqAccess::next_element::<&'a [u8]>(&mut __seq) {
                    serde::export::Ok(__val) => __val,
                    serde::export::Err(__err) => {
                        return serde::export::Err(__err);
                    }
                } {
                    serde::export::Some(__value) => __value,
                    serde::export::None => {
                        return serde::export::Err(serde::de::Error::invalid_length(
                            0usize,
                            &"tuple struct Id with 1 element",
                        ));
                    }
                };
                serde::export::Ok(Id(__field0.try_into().expect("exactly 20 bytes")))
            }
        }
        serde::Deserializer::deserialize_newtype_struct(
            deserializer,
            "Id",
            __Visitor {
                marker: serde::export::PhantomData::<Id<'a>>,
                lifetime: serde::export::PhantomData,
            },
        )
    }
}
