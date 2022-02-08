use std::borrow::Cow;
use std::path::Path;

/// Convert a path to a byte sequence, which is assumed to be UTF-8 with customizations to allow recreating `OsString` instances losslessly.
///
/// Note that the byte sequences produced by this function must not be written to disk as the encoding may change over time, nor should
/// it be transferred to other platforms.
pub fn to_bytes_transient<'a>(path: impl Into<Cow<'a, Path>>) -> Cow<'a, [u8]> {
    use os_str_bytes::{OsStrBytes, OsStringBytes};
    let path = path.into();
    let path = match path {
        Cow::Owned(path) => Cow::Owned(path.into_raw_vec()),
        Cow::Borrowed(path) => path.to_raw_bytes(),
    };
    path
}

/// Methods to handle paths as bytes and do conversions between them.
pub mod bytes {
    use std::borrow::Cow;

    /// Find backslashes and replace them with slashes, which typically resembles a unix path.
    ///
    /// No other transformation is performed, the caller must check other invariants.
    pub fn backslash_to_slash<'a>(path: impl Into<Cow<'a, [u8]>>) -> Cow<'a, [u8]> {
        let path = path.into();
        if !path.contains(&b'\\') {
            return path;
        }
        let mut path = path.into_owned();
        for b in path.iter_mut().filter(|b| **b == b'\\') {
            *b = b'/';
        }
        path.into()
    }

    /// Obtain a `BStr` compatible `Cow` from one that is bytes.
    #[cfg(feature = "bstr")]
    pub fn to_bstr(path: Cow<'_, [u8]>) -> Cow<'_, bstr::BStr> {
        match path {
            Cow::Owned(p) => Cow::Owned(p.into()),
            Cow::Borrowed(p) => Cow::Borrowed(p.into()),
        }
    }
}
