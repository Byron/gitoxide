use bstr::{BStr, ByteSlice};
use std::borrow::Cow;

///
#[allow(clippy::empty_docs)]
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
        #[error("A ref must not end with a '.'")]
        EndsWithDot,
        #[error("A ref must not end with a '/'")]
        EndsWithSlash,
        #[error("A ref must not be empty")]
        Empty,
    }
}

/// Assure the given `input` resemble a valid git tag name, which is returned unchanged on success.
/// Tag names are provided as names, lik` v1.0` or `alpha-1`, without paths.
pub fn name(input: &BStr) -> Result<&BStr, name::Error> {
    match name_inner(input, Mode::Validate)? {
        Cow::Borrowed(inner) => Ok(inner),
        Cow::Owned(_) => {
            unreachable!("When validating, the input isn't changed")
        }
    }
}

#[derive(Eq, PartialEq)]
pub(crate) enum Mode {
    Sanitize,
    Validate,
}

pub(crate) fn name_inner(input: &BStr, mode: Mode) -> Result<Cow<'_, BStr>, name::Error> {
    let mut out = Cow::Borrowed(input);
    let sanitize = matches!(mode, Mode::Sanitize);
    if input.is_empty() {
        return if sanitize {
            out.to_mut().push(b'-');
            Ok(out)
        } else {
            Err(name::Error::Empty)
        };
    }
    if *input.last().expect("non-empty") == b'/' {
        if sanitize {
            while out.last() == Some(&b'/') {
                out.to_mut().pop();
            }
            let bytes_from_end = out.to_mut().as_bytes_mut().iter_mut().rev();
            for b in bytes_from_end.take_while(|b| **b == b'/') {
                *b = b'-';
            }
        } else {
            return Err(name::Error::EndsWithSlash);
        }
    }

    let mut previous = 0;
    let mut out_ofs = 0;
    for (mut byte_pos, byte) in input.iter().enumerate() {
        byte_pos -= out_ofs;
        match byte {
            b'\\' | b'^' | b':' | b'[' | b'?' | b' ' | b'~' | b'\0'..=b'\x1F' | b'\x7F' => {
                if sanitize {
                    out.to_mut()[byte_pos] = b'-';
                } else {
                    return Err(name::Error::InvalidByte {
                        byte: (&[*byte][..]).into(),
                    });
                }
            }
            b'*' => {
                if sanitize {
                    out.to_mut()[byte_pos] = b'-';
                } else {
                    return Err(name::Error::Asterisk);
                }
            }

            b'.' if previous == b'.' => {
                if sanitize {
                    out.to_mut().remove(byte_pos);
                    out_ofs += 1;
                } else {
                    return Err(name::Error::DoubleDot);
                }
            }
            b'{' if previous == b'@' => {
                if sanitize {
                    out.to_mut()[byte_pos] = b'-';
                } else {
                    return Err(name::Error::ReflogPortion);
                }
            }
            _ => {}
        }
        previous = *byte;
    }
    if input[0] == b'.' {
        if sanitize {
            out.to_mut()[0] = b'-';
        } else {
            return Err(name::Error::StartsWithDot);
        }
    }
    if input[input.len() - 1] == b'.' {
        if sanitize {
            let last = out.len() - 1;
            out.to_mut()[last] = b'-';
        } else {
            return Err(name::Error::EndsWithDot);
        }
    }
    if input.ends_with(b".lock") {
        if sanitize {
            while out.ends_with(b".lock") {
                let len_without_suffix = out.len() - b".lock".len();
                out.to_mut().truncate(len_without_suffix);
            }
        } else {
            return Err(name::Error::LockFileSuffix);
        }
    }
    Ok(out)
}
