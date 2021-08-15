//!
use bstr::BStr;

use crate::Time;

/// A signature is created by an actor at a certain time.
///
/// Note that this is not a cryptographical signature.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Signature<'a> {
    /// The actor's name.
    #[cfg_attr(feature = "serde1", serde(borrow))]
    pub name: &'a BStr,
    /// The actor's email.
    pub email: &'a BStr,
    /// The time stamp at which the signature was performed.
    pub time: Time,
}

impl<'a> Signature<'a> {
    /// Deserialize a signature from the given `data`.
    pub fn from_bytes<E: nom::error::ParseError<&'a [u8]> + nom::error::ContextError<&'a [u8]>>(
        data: &'a [u8],
    ) -> Result<Signature<'a>, nom::Err<E>> {
        signature::decode(data).map(|(_, t)| t)
    }
}

///
pub mod signature;
