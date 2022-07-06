//! Rust containers for valid `git-config` types.

use std::borrow::Cow;

use bstr::{BStr, BString};
#[cfg(feature = "serde")]
use serde::{Serialize, Serializer};

/// Removes quotes, if any, from the provided inputs. This assumes the input
/// contains a even number of unescaped quotes, and will unescape escaped
/// quotes. The return values should be safe for value interpretation.
///
/// This has optimizations for fully-quoted values, where the returned value
/// will be a borrowed reference if the only mutation necessary is to unquote
/// the value.
///
/// This is the function used to normalize raw values from higher level
/// abstractions over the [`parser`] implementation. Generally speaking these
/// high level abstractions will handle normalization for you, and you do not
/// need to call this yourself. However, if you're directly handling events
/// from the parser, you may want to use this to help with value interpretation.
///
/// Generally speaking, you'll want to use one of the variants of this function,
/// such as [`normalize_bstr`] or [`normalize_bstring`].
///
/// # Examples
///
/// Values don't need modification are returned borrowed, without allocation.
///
/// ```
/// # use std::borrow::Cow;
/// # use bstr::ByteSlice;
/// # use git_config::values::normalize_bstr;
/// assert_eq!(normalize_bstr("hello world"), Cow::Borrowed(b"hello world".as_bstr()));
/// ```
///
/// Fully quoted values are optimized to not need allocations.
///
/// ```
/// # use std::borrow::Cow;
/// # use bstr::ByteSlice;
/// # use git_config::values::normalize_bstr;
/// assert_eq!(normalize_bstr("\"hello world\""), Cow::Borrowed(b"hello world".as_bstr()));
/// ```
///
/// Quoted values are unwrapped as an owned variant.
///
/// ```
/// # use std::borrow::Cow;
/// # use bstr::{BStr, BString};
/// # use git_config::values::{normalize_bstr};
/// assert_eq!(normalize_bstr("hello \"world\""), Cow::<BStr>::Owned(BString::from( "hello world" )));
/// ```
///
/// Escaped quotes are unescaped.
///
/// ```
/// # use std::borrow::Cow;
/// # use bstr::{BStr, BString};
/// # use git_config::values::normalize_bstr;
/// assert_eq!(normalize_bstr(r#"hello "world\"""#), Cow::<BStr>::Owned(BString::from(r#"hello world""#)));
/// ```
///
/// [`parser`]: crate::parser::Parser
#[must_use]
pub fn normalize(input: Cow<'_, BStr>) -> Cow<'_, BStr> {
    let size = input.len();
    if input.as_ref() == "\"\"" {
        return Cow::default();
    }

    if size >= 3 && input[0] == b'=' && input[size - 1] == b'=' && input[size - 2] != b'\\' {
        match input {
            Cow::Borrowed(input) => return normalize_bstr(&input[1..size]),
            Cow::Owned(mut input) => {
                input.pop();
                input.remove(0);
                return normalize_bstring(input);
            }
        }
    }

    let mut owned = BString::default();

    let mut first_index = 0;
    let mut last_index = 0;
    let mut was_escaped = false;
    for (i, c) in input.iter().enumerate() {
        if was_escaped {
            was_escaped = false;
            if *c == b'"' {
                if first_index == 0 {
                    owned.extend(&*input[last_index..i - 1]);
                    last_index = i;
                } else {
                    owned.extend(&*input[first_index..i - 1]);
                    first_index = i;
                }
            }
            continue;
        }

        if *c == b'\\' {
            was_escaped = true;
        } else if *c == b'"' {
            if first_index == 0 {
                owned.extend(&*input[last_index..i]);
                first_index = i + 1;
            } else {
                owned.extend(&*input[first_index..i]);
                first_index = 0;
                last_index = i + 1;
            }
        }
    }

    if last_index == 0 {
        input
    } else {
        owned.extend(&*input[last_index..]);
        Cow::Owned(owned)
    }
}

/// `&[u8]` variant of [`normalize_cow`].
#[must_use]
pub fn normalize_bstr<'a>(input: impl Into<&'a BStr>) -> Cow<'a, BStr> {
    normalize(Cow::Borrowed(input.into()))
}

/// `Vec[u8]` variant of [`normalize_cow`].
#[must_use]
pub fn normalize_bstring(input: impl Into<BString>) -> Cow<'static, BStr> {
    normalize(Cow::Owned(input.into()))
}

///
pub mod path {
    use std::borrow::Cow;

    #[cfg(not(any(target_os = "android", target_os = "windows")))]
    fn home_for_user(name: &str) -> Option<std::path::PathBuf> {
        let cname = std::ffi::CString::new(name).ok()?;
        // SAFETY: calling this in a threaded program that modifies the pw database is not actually safe.
        //         TODO: use the `*_r` version, but it's much harder to use.
        #[allow(unsafe_code)]
        let pwd = unsafe { libc::getpwnam(cname.as_ptr()) };
        if pwd.is_null() {
            None
        } else {
            use std::os::unix::ffi::OsStrExt;
            // SAFETY: pw_dir is a cstr and it lives as long as… well, we hope nobody changes the pw database while we are at it
            //         from another thread. Otherwise it lives long enough.
            #[allow(unsafe_code)]
            let cstr = unsafe { std::ffi::CStr::from_ptr((*pwd).pw_dir) };
            Some(std::ffi::OsStr::from_bytes(cstr.to_bytes()).into())
        }
    }

    use crate::values::Path;

    pub mod interpolate {
        /// The error returned by [`Path::interpolate()`][crate::values::Path::interpolate()].
        #[derive(Debug, thiserror::Error)]
        #[allow(missing_docs)]
        pub enum Error {
            #[error("{} is missing", .what)]
            Missing { what: &'static str },
            #[error("Ill-formed UTF-8 in {}", .what)]
            Utf8Conversion {
                what: &'static str,
                #[source]
                err: git_path::Utf8Error,
            },
            #[error("Ill-formed UTF-8 in username")]
            UsernameConversion(#[from] std::str::Utf8Error),
            #[error("User interpolation is not available on this platform")]
            UserInterpolationUnsupported,
        }
    }

    impl<'a> Path<'a> {
        /// Interpolates this path into a file system path.
        ///
        /// If this path starts with `~/` or `~user/` or `%(prefix)/`
        ///  - `~/` is expanded to the value of `home_dir`. The caller can use the [dirs](https://crates.io/crates/dirs) crate to obtain it.
        ///    It it is required but not set, an error is produced.
        ///  - `~user/` to the specified user’s home directory, e.g `~alice` might get expanded to `/home/alice` on linux.
        /// The interpolation uses `getpwnam` sys call and is therefore not available on windows. See also [pwd](https://crates.io/crates/pwd).
        ///  - `%(prefix)/` is expanded to the location where gitoxide is installed. This location is not known at compile time and therefore need to be
        /// optionally provided by the caller through `git_install_dir`.
        ///
        /// Any other, non-empty path value is returned unchanged and error is returned in case of an empty path value.
        pub fn interpolate(
            self,
            git_install_dir: Option<&std::path::Path>,
            home_dir: Option<&std::path::Path>,
        ) -> Result<Cow<'a, std::path::Path>, interpolate::Error> {
            if self.is_empty() {
                return Err(interpolate::Error::Missing { what: "path" });
            }

            const PREFIX: &[u8] = b"%(prefix)/";
            const USER_HOME: &[u8] = b"~/";
            if self.starts_with(PREFIX) {
                let git_install_dir = git_install_dir.ok_or(interpolate::Error::Missing {
                    what: "git install dir",
                })?;
                let (_prefix, path_without_trailing_slash) = self.split_at(PREFIX.len());
                let path_without_trailing_slash =
                    git_path::try_from_bstring(path_without_trailing_slash).map_err(|err| {
                        interpolate::Error::Utf8Conversion {
                            what: "path past %(prefix)",
                            err,
                        }
                    })?;
                Ok(git_install_dir.join(path_without_trailing_slash).into())
            } else if self.starts_with(USER_HOME) {
                let home_path = home_dir.ok_or(interpolate::Error::Missing { what: "home dir" })?;
                let (_prefix, val) = self.split_at(USER_HOME.len());
                let val = git_path::try_from_byte_slice(val).map_err(|err| interpolate::Error::Utf8Conversion {
                    what: "path past ~/",
                    err,
                })?;
                Ok(home_path.join(val).into())
            } else if self.starts_with(b"~") && self.contains(&b'/') {
                self.interpolate_user()
            } else {
                Ok(git_path::from_bstr(self.value))
            }
        }

        #[cfg(any(target_os = "windows", target_os = "android"))]
        fn interpolate_user(self) -> Result<Cow<'a, std::path::Path>, interpolate::Error> {
            Err(interpolate::Error::UserInterpolationUnsupported)
        }

        #[cfg(not(windows))]
        fn interpolate_user(self) -> Result<Cow<'a, std::path::Path>, interpolate::Error> {
            let (_prefix, val) = self.split_at("/".len());
            let i = val
                .iter()
                .position(|&e| e == b'/')
                .ok_or(interpolate::Error::Missing { what: "/" })?;
            let (username, path_with_leading_slash) = val.split_at(i);
            let username = std::str::from_utf8(username)?;
            let home = home_for_user(username).ok_or(interpolate::Error::Missing { what: "pwd user info" })?;
            let path_past_user_prefix =
                git_path::try_from_byte_slice(&path_with_leading_slash["/".len()..]).map_err(|err| {
                    interpolate::Error::Utf8Conversion {
                        what: "path past ~user/",
                        err,
                    }
                })?;
            Ok(home.join(path_past_user_prefix).into())
        }
    }
}

/// Any value that can be interpreted as a file path.
///
/// Git represents file paths as byte arrays, modeled here as owned or borrowed byte sequences.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Path<'a> {
    /// The path string, un-interpolated
    pub value: Cow<'a, BStr>,
}

impl<'a> std::ops::Deref for Path<'a> {
    type Target = BStr;

    fn deref(&self) -> &Self::Target {
        self.value.as_ref()
    }
}

impl<'a> AsRef<[u8]> for Path<'a> {
    fn as_ref(&self) -> &[u8] {
        self.value.as_ref()
    }
}

impl<'a> AsRef<BStr> for Path<'a> {
    fn as_ref(&self) -> &BStr {
        self.value.as_ref()
    }
}

impl<'a> From<Cow<'a, BStr>> for Path<'a> {
    fn from(value: Cow<'a, BStr>) -> Self {
        Path { value }
    }
}
