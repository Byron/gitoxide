use crate::Blob;
use crate::BlobRef;

use std::convert::Infallible;
use std::io;

impl Blob {
    /// Write the blobs data to `out` verbatim.
    pub fn write_to(&self, mut out: impl io::Write) -> io::Result<()> {
        out.write_all(&self.data)
    }
}

impl<'a> BlobRef<'a> {
    /// Instantiate a `Blob` from the given `data`, which is used as-is.
    pub fn from_bytes(data: &[u8]) -> Result<BlobRef<'_>, Infallible> {
        Ok(BlobRef { data })
    }
}
