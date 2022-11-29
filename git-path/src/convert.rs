use std::{
    borrow::Cow,
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
};

use bstr::{BStr, BString};

#[derive(Debug)]
/// The error type returned by [`into_bstr()`] and others may suffer from failed conversions from or to bytes.
pub struct Utf8Error;

impl std::fmt::Display for Utf8Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Could not convert to UTF8 or from UTF8 due to ill-formed input")
    }
}

impl std::error::Error for Utf8Error {}

/// Like [`into_bstr()`], but takes `OsStr` as input for a lossless, but fallible, conversion.
pub fn os_str_into_bstr(path: &OsStr) -> Result<&BStr, Utf8Error> {
    let path = try_into_bstr(Cow::Borrowed(path.as_ref()))?;
    match path {
        Cow::Borrowed(path) => Ok(path),
        Cow::Owned(_) => unreachable!("borrowed cows stay borrowed"),
    }
}

/// Like [`into_bstr()`], but takes `OsString` as input for a lossless, but fallible, conversion.
pub fn os_string_into_bstring(path: OsString) -> Result<BString, Utf8Error> {
    let path = try_into_bstr(Cow::Owned(path.into()))?;
    match path {
        Cow::Borrowed(_path) => unreachable!("borrowed cows stay borrowed"),
        Cow::Owned(path) => Ok(path),
    }
}

/// Convert the given path either into its raw bytes on unix or its UTF8 encoded counterpart on windows.
///
/// On windows, if the source Path contains ill-formed, lone surrogates, the UTF-8 conversion will fail
/// causing `Utf8Error` to be returned.
pub fn try_into_bstr<'a>(path: impl Into<Cow<'a, Path>>) -> Result<Cow<'a, BStr>, Utf8Error> {
    let path = path.into();
    let path_str = match path {
        Cow::Owned(path) => Cow::Owned({
            #[cfg(unix)]
            let p: BString = {
                use std::os::unix::ffi::OsStringExt;
                path.into_os_string().into_vec().into()
            };
            #[cfg(target_os = "wasi")]
            let p: BString = {
                use std::os::wasi::ffi::OsStringExt;
                path.into_os_string().into_vec().into()
            };
            #[cfg(not(unix))]
            let p: BString = path.into_os_string().into_string().map_err(|_| Utf8Error)?.into();
            p
        }),
        Cow::Borrowed(path) => Cow::Borrowed({
            #[cfg(unix)]
            let p: &BStr = {
                use std::os::unix::ffi::OsStrExt;
                path.as_os_str().as_bytes().into()
            };
            #[cfg(target_os = "wasi")]
            let p: &BStr = {
                use std::os::wasi::ffi::OsStrExt;
                path.as_os_str().as_bytes().into()
            };
            #[cfg(not(unix))]
            let p: &BStr = path.to_str().ok_or(Utf8Error)?.as_bytes().into();
            p
        }),
    };
    Ok(path_str)
}

/// Similar to [`try_into_bstr()`] but **panics** if malformed surrogates are encountered on windows.
pub fn into_bstr<'a>(path: impl Into<Cow<'a, Path>>) -> Cow<'a, BStr> {
    try_into_bstr(path).expect("prefix path doesn't contain ill-formed UTF-8")
}

/// Given `input` bytes, produce a `Path` from them ignoring encoding entirely if on unix.
///
/// On windows, the input is required to be valid UTF-8, which is guaranteed if we wrote it before. There are some potential
/// git versions and windows installation which produce mal-formed UTF-16 if certain emojies are in the path. It's as rare as
/// it sounds, but possible.
pub fn try_from_byte_slice(input: &[u8]) -> Result<&Path, Utf8Error> {
    #[cfg(unix)]
    let p = {
        use std::os::unix::ffi::OsStrExt;
        OsStr::from_bytes(input).as_ref()
    };
    #[cfg(target_os = "wasi")]
    let p = {
        use std::os::wasi::ffi::OsStrExt;
        OsStr::from_bytes(input).as_ref()
    };
    #[cfg(not(unix))]
    let p = Path::new(std::str::from_utf8(input).map_err(|_| Utf8Error)?);
    Ok(p)
}

/// Similar to [`from_byte_slice()`], but takes either borrowed or owned `input`.
pub fn try_from_bstr<'a>(input: impl Into<Cow<'a, BStr>>) -> Result<Cow<'a, Path>, Utf8Error> {
    let input = input.into();
    match input {
        Cow::Borrowed(input) => try_from_byte_slice(input).map(Cow::Borrowed),
        Cow::Owned(input) => try_from_bstring(input).map(Cow::Owned),
    }
}

/// Similar to [`try_from_bstr()`], but **panics** if malformed surrogates are encountered on windows.
pub fn from_bstr<'a>(input: impl Into<Cow<'a, BStr>>) -> Cow<'a, Path> {
    try_from_bstr(input).expect("prefix path doesn't contain ill-formed UTF-8")
}

/// Similar to [`try_from_bstr()`], but takes and produces owned data.
pub fn try_from_bstring(input: impl Into<BString>) -> Result<PathBuf, Utf8Error> {
    let input = input.into();
    #[cfg(unix)]
    let p = {
        use std::os::unix::ffi::OsStringExt;
        std::ffi::OsString::from_vec(input.into()).into()
    };
    #[cfg(target_os = "wasi")]
    let p = {
        use std::os::wasi::ffi::OsStringExt;
        std::ffi::OsString::from_vec(input.into()).into()
    };
    #[cfg(not(unix))]
    let p = {
        use bstr::ByteVec;
        PathBuf::from(
            {
                let v: Vec<_> = input.into();
                v
            }
            .into_string()
            .map_err(|_| Utf8Error)?,
        )
    };
    Ok(p)
}

/// Similar to [`try_from_bstring()`], but will **panic** if there is ill-formed UTF-8 in the `input`.
pub fn from_bstring(input: impl Into<BString>) -> PathBuf {
    try_from_bstring(input).expect("well-formed UTF-8 on windows")
}

/// Similar to [`try_from_byte_slice()`], but will **panic** if there is ill-formed UTF-8 in the `input`.
pub fn from_byte_slice(input: &[u8]) -> &Path {
    try_from_byte_slice(input).expect("well-formed UTF-8 on windows")
}

fn replace<'a>(path: impl Into<Cow<'a, BStr>>, find: u8, replace: u8) -> Cow<'a, BStr> {
    let path = path.into();
    match path {
        Cow::Owned(mut path) => {
            for b in path.iter_mut().filter(|b| **b == find) {
                *b = replace;
            }
            path.into()
        }
        Cow::Borrowed(path) => {
            if !path.contains(&find) {
                return path.into();
            }
            let mut path = path.to_owned();
            for b in path.iter_mut().filter(|b| **b == find) {
                *b = replace;
            }
            path.into()
        }
    }
}

/// Assures the given bytes use the native path separator.
pub fn to_native_separators<'a>(path: impl Into<Cow<'a, BStr>>) -> Cow<'a, BStr> {
    #[cfg(not(windows))]
    let p = to_unix_separators(path);
    #[cfg(windows)]
    let p = to_windows_separators(path);
    p
}

/// Convert paths with slashes to backslashes on windows and do nothing on unix, but **panics** if malformed surrogates are encountered on windows.
pub fn to_native_path_on_windows<'a>(path: impl Into<Cow<'a, BStr>>) -> Cow<'a, std::path::Path> {
    #[cfg(not(windows))]
    {
        crate::from_bstr(path)
    }
    #[cfg(windows)]
    {
        crate::from_bstr(to_windows_separators(path))
    }
}

/// Replaces windows path separators with slashes, but only do so on windows.
pub fn to_unix_separators_on_windows<'a>(path: impl Into<Cow<'a, BStr>>) -> Cow<'a, BStr> {
    #[cfg(windows)]
    {
        replace(path, b'\\', b'/')
    }
    #[cfg(not(windows))]
    {
        path.into()
    }
}

/// Replaces windows path separators with slashes, unconditionally.
///
/// **Note** Do not use these and prefer the conditional versions of this method.
// TODO: use https://lib.rs/crates/path-slash to handle escapes
pub fn to_unix_separators<'a>(path: impl Into<Cow<'a, BStr>>) -> Cow<'a, BStr> {
    replace(path, b'\\', b'/')
}

/// Find backslashes and replace them with slashes, which typically resembles a unix path, unconditionally.
///
/// **Note** Do not use these and prefer the conditional versions of this method.
// TODO: use https://lib.rs/crates/path-slash to handle escapes
pub fn to_windows_separators<'a>(path: impl Into<Cow<'a, BStr>>) -> Cow<'a, BStr> {
    replace(path, b'/', b'\\')
}

/// Resolve relative components virtually without accessing the file system, e.g. turn `a/./b/c/.././..` into `a`,
/// without keeping intermediate `..` and `/a/../b/..` becomes `/`.
/// If the input path was relative and ends up being the `current_dir`, `.` is returned instead of the full path to `current_dir`.
///
/// This is particularly useful when manipulating paths that are based on user input, and not resolving intermediate
/// symlinks keeps the path similar to what the user provided. If that's not desirable, use `[realpath()][crate::realpath()`
/// instead.
///
/// Note that we might access the `current_dir` if we run out of path components to pop off, which is expected to be absolute
/// as typical return value of `std::env::current_dir()`.
/// As a `current_dir` like `/c` can be exhausted by paths like `../../r`, `None` will be returned to indicate the inability
/// to produce a logically consistent path.
pub fn normalize<'a>(path: impl Into<Cow<'a, Path>>, current_dir: impl AsRef<Path>) -> Option<Cow<'a, Path>> {
    use std::path::Component::ParentDir;

    let path = path.into();
    if !path.components().any(|c| matches!(c, ParentDir)) {
        return Some(path);
    }
    let current_dir = current_dir.as_ref();
    let mut current_dir_opt = Some(current_dir);
    let was_relative = path.is_relative();
    let components = path.components();
    let mut path = PathBuf::new();
    for component in components {
        if let ParentDir = component {
            let path_was_dot = path == Path::new(".");
            if path.as_os_str().is_empty() || path_was_dot {
                path.push(current_dir_opt.take()?);
            }
            if !path.pop() {
                return None;
            }
        } else {
            path.push(component)
        }
    }

    if (path.as_os_str().is_empty() || path == current_dir) && was_relative {
        Cow::Borrowed(Path::new("."))
    } else {
        path.into()
    }
    .into()
}
