use bstr::BStr;

///
pub mod name {
    use bstr::BString;

    /// The error returned by [`name()`][super::name()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("A ref must not contain invalid bytes or ascii control characters: {byte:?}")]
        InvalidByte { byte: BString },
        #[error("A ref must not contain '..' as it may be mistaken for a range")]
        DoubleDot,
        #[error("A ref must not end with '.lock'")]
        LockFileSuffix,
        #[error("A ref must not contain '@{{' which is a part of a ref-log")]
        ReflogPortion,
        #[error("A ref must not contain '*' character")]
        Asterisk,
        #[error("A ref must not start with a '.'")]
        StartsWithDot,
        #[error("A ref must not end with a '/'")]
        EndsWithSlash,
        #[error("A ref must not be empty")]
        Empty,
    }
}

/// Assure the given `bytes` resemble a valid git ref name, which are returned unchanged on success.
pub fn name(bytes: &BStr) -> Result<&BStr, name::Error> {
    if bytes.is_empty() {
        return Err(name::Error::Empty);
    }
    if *bytes.last().expect("non-empty") == b'/' {
        return Err(name::Error::EndsWithSlash);
    }

    let mut previous = 0;
    for byte in bytes.iter() {
        match byte {
            b'\\' | b'^' | b':' | b'[' | b'?' | b' ' | b'~' | b'\0'..=b'\x1F' | b'\x7F' => {
                return Err(name::Error::InvalidByte {
                    byte: (&[*byte][..]).into(),
                })
            }
            b'*' => return Err(name::Error::Asterisk),
            b'.' if previous == b'.' => return Err(name::Error::DoubleDot),
            b'{' if previous == b'@' => return Err(name::Error::ReflogPortion),
            _ => {}
        }
        previous = *byte;
    }
    if bytes[0] == b'.' {
        return Err(name::Error::StartsWithDot);
    }
    if bytes.ends_with(b".lock") {
        return Err(name::Error::LockFileSuffix);
    }
    Ok(bytes)
}
