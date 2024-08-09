///
#[allow(clippy::empty_docs)]
pub mod name {
    use std::convert::Infallible;

    /// The error used in [name()][super::name()] and [`name_partial()`][super::name_partial()]
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("A reference must be a valid tag name as well")]
        Tag(#[from] crate::tag::name::Error),
        #[error("Standalone references must be all uppercased, like 'HEAD'")]
        SomeLowercase,
        #[error("A reference name must not start with a slash '/'")]
        StartsWithSlash,
        #[error("Multiple slashes in a row are not allowed as they may change the reference's meaning")]
        RepeatedSlash,
        #[error("Path components must not start with '.'")]
        StartsWithDot,
    }

    impl From<Infallible> for Error {
        fn from(_: Infallible) -> Self {
            unreachable!("this impl is needed to allow passing a known valid partial path as parameter")
        }
    }
}

use bstr::BStr;
use std::borrow::Cow;

/// Validate a reference name running all the tests in the book. This disallows lower-case references like `lower`, but also allows
/// ones like `HEAD`, and `refs/lower`.
pub fn name(path: &BStr) -> Result<&BStr, name::Error> {
    match validate(path, Mode::Complete)? {
        Cow::Borrowed(inner) => Ok(inner),
        Cow::Owned(_) => {
            unreachable!("Without sanitization, there is no chance a sanitized version is returned.")
        }
    }
}

/// Validate a partial reference name. As it is assumed to be partial, names like `some-name` is allowed
/// even though these would be disallowed with when using [`name()`].
pub fn name_partial(path: &BStr) -> Result<&BStr, name::Error> {
    match validate(path, Mode::Partial)? {
        Cow::Borrowed(inner) => Ok(inner),
        Cow::Owned(_) => {
            unreachable!("Without sanitization, there is no chance a sanitized version is returned.")
        }
    }
}

/// The infallible version of [`name_partial()`] which instead of failing, alters `path` and returns it to be a valid
/// partial name, which would also pass [`name_partial()`].
///
/// Note that an empty `path` is replaced with a `-` in order to be valid.
pub fn name_partial_or_sanitize(path: &BStr) -> Cow<'_, BStr> {
    validate(path, Mode::PartialSanitize).expect("BUG: errors cannot happen as any issue is fixed instantly")
}

enum Mode {
    Complete,
    Partial,
    /// like Partial, but instead of failing, a sanitized version is returned.
    PartialSanitize,
}

fn validate(path: &BStr, mode: Mode) -> Result<Cow<'_, BStr>, name::Error> {
    let mut out = crate::tag::name_inner(
        path,
        match mode {
            Mode::Complete | Mode::Partial => crate::tag::Mode::Validate,
            Mode::PartialSanitize => crate::tag::Mode::Sanitize,
        },
    )?;
    let sanitize = matches!(mode, Mode::PartialSanitize);
    if path.get(0) == Some(&b'/') {
        if sanitize {
            out.to_mut()[0] = b'-';
        } else {
            return Err(name::Error::StartsWithSlash);
        }
    }
    let mut previous = 0;
    let mut saw_slash = false;
    let mut out_ofs = 0;
    for (mut byte_pos, byte) in path.iter().enumerate() {
        byte_pos -= out_ofs;
        match *byte {
            b'/' if previous == b'/' => {
                if sanitize {
                    out.to_mut().remove(byte_pos);
                    out_ofs += 1;
                } else {
                    return Err(name::Error::RepeatedSlash);
                }
            }
            b'.' if previous == b'/' => {
                if sanitize {
                    out.to_mut()[byte_pos] = b'-';
                } else {
                    return Err(name::Error::StartsWithDot);
                }
            }
            _ => {}
        }

        if *byte == b'/' {
            saw_slash = true;
        }
        previous = *byte;
    }

    if let Mode::Complete = mode {
        if !saw_slash && !path.iter().all(|c| c.is_ascii_uppercase() || *c == b'_') {
            return Err(name::Error::SomeLowercase);
        }
    }
    Ok(out)
}
