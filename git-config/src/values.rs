//! Rust containers for valid `git-config` types.

use std::{borrow::Cow, convert::TryFrom, fmt::Display, str::FromStr};

use bstr::{BStr, ByteSlice};
use quick_error::quick_error;
#[cfg(feature = "serde")]
use serde::{Serialize, Serializer};
#[cfg(feature = "expiry-date")]
use time::macros::format_description;
#[cfg(feature = "expiry-date")]
use time::{OffsetDateTime, PrimitiveDateTime};

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
/// such as [`normalize_str`] or [`normalize_vec`].
///
/// # Examples
///
/// Values don't need modification are returned borrowed, without allocation.
///
/// ```
/// # use std::borrow::Cow;
/// # use git_config::values::normalize_str;
/// assert_eq!(normalize_str("hello world"), Cow::Borrowed(b"hello world".as_slice()));
/// ```
///
/// Fully quoted values are optimized to not need allocations.
///
/// ```
/// # use std::borrow::Cow;
/// # use git_config::values::normalize_str;
/// assert_eq!(normalize_str("\"hello world\""), Cow::Borrowed(b"hello world".as_slice()));
/// ```
///
/// Quoted values are unwrapped as an owned variant.
///
/// ```
/// # use std::borrow::Cow;
/// # use git_config::values::normalize_str;
/// assert_eq!(normalize_str("hello \"world\""), Cow::<[u8]>::Owned(b"hello world".to_vec()));
/// ```
///
/// Escaped quotes are unescaped.
///
/// ```
/// # use std::borrow::Cow;
/// # use git_config::values::normalize_str;
/// assert_eq!(normalize_str(r#"hello "world\"""#), Cow::<[u8]>::Owned(br#"hello world""#.to_vec()));
/// ```
///
/// [`parser`]: crate::parser::Parser
#[must_use]
pub fn normalize_cow(input: Cow<'_, [u8]>) -> Cow<'_, [u8]> {
    let size = input.len();
    if &*input == b"\"\"" {
        return Cow::Borrowed(&[]);
    }

    if size >= 3 && input[0] == b'=' && input[size - 1] == b'=' && input[size - 2] != b'\\' {
        match input {
            Cow::Borrowed(input) => return normalize_bytes(&input[1..size]),
            Cow::Owned(mut input) => {
                input.pop();
                input.remove(0);
                return normalize_vec(input);
            }
        }
    }

    let mut owned = vec![];

    let mut first_index = 0;
    let mut last_index = 0;
    let mut was_escaped = false;
    for (i, c) in input.iter().enumerate() {
        if was_escaped {
            was_escaped = false;
            if *c == b'"' {
                if first_index == 0 {
                    owned.extend(&input[last_index..i - 1]);
                    last_index = i;
                } else {
                    owned.extend(&input[first_index..i - 1]);
                    first_index = i;
                }
            }
            continue;
        }

        if *c == b'\\' {
            was_escaped = true;
        } else if *c == b'"' {
            if first_index == 0 {
                owned.extend(&input[last_index..i]);
                first_index = i + 1;
            } else {
                owned.extend(&input[first_index..i]);
                first_index = 0;
                last_index = i + 1;
            }
        }
    }

    if last_index == 0 {
        input
    } else {
        owned.extend(&input[last_index..]);
        Cow::Owned(owned)
    }
}

/// `&[u8]` variant of [`normalize_cow`].
#[inline]
#[must_use]
pub fn normalize_bytes(input: &[u8]) -> Cow<'_, [u8]> {
    normalize_cow(Cow::Borrowed(input))
}

/// `Vec[u8]` variant of [`normalize_cow`].
#[inline]
#[must_use]
pub fn normalize_vec(input: Vec<u8>) -> Cow<'static, [u8]> {
    normalize_cow(Cow::Owned(input))
}

/// [`str`] variant of [`normalize_cow`].
#[inline]
#[must_use]
pub fn normalize_str(input: &str) -> Cow<'_, [u8]> {
    normalize_bytes(input.as_bytes())
}

// TODO: remove bytes
/// Any string value
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Bytes<'a> {
    /// bytes
    pub value: Cow<'a, [u8]>,
}

impl<'a> From<&'a [u8]> for Bytes<'a> {
    #[inline]
    fn from(s: &'a [u8]) -> Self {
        Self {
            value: Cow::Borrowed(s),
        }
    }
}

impl From<Vec<u8>> for Bytes<'_> {
    fn from(s: Vec<u8>) -> Self {
        Self { value: Cow::Owned(s) }
    }
}

impl<'a> From<Cow<'a, [u8]>> for Bytes<'a> {
    #[inline]
    fn from(c: Cow<'a, [u8]>) -> Self {
        match c {
            Cow::Borrowed(c) => Self::from(c),
            Cow::Owned(c) => Self::from(c),
        }
    }
}

/// Any string value
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct String<'a> {
    /// The string value
    pub value: Cow<'a, BStr>,
}

impl<'a> From<Cow<'a, [u8]>> for String<'a> {
    #[inline]
    fn from(c: Cow<'a, [u8]>) -> Self {
        String {
            value: match c {
                Cow::Borrowed(c) => Cow::Borrowed(c.into()),
                Cow::Owned(c) => Cow::Owned(c.into()),
            },
        }
    }
}

///
pub mod path {
    use std::borrow::Cow;

    #[cfg(not(any(target_os = "android", target_os = "windows")))]
    use pwd::Passwd;
    use quick_error::ResultExt;

    use crate::values::Path;

    pub mod interpolate {
        use quick_error::quick_error;

        quick_error! {
            #[derive(Debug)]
            /// The error returned by [`Path::interpolate()`].
            #[allow(missing_docs)]
            pub enum Error {
                Missing { what: &'static str } {
                    display("{} is missing", what)
                }
                Utf8Conversion(what: &'static str, err: git_features::path::Utf8Error) {
                    display("Ill-formed UTF-8 in {}", what)
                    context(what: &'static str, err: git_features::path::Utf8Error) -> (what, err)
                    source(err)
                }
                UsernameConversion(err: std::str::Utf8Error) {
                    display("Ill-formed UTF-8 in username")
                    source(err)
                    from()
                }
                PwdFileQuery {
                    display("User home info missing")
                }
                UserInterpolationUnsupported {
                    display("User interpolation is not available on this platform")
                }
            }
        }
    }

    impl<'a> Path<'a> {
        /// Interpolates this path into a file system path.
        ///
        /// If this path starts with `~/` or `~user/` or `%(prefix)/`
        ///  - `~/` is expanded to the value of `$HOME` on unix based systems. On windows, `SHGetKnownFolderPath` is used.
        /// See also [dirs](https://crates.io/crates/dirs).
        ///  - `~user/` to the specified user’s home directory, e.g `~alice` might get expanded to `/home/alice` on linux.
        /// The interpolation uses `getpwnam` sys call and is therefore not available on windows. See also [pwd](https://crates.io/crates/pwd).
        ///  - `%(prefix)/` is expanded to the location where gitoxide is installed. This location is not known at compile time and therefore need to be
        /// optionally provided by the caller through `git_install_dir`.
        ///
        /// Any other, non-empty path value is returned unchanged and error is returned in case of an empty path value.
        pub fn interpolate(
            self,
            git_install_dir: Option<&std::path::Path>,
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
                    git_features::path::from_byte_vec(path_without_trailing_slash).context("path past %(prefix)")?;
                Ok(git_install_dir.join(path_without_trailing_slash).into())
            } else if self.starts_with(USER_HOME) {
                let home_path = dirs::home_dir().ok_or(interpolate::Error::Missing { what: "home dir" })?;
                let (_prefix, val) = self.split_at(USER_HOME.len());
                let val = git_features::path::from_bytes(val).context("path past ~/")?;
                Ok(home_path.join(val).into())
            } else if self.starts_with(b"~") && self.contains(&b'/') {
                self.interpolate_user()
            } else {
                Ok(git_features::path::from_bytes(self.value).context("unexpanded path")?)
            }
        }

        #[cfg(any(target_os = "windows", target_os = "android"))]
        fn interpolate_user(self) -> Result<Cow<'a, std::path::Path>, interpolate::Error> {
            Err(interpolate::Error::UserInterpolationUnsupported)
        }

        #[cfg(not(target_os = "windows"))]
        fn interpolate_user(self) -> Result<Cow<'a, std::path::Path>, interpolate::Error> {
            let (_prefix, val) = self.split_at("/".len());
            let i = val
                .iter()
                .position(|&e| e == b'/')
                .ok_or(interpolate::Error::Missing { what: "/" })?;
            let (username, path_with_leading_slash) = val.split_at(i);
            let username = std::str::from_utf8(username)?;
            let home = Passwd::from_name(username)
                .map_err(|_| interpolate::Error::PwdFileQuery)?
                .ok_or(interpolate::Error::Missing { what: "pwd user info" })?
                .dir;
            let path_past_user_prefix = git_features::path::from_byte_slice(&path_with_leading_slash["/".len()..])
                .context("path past ~user/")?;
            Ok(std::path::PathBuf::from(home).join(path_past_user_prefix).into())
        }
    }
}

/// Any value that can be interpreted as a file path.
///
/// Git represents file paths as byte arrays, modeled here as owned or borrowed byte sequences.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Path<'a> {
    /// The path string, un-interpolated
    pub value: Cow<'a, [u8]>,
}

impl<'a> std::ops::Deref for Path<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.value.as_ref()
    }
}

impl<'a> AsRef<[u8]> for Path<'a> {
    fn as_ref(&self) -> &[u8] {
        self.value.as_ref()
    }
}

impl<'a> From<Cow<'a, [u8]>> for Path<'a> {
    #[inline]
    fn from(value: Cow<'a, [u8]>) -> Self {
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
    False(Cow<'a, str>),
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
    #[inline]
    #[must_use]
    pub fn to_vec(&self) -> Vec<u8> {
        self.into()
    }

    /// Generates a byte representation of the value. This should be used when
    /// non-UTF-8 sequences are present or a UTF-8 representation can't be
    /// guaranteed.
    #[inline]
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.into()
    }
}

quick_error! {
    #[derive(Debug, PartialEq)]
    /// The error returned when creating `Boolean` from byte string.
    #[allow(missing_docs)]
    pub enum BooleanError {
        InvalidFormat {
            display("Invalid argument format")
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for Boolean<'a> {
    type Error = BooleanError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        if let Ok(v) = TrueVariant::try_from(value) {
            return Ok(Self::True(v));
        }

        if value.eq_ignore_ascii_case(b"no")
            || value.eq_ignore_ascii_case(b"off")
            || value.eq_ignore_ascii_case(b"false")
            || value.eq_ignore_ascii_case(b"zero")
            || value == b"\"\""
        {
            return Ok(Self::False(
                std::str::from_utf8(value).expect("value is already validated").into(),
            ));
        }

        Err(BooleanError::InvalidFormat)
    }
}

impl TryFrom<Vec<u8>> for Boolean<'_> {
    type Error = BooleanError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.eq_ignore_ascii_case(b"no")
            || value.eq_ignore_ascii_case(b"off")
            || value.eq_ignore_ascii_case(b"false")
            || value.eq_ignore_ascii_case(b"zero")
            || value == b"\"\""
        {
            return Ok(Self::False(Cow::Owned(
                std::string::String::from_utf8(value).expect("value is already validated"),
            )));
        }

        TrueVariant::try_from(value).map(Self::True)
    }
}

impl<'a> TryFrom<Cow<'a, [u8]>> for Boolean<'a> {
    type Error = BooleanError;
    fn try_from(c: Cow<'a, [u8]>) -> Result<Self, Self::Error> {
        match c {
            Cow::Borrowed(c) => Self::try_from(c),
            Cow::Owned(c) => Self::try_from(c),
        }
    }
}

impl Display for Boolean<'_> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Boolean::True(v) => v.fmt(f),
            Boolean::False(v) => write!(f, "{}", v),
        }
    }
}

impl From<Boolean<'_>> for bool {
    #[inline]
    fn from(b: Boolean) -> Self {
        match b {
            Boolean::True(_) => true,
            Boolean::False(_) => false,
        }
    }
}

impl<'a, 'b: 'a> From<&'b Boolean<'a>> for &'a [u8] {
    #[inline]
    fn from(b: &'b Boolean) -> Self {
        match b {
            Boolean::True(t) => t.into(),
            Boolean::False(f) => f.as_bytes(),
        }
    }
}

impl From<Boolean<'_>> for Vec<u8> {
    #[inline]
    fn from(b: Boolean) -> Self {
        b.into()
    }
}

impl From<&Boolean<'_>> for Vec<u8> {
    #[inline]
    fn from(b: &Boolean) -> Self {
        b.to_string().into_bytes()
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
    Explicit(Cow<'a, str>),
    /// For values defined without a `= <value>`.
    Implicit,
}

impl<'a> TryFrom<&'a [u8]> for TrueVariant<'a> {
    type Error = BooleanError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        if value.eq_ignore_ascii_case(b"yes")
            || value.eq_ignore_ascii_case(b"on")
            || value.eq_ignore_ascii_case(b"true")
            || value.eq_ignore_ascii_case(b"one")
        {
            Ok(Self::Explicit(
                std::str::from_utf8(value).expect("value is already validated").into(),
            ))
        } else if value.is_empty() {
            Ok(Self::Implicit)
        } else {
            Err(BooleanError::InvalidFormat)
        }
    }
}

impl TryFrom<Vec<u8>> for TrueVariant<'_> {
    type Error = BooleanError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.eq_ignore_ascii_case(b"yes")
            || value.eq_ignore_ascii_case(b"on")
            || value.eq_ignore_ascii_case(b"true")
            || value.eq_ignore_ascii_case(b"one")
        {
            Ok(Self::Explicit(Cow::Owned(
                std::string::String::from_utf8(value).expect("value is already validated"),
            )))
        } else if value.is_empty() {
            Ok(Self::Implicit)
        } else {
            Err(BooleanError::InvalidFormat)
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

impl<'a, 'b: 'a> From<&'b TrueVariant<'a>> for &'a [u8] {
    #[inline]
    fn from(t: &'b TrueVariant<'a>) -> Self {
        match t {
            TrueVariant::Explicit(e) => e.as_bytes(),
            TrueVariant::Implicit => &[],
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
    pub fn to_vec(self) -> Vec<u8> {
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

quick_error! {
    #[derive(Debug)]
    /// The error returned when creating `Integer` from byte string.
    #[allow(missing_docs)]
    pub enum IntegerError {
        Utf8Conversion(err: std::str::Utf8Error) {
            display("Ill-formed UTF-8")
            source(err)
            from()
        }
        InvalidFormat {
            display("Invalid argument format")
        }
        InvalidSuffix {
            display("Invalid suffix")
        }
    }
}

impl TryFrom<&[u8]> for Integer {
    type Error = IntegerError;

    #[inline]
    fn try_from(s: &[u8]) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(s)?;
        if let Ok(value) = s.parse() {
            return Ok(Self { value, suffix: None });
        }

        // Assume we have a prefix at this point.

        if s.len() <= 1 {
            return Err(IntegerError::InvalidFormat);
        }

        let (number, suffix) = s.split_at(s.len() - 1);
        if let (Ok(value), Ok(suffix)) = (number.parse(), suffix.parse()) {
            Ok(Self {
                value,
                suffix: Some(suffix),
            })
        } else {
            Err(IntegerError::InvalidFormat)
        }
    }
}

impl TryFrom<Vec<u8>> for Integer {
    type Error = IntegerError;

    #[inline]
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(value.as_ref())
    }
}

impl TryFrom<Cow<'_, [u8]>> for Integer {
    type Error = IntegerError;

    #[inline]
    fn try_from(c: Cow<'_, [u8]>) -> Result<Self, Self::Error> {
        match c {
            Cow::Borrowed(c) => Self::try_from(c),
            Cow::Owned(c) => Self::try_from(c),
        }
    }
}

impl From<Integer> for Vec<u8> {
    #[inline]
    fn from(i: Integer) -> Self {
        i.into()
    }
}

impl From<&Integer> for Vec<u8> {
    #[inline]
    fn from(i: &Integer) -> Self {
        i.to_string().into_bytes()
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
    #[inline]
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
    #[inline]
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
    type Err = IntegerError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "k" | "K" => Ok(Self::Kibi),
            "m" | "M" => Ok(Self::Mebi),
            "g" | "G" => Ok(Self::Gibi),
            _ => Err(IntegerError::InvalidSuffix),
        }
    }
}

impl TryFrom<&[u8]> for IntegerSuffix {
    type Error = IntegerError;

    #[inline]
    fn try_from(s: &[u8]) -> Result<Self, Self::Error> {
        Self::from_str(std::str::from_utf8(s)?)
    }
}

impl TryFrom<Vec<u8>> for IntegerSuffix {
    type Error = IntegerError;

    #[inline]
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
    #[inline]
    #[must_use]
    pub fn to_vec(&self) -> Vec<u8> {
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

quick_error! {
    #[derive(Debug, PartialEq)]
    ///
    #[allow(missing_docs)]
    pub enum ColorError {
        Utf8Conversion(err: std::str::Utf8Error) {
            display("Ill-formed UTF-8")
            source(err)
            from()
        }
        InvalidColorItem {
            display("Invalid color item")
        }
        InvalidFormat {
            display("Invalid argument format")
        }
    }
}

impl TryFrom<&[u8]> for Color {
    type Error = ColorError;

    #[inline]
    fn try_from(s: &[u8]) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(s)?;
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
                            return Err(ColorError::InvalidColorItem);
                        }
                    }
                    ColorItem::Attr(a) => new_self.attributes.push(a),
                },
                Err(_) => return Err(ColorError::InvalidColorItem),
            }
        }

        Ok(new_self)
    }
}

impl TryFrom<Vec<u8>> for Color {
    type Error = ColorError;

    #[inline]
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(value.as_ref())
    }
}

impl TryFrom<Cow<'_, [u8]>> for Color {
    type Error = ColorError;

    #[inline]
    fn try_from(c: Cow<'_, [u8]>) -> Result<Self, Self::Error> {
        match c {
            Cow::Borrowed(c) => Self::try_from(c),
            Cow::Owned(c) => Self::try_from(c),
        }
    }
}

impl From<Color> for Vec<u8> {
    #[inline]
    fn from(c: Color) -> Self {
        c.into()
    }
}

impl From<&Color> for Vec<u8> {
    #[inline]
    fn from(c: &Color) -> Self {
        c.to_string().into_bytes()
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
    type Err = ColorError;

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
            "normal" if bright => return Err(ColorError::InvalidFormat),
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

        Err(ColorError::InvalidFormat)
    }
}

impl TryFrom<&[u8]> for ColorValue {
    type Error = ColorError;

    #[inline]
    fn try_from(s: &[u8]) -> Result<Self, Self::Error> {
        Self::from_str(std::str::from_utf8(s)?)
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
    type Err = ColorError;

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
            _ => Err(ColorError::InvalidFormat),
        }
    }
}

impl TryFrom<&[u8]> for ColorAttribute {
    type Error = ColorError;

    #[inline]
    fn try_from(s: &[u8]) -> Result<Self, Self::Error> {
        Self::from_str(std::str::from_utf8(s)?)
    }
}

quick_error! {
    #[derive(Debug, PartialEq)]
    ///
    #[allow(missing_docs)]
    pub enum ExpiryDateError {
        Utf8Conversion(err: bstr::Utf8Error) {
            display("Ill-formed UTF-8")
            source(err)
            from()
        }
        UnsupportedFormat {
            display("Could not parse expiry date")
        }
    }
}

#[cfg(feature = "expiry-date")]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ExpiryDate<'a> {
    pub value: Cow<'a, BStr>,
}

#[cfg(feature = "expiry-date")]
impl<'a> From<Cow<'a, [u8]>> for ExpiryDate<'a> {
    fn from(c: Cow<'a, [u8]>) -> Self {
        Self {
            value: match c {
                Cow::Borrowed(c) => Cow::Borrowed(c.into()),
                Cow::Owned(c) => Cow::Owned(c.into()),
            },
        }
    }
}

#[cfg(feature = "expiry-date")]
impl<'a> ExpiryDate<'a> {
    pub fn to_timestamp(self) -> Result<u64, ExpiryDateError> {
        let v = self.value.to_str()?;

        if v == "never" || v == "false" {
            return Ok(0);
        }

        if v == "all" || v == "now" {
            return Ok(u64::MAX);
        }

        let tz_format_descriptions = [
            // rfc2822: Fri, 4 Jun 2010 15:46:55 +0400
            &format_description!("[weekday repr:short], [day padding:none] [month repr:short] [year] [hour]:[minute]:[second] [offset_hour sign:mandatory][offset_minute]"),
            // iso8601: 2006-07-03 17:18:43 +0200 
            &format_description!("[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour sign:mandatory][offset_minute]"),
            // local with tz: Fri Jun 4 15:46:55 2010 +0300
            &format_description!("[weekday repr:short] [month repr:short] [day padding:none] [hour]:[minute]:[second] [year] [offset_hour sign:mandatory][offset_minute]"),
        ];

        for descr in tz_format_descriptions {
            if let Ok(date) = OffsetDateTime::parse(v, descr) {
                return Ok(date.unix_timestamp() as u64);
            }
        }

        let notz_format_descriptions = [
            // local: Fri Jun 4 15:46:55 2010
            &format_description!(
                "[weekday repr:short] [month repr:short] [day padding:none] [hour]:[minute]:[second] [year]"
            ),
            // 2017/11/11 11:11:11PM
            &format_description!("[year]/[month]/[day] [hour repr:12]:[minute]:[second][period case_sensitive:false]"),
            // 2017/11/11 11:11:11 PM
            &format_description!("[year]/[month]/[day] [hour repr:12]:[minute]:[second] [period case_sensitive:false]"),
        ];

        for descr in notz_format_descriptions {
            if let Ok(date) = PrimitiveDateTime::parse(v, descr) {
                return Ok(date.assume_utc().unix_timestamp() as u64);
            }
        }

        Err(ExpiryDateError::UnsupportedFormat)
    }
}
