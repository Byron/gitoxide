//! Rust containers for valid `git-config` types.

use std::{borrow::Cow, convert::TryFrom, fmt::Display, str::FromStr};

use bstr::{BStr, BString, ByteSlice};
#[cfg(feature = "serde")]
use serde::{Serialize, Serializer};

use crate::value;

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
pub fn normalize_cow(input: Cow<'_, BStr>) -> Cow<'_, BStr> {
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
    normalize_cow(Cow::Borrowed(input.into()))
}

/// `Vec[u8]` variant of [`normalize_cow`].
#[must_use]
pub fn normalize_bstring(input: impl Into<BString>) -> Cow<'static, BStr> {
    normalize_cow(Cow::Owned(input.into()))
}

/// Any string value
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct String<'a> {
    /// The string value
    pub value: Cow<'a, BStr>,
}

impl<'a> From<Cow<'a, BStr>> for String<'a> {
    fn from(c: Cow<'a, BStr>) -> Self {
        String {
            value: normalize_cow(c),
        }
    }
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

/// Any value that can be interpreted as a boolean.
///
/// Note that while values can effectively be any byte string, the `git-config`
/// documentation has a strict subset of values that may be interpreted as a
/// boolean value, all of which are ASCII and thus UTF-8 representable.
/// Consequently, variants hold [`str`]s rather than [`[u8]`]s.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[allow(missing_docs)]
pub enum Boolean<'a> {
    True(TrueVariant<'a>),
    False(Cow<'a, BStr>),
}

impl Boolean<'_> {
    /// Return ourselves as plain bool.
    pub fn to_bool(&self) -> bool {
        match self {
            Boolean::True(_) => true,
            Boolean::False(_) => false,
        }
    }
    /// Generates a byte representation of the value. This should be used when
    /// non-UTF-8 sequences are present or a UTF-8 representation can't be
    /// guaranteed.
    #[must_use]
    pub fn to_bstring(&self) -> BString {
        self.into()
    }
}

fn bool_err(input: impl Into<BString>) -> value::parse::Error {
    value::parse::Error::new(
        "Booleans need to be 'no', 'off', 'false', 'zero' or 'yes', 'on', 'true', 'one'",
        input,
    )
}

impl<'a> TryFrom<&'a BStr> for Boolean<'a> {
    type Error = value::parse::Error;

    fn try_from(value: &'a BStr) -> Result<Self, Self::Error> {
        if let Ok(v) = TrueVariant::try_from(value) {
            return Ok(Self::True(v));
        }

        if value.eq_ignore_ascii_case(b"no")
            || value.eq_ignore_ascii_case(b"off")
            || value.eq_ignore_ascii_case(b"false")
            || value.eq_ignore_ascii_case(b"zero")
            || value == "\"\""
        {
            return Ok(Self::False(value.as_bstr().into()));
        }

        Err(bool_err(value))
    }
}

impl TryFrom<BString> for Boolean<'_> {
    type Error = value::parse::Error;

    fn try_from(value: BString) -> Result<Self, Self::Error> {
        if value.eq_ignore_ascii_case(b"no")
            || value.eq_ignore_ascii_case(b"off")
            || value.eq_ignore_ascii_case(b"false")
            || value.eq_ignore_ascii_case(b"zero")
            || value == "\"\""
        {
            return Ok(Self::False(Cow::Owned(value)));
        }

        TrueVariant::try_from(value).map(Self::True)
    }
}

impl<'a> TryFrom<Cow<'a, BStr>> for Boolean<'a> {
    type Error = value::parse::Error;
    fn try_from(c: Cow<'a, BStr>) -> Result<Self, Self::Error> {
        match c {
            Cow::Borrowed(c) => Self::try_from(c),
            Cow::Owned(c) => Self::try_from(c),
        }
    }
}

impl Display for Boolean<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Boolean::True(v) => v.fmt(f),
            Boolean::False(v) => write!(f, "{}", v),
        }
    }
}

impl From<Boolean<'_>> for bool {
    fn from(b: Boolean<'_>) -> Self {
        match b {
            Boolean::True(_) => true,
            Boolean::False(_) => false,
        }
    }
}

impl<'a, 'b: 'a> From<&'b Boolean<'a>> for &'a BStr {
    fn from(b: &'b Boolean<'_>) -> Self {
        match b {
            Boolean::True(t) => t.into(),
            Boolean::False(f) => f.as_ref(),
        }
    }
}

impl From<Boolean<'_>> for BString {
    fn from(b: Boolean<'_>) -> Self {
        b.into()
    }
}

impl From<&Boolean<'_>> for BString {
    fn from(b: &Boolean<'_>) -> Self {
        b.to_string().into()
    }
}

#[cfg(feature = "serde")]
impl Serialize for Boolean<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Boolean::True(_) => serializer.serialize_bool(true),
            Boolean::False(_) => serializer.serialize_bool(false),
        }
    }
}

/// Discriminating enum between implicit and explicit truthy values.
///
/// This enum is part of the [`Boolean`] struct.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[allow(missing_docs)]
pub enum TrueVariant<'a> {
    Explicit(Cow<'a, BStr>),
    /// For values defined without a `= <value>`.
    Implicit,
}

impl<'a> TryFrom<&'a BStr> for TrueVariant<'a> {
    type Error = value::parse::Error;

    fn try_from(value: &'a BStr) -> Result<Self, Self::Error> {
        if value.eq_ignore_ascii_case(b"yes")
            || value.eq_ignore_ascii_case(b"on")
            || value.eq_ignore_ascii_case(b"true")
            || value.eq_ignore_ascii_case(b"one")
        {
            Ok(Self::Explicit(value.as_bstr().into()))
        } else if value.is_empty() {
            Ok(Self::Implicit)
        } else {
            Err(bool_err(value))
        }
    }
}

impl TryFrom<BString> for TrueVariant<'_> {
    type Error = value::parse::Error;

    fn try_from(value: BString) -> Result<Self, Self::Error> {
        if value.eq_ignore_ascii_case(b"yes")
            || value.eq_ignore_ascii_case(b"on")
            || value.eq_ignore_ascii_case(b"true")
            || value.eq_ignore_ascii_case(b"one")
        {
            Ok(Self::Explicit(Cow::Owned(value)))
        } else if value.is_empty() {
            Ok(Self::Implicit)
        } else {
            Err(bool_err(value))
        }
    }
}

impl Display for TrueVariant<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Self::Explicit(v) = self {
            write!(f, "{}", v)
        } else {
            Ok(())
        }
    }
}

impl<'a, 'b: 'a> From<&'b TrueVariant<'a>> for &'a BStr {
    fn from(t: &'b TrueVariant<'a>) -> Self {
        match t {
            TrueVariant::Explicit(e) => e.as_ref(),
            TrueVariant::Implicit => "".into(),
        }
    }
}

#[cfg(feature = "serde")]
impl Serialize for TrueVariant<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bool(true)
    }
}

/// Any value that can be interpreted as an integer.
///
/// This supports any numeric value that can fit in a [`i64`], excluding the
/// suffix. The suffix is parsed separately from the value itself, so if you
/// wish to obtain the true value of the integer, you must account for the
/// suffix after fetching the value. [`IntegerSuffix`] provides
/// [`bitwise_offset`] to help with the math, but do be warned that if the value
/// is very large, you may run into overflows.
///
/// [`BStr`]: bstr::BStr
/// [`bitwise_offset`]: IntegerSuffix::bitwise_offset
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Integer {
    /// The value, without any suffix modification
    pub value: i64,
    /// A provided suffix, if any.
    pub suffix: Option<IntegerSuffix>,
}

impl Integer {
    /// Generates a byte representation of the value. This should be used when
    /// non-UTF-8 sequences are present or a UTF-8 representation can't be
    /// guaranteed.
    #[must_use]
    pub fn to_bstring(self) -> BString {
        self.into()
    }

    /// Canonicalize values as simple decimal numbers.
    /// An optional suffix of k, m, or g (case-insensitive), upon creation, will cause the value to be multiplied by
    /// 1024 (k), 1048576 (m), or 1073741824 (g) respectively.
    ///
    /// Returns the result if no multiplication overflow.
    pub fn to_decimal(&self) -> Option<i64> {
        match self.suffix {
            None => Some(self.value),
            Some(suffix) => match suffix {
                IntegerSuffix::Kibi => self.value.checked_mul(1024),
                IntegerSuffix::Mebi => self.value.checked_mul(1024 * 1024),
                IntegerSuffix::Gibi => self.value.checked_mul(1024 * 1024 * 1024),
            },
        }
    }
}

impl Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)?;
        if let Some(suffix) = self.suffix {
            write!(f, "{}", suffix)
        } else {
            Ok(())
        }
    }
}

#[cfg(feature = "serde")]
impl Serialize for Integer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(suffix) = self.suffix {
            serializer.serialize_i64(self.value << suffix.bitwise_offset())
        } else {
            serializer.serialize_i64(self.value)
        }
    }
}

fn int_err(input: impl Into<BString>) -> value::parse::Error {
    value::parse::Error::new(
        "Integers needs to be positive or negative numbers which may have a suffix like 1k, 42, or 50G",
        input,
    )
}

impl TryFrom<&BStr> for Integer {
    type Error = value::parse::Error;

    fn try_from(s: &BStr) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(s).map_err(|err| int_err(s).with_err(err))?;
        if let Ok(value) = s.parse() {
            return Ok(Self { value, suffix: None });
        }

        // Assume we have a prefix at this point.

        if s.len() <= 1 {
            return Err(int_err(s));
        }

        let (number, suffix) = s.split_at(s.len() - 1);
        if let (Ok(value), Ok(suffix)) = (number.parse(), suffix.parse()) {
            Ok(Self {
                value,
                suffix: Some(suffix),
            })
        } else {
            Err(int_err(s))
        }
    }
}

impl TryFrom<BString> for Integer {
    type Error = value::parse::Error;

    fn try_from(value: BString) -> Result<Self, Self::Error> {
        Self::try_from(value.as_ref())
    }
}

impl TryFrom<Cow<'_, BStr>> for Integer {
    type Error = value::parse::Error;

    fn try_from(c: Cow<'_, BStr>) -> Result<Self, Self::Error> {
        match c {
            Cow::Borrowed(c) => Self::try_from(c),
            Cow::Owned(c) => Self::try_from(c),
        }
    }
}

impl From<Integer> for BString {
    fn from(i: Integer) -> Self {
        i.into()
    }
}

impl From<&Integer> for BString {
    fn from(i: &Integer) -> Self {
        i.to_string().into()
    }
}

/// Integer prefixes that are supported by `git-config`.
///
/// These values are base-2 unit of measurements, not the base-10 variants.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[allow(missing_docs)]
pub enum IntegerSuffix {
    Kibi,
    Mebi,
    Gibi,
}

impl IntegerSuffix {
    /// Returns the number of bits that the suffix shifts left by.
    #[must_use]
    pub const fn bitwise_offset(self) -> usize {
        match self {
            Self::Kibi => 10,
            Self::Mebi => 20,
            Self::Gibi => 30,
        }
    }
}

impl Display for IntegerSuffix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Kibi => write!(f, "k"),
            Self::Mebi => write!(f, "m"),
            Self::Gibi => write!(f, "g"),
        }
    }
}

#[cfg(feature = "serde")]
impl Serialize for IntegerSuffix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match self {
            Self::Kibi => "k",
            Self::Mebi => "m",
            Self::Gibi => "g",
        })
    }
}

impl FromStr for IntegerSuffix {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "k" | "K" => Ok(Self::Kibi),
            "m" | "M" => Ok(Self::Mebi),
            "g" | "G" => Ok(Self::Gibi),
            _ => Err(()),
        }
    }
}

impl TryFrom<&[u8]> for IntegerSuffix {
    type Error = ();

    fn try_from(s: &[u8]) -> Result<Self, Self::Error> {
        Self::from_str(std::str::from_utf8(s).map_err(|_| ())?)
    }
}

impl TryFrom<Vec<u8>> for IntegerSuffix {
    type Error = ();

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(value.as_ref())
    }
}

/// Any value that may contain a foreground color, background color, a
/// collection of color (text) modifiers, or a combination of any of the
/// aforementioned values.
///
/// Note that `git-config` allows color values to simply be a collection of
/// [`ColorAttribute`]s, and does not require a [`ColorValue`] for either the
/// foreground or background color.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Color {
    /// A provided foreground color
    pub foreground: Option<ColorValue>,
    /// A provided background color
    pub background: Option<ColorValue>,
    /// A potentially empty list of text attributes
    pub attributes: Vec<ColorAttribute>,
}

impl Color {
    /// Generates a byte representation of the value. This should be used when
    /// non-UTF-8 sequences are present or a UTF-8 representation can't be
    /// guaranteed.
    #[must_use]
    pub fn to_bstring(&self) -> BString {
        self.into()
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(fg) = self.foreground {
            fg.fmt(f)?;
        }

        write!(f, " ")?;

        if let Some(bg) = self.background {
            bg.fmt(f)?;
        }

        self.attributes
            .iter()
            .try_for_each(|attr| write!(f, " ").and_then(|_| attr.fmt(f)))
    }
}

#[cfg(feature = "serde")]
impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // todo: maybe not?
        serializer.serialize_str(&self.to_string())
    }
}

fn color_err(input: impl Into<BString>) -> value::parse::Error {
    value::parse::Error::new(
        "Colors are specific color values and their attributes, like 'brightred', or 'blue'",
        input,
    )
}

impl TryFrom<&BStr> for Color {
    type Error = value::parse::Error;

    fn try_from(s: &BStr) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(s).map_err(|err| color_err(s).with_err(err))?;
        enum ColorItem {
            Value(ColorValue),
            Attr(ColorAttribute),
        }

        let items = s.split_whitespace().filter_map(|s| {
            if s.is_empty() {
                return None;
            }

            Some(
                ColorValue::from_str(s)
                    .map(ColorItem::Value)
                    .or_else(|_| ColorAttribute::from_str(s).map(ColorItem::Attr)),
            )
        });

        let mut new_self = Self::default();
        for item in items {
            match item {
                Ok(item) => match item {
                    ColorItem::Value(v) => {
                        if new_self.foreground.is_none() {
                            new_self.foreground = Some(v);
                        } else if new_self.background.is_none() {
                            new_self.background = Some(v);
                        } else {
                            return Err(color_err(s));
                        }
                    }
                    ColorItem::Attr(a) => new_self.attributes.push(a),
                },
                Err(_) => return Err(color_err(s)),
            }
        }

        Ok(new_self)
    }
}

impl TryFrom<BString> for Color {
    type Error = value::parse::Error;

    fn try_from(value: BString) -> Result<Self, Self::Error> {
        Self::try_from(value.as_ref())
    }
}

impl TryFrom<Cow<'_, BStr>> for Color {
    type Error = value::parse::Error;

    fn try_from(c: Cow<'_, BStr>) -> Result<Self, Self::Error> {
        match c {
            Cow::Borrowed(c) => Self::try_from(c),
            Cow::Owned(c) => Self::try_from(c),
        }
    }
}

impl From<Color> for BString {
    fn from(c: Color) -> Self {
        c.into()
    }
}

impl From<&Color> for BString {
    fn from(c: &Color) -> Self {
        c.to_string().into()
    }
}

/// Discriminating enum for [`Color`] values.
///
/// `git-config` supports the eight standard colors, their bright variants, an
/// ANSI color code, or a 24-bit hex value prefixed with an octothorpe.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[allow(missing_docs)]
pub enum ColorValue {
    Normal,
    Black,
    BrightBlack,
    Red,
    BrightRed,
    Green,
    BrightGreen,
    Yellow,
    BrightYellow,
    Blue,
    BrightBlue,
    Magenta,
    BrightMagenta,
    Cyan,
    BrightCyan,
    White,
    BrightWhite,
    Ansi(u8),
    Rgb(u8, u8, u8),
}

impl Display for ColorValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => write!(f, "normal"),
            Self::Black => write!(f, "black"),
            Self::BrightBlack => write!(f, "brightblack"),
            Self::Red => write!(f, "red"),
            Self::BrightRed => write!(f, "brightred"),
            Self::Green => write!(f, "green"),
            Self::BrightGreen => write!(f, "brightgreen"),
            Self::Yellow => write!(f, "yellow"),
            Self::BrightYellow => write!(f, "brightyellow"),
            Self::Blue => write!(f, "blue"),
            Self::BrightBlue => write!(f, "brightblue"),
            Self::Magenta => write!(f, "magenta"),
            Self::BrightMagenta => write!(f, "brightmagenta"),
            Self::Cyan => write!(f, "cyan"),
            Self::BrightCyan => write!(f, "brightcyan"),
            Self::White => write!(f, "white"),
            Self::BrightWhite => write!(f, "brightwhite"),
            Self::Ansi(num) => num.fmt(f),
            Self::Rgb(r, g, b) => write!(f, "#{:02x}{:02x}{:02x}", r, g, b),
        }
    }
}

#[cfg(feature = "serde")]
impl Serialize for ColorValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl FromStr for ColorValue {
    type Err = value::parse::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s;
        let bright = if s.starts_with("bright") {
            s = &s[6..];
            true
        } else {
            false
        };

        match s {
            "normal" if !bright => return Ok(Self::Normal),
            "normal" if bright => return Err(color_err(s)),
            "black" if !bright => return Ok(Self::Black),
            "black" if bright => return Ok(Self::BrightBlack),
            "red" if !bright => return Ok(Self::Red),
            "red" if bright => return Ok(Self::BrightRed),
            "green" if !bright => return Ok(Self::Green),
            "green" if bright => return Ok(Self::BrightGreen),
            "yellow" if !bright => return Ok(Self::Yellow),
            "yellow" if bright => return Ok(Self::BrightYellow),
            "blue" if !bright => return Ok(Self::Blue),
            "blue" if bright => return Ok(Self::BrightBlue),
            "magenta" if !bright => return Ok(Self::Magenta),
            "magenta" if bright => return Ok(Self::BrightMagenta),
            "cyan" if !bright => return Ok(Self::Cyan),
            "cyan" if bright => return Ok(Self::BrightCyan),
            "white" if !bright => return Ok(Self::White),
            "white" if bright => return Ok(Self::BrightWhite),
            _ => (),
        }

        if let Ok(v) = u8::from_str(s) {
            return Ok(Self::Ansi(v));
        }

        if let Some(s) = s.strip_prefix('#') {
            if s.len() == 6 {
                let rgb = (
                    u8::from_str_radix(&s[..2], 16),
                    u8::from_str_radix(&s[2..4], 16),
                    u8::from_str_radix(&s[4..], 16),
                );

                if let (Ok(r), Ok(g), Ok(b)) = rgb {
                    return Ok(Self::Rgb(r, g, b));
                }
            }
        }

        Err(color_err(s))
    }
}

impl TryFrom<&[u8]> for ColorValue {
    type Error = value::parse::Error;

    fn try_from(s: &[u8]) -> Result<Self, Self::Error> {
        Self::from_str(std::str::from_utf8(s).map_err(|err| color_err(s).with_err(err))?)
    }
}

/// Discriminating enum for [`Color`] attributes.
///
/// `git-config` supports modifiers and their negators. The negating color
/// attributes are equivalent to having a `no` or `no-` prefix to the normal
/// variant.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[allow(missing_docs)]
pub enum ColorAttribute {
    Bold,
    NoBold,
    Dim,
    NoDim,
    Ul,
    NoUl,
    Blink,
    NoBlink,
    Reverse,
    NoReverse,
    Italic,
    NoItalic,
    Strike,
    NoStrike,
}

impl Display for ColorAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bold => write!(f, "bold"),
            Self::NoBold => write!(f, "nobold"),
            Self::Dim => write!(f, "dim"),
            Self::NoDim => write!(f, "nodim"),
            Self::Ul => write!(f, "ul"),
            Self::NoUl => write!(f, "noul"),
            Self::Blink => write!(f, "blink"),
            Self::NoBlink => write!(f, "noblink"),
            Self::Reverse => write!(f, "reverse"),
            Self::NoReverse => write!(f, "noreverse"),
            Self::Italic => write!(f, "italic"),
            Self::NoItalic => write!(f, "noitalic"),
            Self::Strike => write!(f, "strike"),
            Self::NoStrike => write!(f, "nostrike"),
        }
    }
}

#[cfg(feature = "serde")]
impl Serialize for ColorAttribute {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            Self::Bold => "bold",
            Self::NoBold => "nobold",
            Self::Dim => "dim",
            Self::NoDim => "nodim",
            Self::Ul => "ul",
            Self::NoUl => "noul",
            Self::Blink => "blink",
            Self::NoBlink => "noblink",
            Self::Reverse => "reverse",
            Self::NoReverse => "noreverse",
            Self::Italic => "italic",
            Self::NoItalic => "noitalic",
            Self::Strike => "strike",
            Self::NoStrike => "nostrike",
        })
    }
}

impl FromStr for ColorAttribute {
    type Err = value::parse::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inverted = s.starts_with("no");
        let mut parsed = s;

        if inverted {
            parsed = &parsed[2..];

            if parsed.starts_with('-') {
                parsed = &parsed[1..];
            }
        }

        match parsed {
            "bold" if !inverted => Ok(Self::Bold),
            "bold" if inverted => Ok(Self::NoBold),
            "dim" if !inverted => Ok(Self::Dim),
            "dim" if inverted => Ok(Self::NoDim),
            "ul" if !inverted => Ok(Self::Ul),
            "ul" if inverted => Ok(Self::NoUl),
            "blink" if !inverted => Ok(Self::Blink),
            "blink" if inverted => Ok(Self::NoBlink),
            "reverse" if !inverted => Ok(Self::Reverse),
            "reverse" if inverted => Ok(Self::NoReverse),
            "italic" if !inverted => Ok(Self::Italic),
            "italic" if inverted => Ok(Self::NoItalic),
            "strike" if !inverted => Ok(Self::Strike),
            "strike" if inverted => Ok(Self::NoStrike),
            _ => Err(color_err(parsed)),
        }
    }
}

impl TryFrom<&[u8]> for ColorAttribute {
    type Error = value::parse::Error;

    fn try_from(s: &[u8]) -> Result<Self, Self::Error> {
        Self::from_str(std::str::from_utf8(s).map_err(|err| color_err(s).with_err(err))?)
    }
}
