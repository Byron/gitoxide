use crate::loose;

/// Returned by [`loose::Object::decode()`] and [`loose::Object::stream()`]
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    Access(#[from] loose::object::access::Error),
    #[error(transparent)]
    Parse(#[from] git_object::borrowed::Error),
}

// Decoding and streaming
impl loose::Object {
    /// Decode the object to make it's fields accessible in case of Trees, Tags and Commits.
    ///
    /// This is a zero-copy operation with data read from disk if needed and stored in memory.
    /// The returned [`borrowed::Object`] references this data where possible.
    ///
    /// **Note**: Blobs are also loaded into memory and are made available that way.
    /// Consider using `stream()` if large Blobs are expected.
    pub fn decode(&mut self) -> Result<git_object::borrowed::Object<'_>, Error> {
        self.decompress_all()?;
        let bytes = &self.decompressed_data[self.header_size..];
        Ok(git_object::borrowed::Object::from_bytes(self.kind, bytes)?)
    }
}
