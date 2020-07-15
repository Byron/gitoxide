use bstr::{BStr, BString};
use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    pub enum NameError {
        InvalidByte(name: BString) {
            display("A ref must not contain invalid bytes or ascii control characters: '{}'", name)
        }
        DoubleDot {
            description("A ref must not contain '..' as it may be mistaken for a range")
        }
        LockFileSuffix {
            description("A ref must not end with '.lock'")
        }
        ReflogPortion {
            description("A ref must not contain '@{' which is a part of a ref-log")
        }
        Asterisk {
            description("A ref must not contain '*' character")
        }
        StartsWithDot {
            description("A ref must not start with a '.'")
        }
        EndsWithSlash {
            description("A ref must not end with a '/'")
        }
        Empty {
            description("A ref must not be empty")
        }
    }
}

pub fn name(name: &BStr) -> Result<&BStr, NameError> {
    if name.is_empty() {
        return Err(NameError::Empty);
    }

    let mut last = 0;
    for byte in name.iter() {
        match byte {
            b'\\' | b'^' | b':' | b'[' | b'?' | b' ' | b'~' | b'\0'..=b'\x1F' | b'\x7F' => {
                return Err(NameError::InvalidByte(name.into()))
            }
            b'*' => return Err(NameError::Asterisk),
            b'.' if last == b'.' => return Err(NameError::DoubleDot),
            b'{' if last == b'@' => return Err(NameError::ReflogPortion),
            _ => {}
        }
        last = *byte;
    }
    if name[0] == b'.' {
        return Err(NameError::StartsWithDot);
    }
    if *name.last().expect("non-empty") == b'/' {
        return Err(NameError::EndsWithSlash);
    }
    if name.ends_with(b".lock") {
        return Err(NameError::LockFileSuffix);
    }
    Ok(name)
}

#[cfg(test)]
mod tests;
