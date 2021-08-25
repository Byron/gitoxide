use crate::SignatureRef;

impl<'a> SignatureRef<'a> {
    /// Deserialize a signature from the given `data`.
    pub fn from_bytes<E: nom::error::ParseError<&'a [u8]> + nom::error::ContextError<&'a [u8]>>(
        data: &'a [u8],
    ) -> Result<SignatureRef<'a>, nom::Err<E>> {
        decode(data).map(|(_, t)| t)
    }
}

///
mod decode;
pub use decode::decode;
