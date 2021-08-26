use crate::BlobRef;
use std::convert::Infallible;

impl<'a> BlobRef<'a> {
    /// Instantiate a `Blob` from the given `data`, which is used as-is.
    pub fn from_bytes(data: &[u8]) -> Result<BlobRef<'_>, Infallible> {
        Ok(BlobRef { data })
    }
}
