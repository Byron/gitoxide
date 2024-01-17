use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::Path;

/// Assure that `s` is precomposed, i.e. `ä` is a single code-point, and not two i.e. `a` and `<umlaut>`.
///
/// At the expense of extra-compute, it does nothing if there is no work to be done, returning the original input without allocating.
pub fn precompose(s: Cow<'_, str>) -> Cow<'_, str> {
    use unicode_normalization::UnicodeNormalization;
    if s.as_ref().nfc().cmp(s.as_ref().chars()).is_eq() {
        s
    } else {
        Cow::Owned(s.as_ref().nfc().collect())
    }
}

/// Assure that `s` is decomposed, i.e. `ä` turns into `a` and `<umlaut>`.
///
/// At the expense of extra-compute, it does nothing if there is no work to be done, returning the original input without allocating.
pub fn decompose(s: Cow<'_, str>) -> Cow<'_, str> {
    use unicode_normalization::UnicodeNormalization;
    if s.as_ref().nfd().cmp(s.as_ref().chars()).is_eq() {
        s
    } else {
        Cow::Owned(s.as_ref().nfd().collect())
    }
}

/// Return the precomposed version of `path`, or `path` itself if it contained illformed unicode,
/// or if the unicode version didn't contains decomposed unicode.
/// Otherwise, similar to [`precompose()`]
pub fn precompose_path(path: Cow<'_, Path>) -> Cow<'_, Path> {
    match path.to_str() {
        None => path,
        Some(maybe_decomposed) => match precompose(maybe_decomposed.into()) {
            Cow::Borrowed(_) => path,
            Cow::Owned(precomposed) => Cow::Owned(precomposed.into()),
        },
    }
}

/// Return the precomposed version of `name`, or `name` itself if it contained illformed unicode,
/// or if the unicode version didn't contains decomposed unicode.
/// Otherwise, similar to [`precompose()`]
pub fn precompose_os_string(path: Cow<'_, OsStr>) -> Cow<'_, OsStr> {
    match path.to_str() {
        None => path,
        Some(maybe_decomposed) => match precompose(maybe_decomposed.into()) {
            Cow::Borrowed(_) => path,
            Cow::Owned(precomposed) => Cow::Owned(precomposed.into()),
        },
    }
}
