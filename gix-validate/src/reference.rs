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
    let mut previous = 0;
    let mut saw_slash = false;
    for (byte_pos, byte) in path.iter().enumerate() {
        match *byte {
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
