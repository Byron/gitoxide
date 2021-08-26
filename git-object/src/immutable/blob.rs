use std::convert::Infallible;

use crate::BlobRef;

impl<'a> BlobRef<'a> {
    /// Instantiate a `Blob` from the given `data`, which is used as-is.
    pub fn from_bytes(data: &[u8]) -> Result<BlobRef<'_>, Infallible> {
        Ok(BlobRef { data })
    }
}
