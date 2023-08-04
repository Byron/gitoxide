///
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
        #[error("Names must not be a single '.', but may contain it.")]
        SingleDot,
    }

    impl From<Infallible> for Error {
        fn from(_: Infallible) -> Self {
            unreachable!("this impl is needed to allow passing a known valid partial path as parameter")
        }
    }
}

use bstr::BStr;

/// Validate a reference name running all the tests in the book. This disallows lower-case references, but allows
/// ones like `HEAD`.
pub fn name(path: &BStr) -> Result<&BStr, name::Error> {
    validate(path, Mode::Complete)
}

/// Validate a partial reference name. As it is assumed to be partial, names like `some-name` is allowed
/// even though these would be disallowed with when using [`name()`].
pub fn name_partial(path: &BStr) -> Result<&BStr, name::Error> {
    validate(path, Mode::Partial)
}

enum Mode {
    Complete,
    Partial,
}

fn validate(path: &BStr, mode: Mode) -> Result<&BStr, name::Error> {
    crate::tag::name(path)?;
    if path[0] == b'/' {
        return Err(name::Error::StartsWithSlash);
    }
    let mut previous = 0;
    let mut one_before_previous = 0;
    let mut saw_slash = false;
    for byte in path.iter() {
        match *byte {
            b'/' if previous == b'.' && one_before_previous == b'/' => return Err(name::Error::SingleDot),
            b'/' if previous == b'/' => return Err(name::Error::RepeatedSlash),
            _ => {}
        }

        if *byte == b'/' {
            saw_slash = true;
        }
        one_before_previous = previous;
        previous = *byte;
    }

    if let Mode::Complete = mode {
        if !saw_slash && !path.iter().all(|c| c.is_ascii_uppercase() || *c == b'_') {
            return Err(name::Error::SomeLowercase);
        }
    }
    Ok(path)
}
