use bstr::{BStr, BString};
use quick_error::quick_error;

quick_error! {
    /// The error returned by [`name()`]
    #[derive(Debug)]
    #[allow(missing_docs)]
    pub enum NameError {
        InvalidByte(name: BString) {
            display("A ref must not contain invalid bytes or ascii control characters: '{}'", name)
        }
        DoubleDot {
            display("A ref must not contain '..' as it may be mistaken for a range")
        }
        LockFileSuffix {
            display("A ref must not end with '.lock'")
        }
        ReflogPortion {
            display("A ref must not contain '@{{' which is a part of a ref-log")
        }
        Asterisk {
            display("A ref must not contain '*' character")
        }
        StartsWithDot {
            display("A ref must not start with a '.'")
        }
        EndsWithSlash {
            display("A ref must not end with a '/'")
        }
        Empty {
            display("A ref must not be empty")
        }
    }
}

/// Assure the given `bytes` resemble a valid git ref name, which are returned unchanged on success.
pub fn name(bytes: &BStr) -> Result<&BStr, NameError> {
    if bytes.is_empty() {
        return Err(NameError::Empty);
    }

    let mut last = 0;
    for byte in bytes.iter() {
        match byte {
            b'\\' | b'^' | b':' | b'[' | b'?' | b' ' | b'~' | b'\0'..=b'\x1F' | b'\x7F' => {
                return Err(NameError::InvalidByte(bytes.into()))
            }
            b'*' => return Err(NameError::Asterisk),
            b'.' if last == b'.' => return Err(NameError::DoubleDot),
            b'{' if last == b'@' => return Err(NameError::ReflogPortion),
            _ => {}
        }
        last = *byte;
    }
    if bytes[0] == b'.' {
        return Err(NameError::StartsWithDot);
    }
    if *bytes.last().expect("non-empty") == b'/' {
        return Err(NameError::EndsWithSlash);
    }
    if bytes.ends_with(b".lock") {
        return Err(NameError::LockFileSuffix);
    }
    Ok(bytes)
}
