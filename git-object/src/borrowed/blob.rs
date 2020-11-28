use std::convert::Infallible;

/// A chunk of any [`data`][Blob::data].
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Blob<'a> {
    /// The bytes themselves
    pub data: &'a [u8],
}

impl<'a> Blob<'a> {
    /// Instantiate a `Blob` from the given `data`
    pub fn from_bytes(data: &[u8]) -> Result<Blob<'_>, Infallible> {
        Ok(Blob { data })
    }
}
