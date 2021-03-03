//! Rust containers for valid `git-config` types.

#[cfg(feature = "serde")]
use serde::{Serialize, Serializer};
use std::borrow::Cow;
use std::convert::TryFrom;
use std::fmt::Display;
use std::str::FromStr;

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
/// assert_eq!(normalize_str("hello world"), Cow::Borrowed(b"hello world".into()));
/// ```
///
/// Fully quoted values are optimized to not need allocations.
///
/// ```
/// # use std::borrow::Cow;
/// # use git_config::values::normalize_str;
/// assert_eq!(normalize_str("\"hello world\""), Cow::Borrowed(b"hello world".into()));
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
                    owned.extend(dbg!(&input[last_index..i - 1]));
                    last_index = i;
                } else {
                    owned.extend(dbg!(&input[first_index..i - 1]));
                    first_index = i;
                }
            }
            continue;
        }

        if *c == b'\\' {
            was_escaped = true;
        } else if *c == b'"' {
            if first_index == 0 {
                owned.extend(dbg!(&input[last_index..i]));
                first_index = i + 1;
            } else {
                owned.extend(dbg!(&input[first_index..i]));
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
pub fn normalize_bytes(input: &[u8]) -> Cow<'_, [u8]> {
    normalize_cow(Cow::Borrowed(input))
}

/// `Vec[u8]` variant of [`normalize_cow`].
#[inline]
pub fn normalize_vec(input: Vec<u8>) -> Cow<'static, [u8]> {
    normalize_cow(Cow::Owned(input))
}

/// [`str`] variant of [`normalize_cow`].
#[inline]
pub fn normalize_str(input: &str) -> Cow<'_, [u8]> {
    normalize_bytes(input.as_bytes())
}

/// Fully enumerated valid types that a `git-config` value can be.
#[allow(missing_docs)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Value<'a> {
    Boolean(Boolean<'a>),
    Integer(Integer),
    Color(Color),
    /// If a value does not match from any of the other variants, then this
    /// variant will be matched. As a result, conversion from a `str`-like item
    /// will never fail.
    Other(Cow<'a, [u8]>),
}

impl<'a> From<&'a str> for Value<'a> {
    fn from(s: &'a str) -> Self {
        if let Ok(bool) = Boolean::try_from(s) {
            return Self::Boolean(bool);
        }

        if let Ok(int) = Integer::from_str(s) {
            return Self::Integer(int);
        }

        if let Ok(color) = Color::from_str(s) {
            return Self::Color(color);
        }

        Self::Other(Cow::Borrowed(s.as_bytes()))
    }
}

impl<'a> From<&'a [u8]> for Value<'a> {
    fn from(s: &'a [u8]) -> Self {
        // All parsable values must be utf-8 valid
        if let Ok(s) = std::str::from_utf8(s) {
            Self::from(s)
        } else {
            Self::Other(Cow::Borrowed(s))
        }
    }
}

impl From<String> for Value<'_> {
    fn from(s: String) -> Self {
        Self::from(s.into_bytes())
    }
}

impl From<Vec<u8>> for Value<'_> {
    fn from(s: Vec<u8>) -> Self {
        if let Ok(int) = Integer::try_from(s.as_ref()) {
            return Self::Integer(int);
        }

        if let Ok(color) = Color::try_from(s.as_ref()) {
            return Self::Color(color);
        }

        Boolean::try_from(s).map_or_else(|v| Self::Other(Cow::Owned(v)), Self::Boolean)
    }
}

impl<'a> From<Cow<'a, [u8]>> for Value<'a> {
    fn from(c: Cow<'a, [u8]>) -> Self {
        match c {
            Cow::Borrowed(c) => Self::from(c),
            Cow::Owned(c) => Self::from(c),
        }
    }
}

// todo display for value

#[cfg(feature = "serde")]
impl Serialize for Value<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Value::Boolean(b) => b.serialize(serializer),
            Value::Integer(i) => i.serialize(serializer),
            Value::Color(c) => c.serialize(serializer),
            Value::Other(i) => i.serialize(serializer),
        }
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

impl<'a> TryFrom<&'a str> for Boolean<'a> {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::try_from(value.as_bytes())
    }
}

impl<'a> TryFrom<&'a [u8]> for Boolean<'a> {
    type Error = ();

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
            return Ok(Self::False(std::str::from_utf8(value).unwrap().into()));
        }

        Err(())
    }
}

impl TryFrom<String> for Boolean<'_> {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.into_bytes()).map_err(|v| String::from_utf8(v).unwrap())
    }
}

impl TryFrom<Vec<u8>> for Boolean<'_> {
    type Error = Vec<u8>;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.eq_ignore_ascii_case(b"no")
            || value.eq_ignore_ascii_case(b"off")
            || value.eq_ignore_ascii_case(b"false")
            || value.eq_ignore_ascii_case(b"zero")
            || value == b"\"\""
        {
            return Ok(Self::False(Cow::Owned(String::from_utf8(value).unwrap())));
        }

        TrueVariant::try_from(value).map(Self::True)
    }
}

impl<'a> TryFrom<Cow<'a, [u8]>> for Boolean<'a> {
    type Error = ();
    fn try_from(c: Cow<'a, [u8]>) -> Result<Self, Self::Error> {
        match c {
            Cow::Borrowed(c) => Self::try_from(c),
            Cow::Owned(c) => Self::try_from(c).map_err(|_| ()),
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

impl Into<bool> for Boolean<'_> {
    fn into(self) -> bool {
        match self {
            Boolean::True(_) => true,
            Boolean::False(_) => false,
        }
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

impl<'a> TryFrom<&'a str> for TrueVariant<'a> {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::try_from(value.as_bytes())
    }
}

impl<'a> TryFrom<&'a [u8]> for TrueVariant<'a> {
    type Error = ();

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        if value.eq_ignore_ascii_case(b"yes")
            || value.eq_ignore_ascii_case(b"on")
            || value.eq_ignore_ascii_case(b"true")
            || value.eq_ignore_ascii_case(b"one")
        {
            Ok(Self::Explicit(std::str::from_utf8(value).unwrap().into()))
        } else if value.is_empty() {
            Ok(Self::Implicit)
        } else {
            Err(())
        }
    }
}

impl TryFrom<String> for TrueVariant<'_> {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.into_bytes()).map_err(|v| String::from_utf8(v).unwrap())
    }
}

impl TryFrom<Vec<u8>> for TrueVariant<'_> {
    type Error = Vec<u8>;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.eq_ignore_ascii_case(b"yes")
            || value.eq_ignore_ascii_case(b"on")
            || value.eq_ignore_ascii_case(b"true")
            || value.eq_ignore_ascii_case(b"one")
        {
            Ok(Self::Explicit(Cow::Owned(
                String::from_utf8(value).unwrap(),
            )))
        } else if value.is_empty() {
            Ok(Self::Implicit)
        } else {
            Err(value)
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

impl FromStr for Integer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(value) = s.parse() {
            return Ok(Self {
                value,
                suffix: None,
            });
        }

        // Assume we have a prefix at this point.

        if s.len() <= 1 {
            return Err(s.to_string());
        }

        let (number, suffix) = s.split_at(s.len() - 1);
        if let (Ok(value), Ok(suffix)) = (number.parse(), suffix.parse()) {
            Ok(Self {
                value,
                suffix: Some(suffix),
            })
        } else {
            Err(s.to_string())
        }
    }
}

impl TryFrom<&[u8]> for Integer {
    type Error = ();

    fn try_from(s: &[u8]) -> Result<Self, Self::Error> {
        Self::from_str(std::str::from_utf8(s).map_err(|_| ())?).map_err(|_| ())
    }
}

impl TryFrom<Vec<u8>> for Integer {
    type Error = ();

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(value.as_ref())
    }
}

impl TryFrom<Cow<'_, [u8]>> for Integer {
    type Error = ();
    fn try_from(c: Cow<'_, [u8]>) -> Result<Self, Self::Error> {
        match c {
            Cow::Borrowed(c) => Self::try_from(c),
            Cow::Owned(c) => Self::try_from(c).map_err(|_| ()),
        }
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
            "k" => Ok(Self::Kibi),
            "m" => Ok(Self::Mebi),
            "g" => Ok(Self::Gibi),
            _ => Err(()),
        }
    }
}

impl TryFrom<&[u8]> for IntegerSuffix {
    type Error = ();

    fn try_from(s: &[u8]) -> Result<Self, Self::Error> {
        Self::from_str(std::str::from_utf8(s).map_err(|_| ())?).map_err(|_| ())
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

/// Discriminating enum for [`Color`] parsing.
pub enum ColorParseError {
    /// Too many primary colors were provided.
    TooManyColorValues,
    /// An invalid color value or attribute was provided.
    InvalidColorOption,
}

impl FromStr for Color {
    type Err = ColorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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
                            return Err(ColorParseError::TooManyColorValues);
                        }
                    }
                    ColorItem::Attr(a) => new_self.attributes.push(a),
                },
                Err(_) => return Err(ColorParseError::InvalidColorOption),
            }
        }

        Ok(new_self)
    }
}

impl TryFrom<&[u8]> for Color {
    type Error = ();

    fn try_from(s: &[u8]) -> Result<Self, Self::Error> {
        Self::from_str(std::str::from_utf8(s).map_err(|_| ())?).map_err(|_| ())
    }
}

impl TryFrom<Vec<u8>> for Color {
    type Error = ();

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(value.as_ref())
    }
}

impl TryFrom<Cow<'_, [u8]>> for Color {
    type Error = ();
    fn try_from(c: Cow<'_, [u8]>) -> Result<Self, Self::Error> {
        match c {
            Cow::Borrowed(c) => Self::try_from(c),
            Cow::Owned(c) => Self::try_from(c).map_err(|_| ()),
        }
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
    type Err = ();

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
            "normal" if bright => return Err(()),
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

        Err(())
    }
}

impl TryFrom<&[u8]> for ColorValue {
    type Error = ();

    fn try_from(s: &[u8]) -> Result<Self, Self::Error> {
        Self::from_str(std::str::from_utf8(s).map_err(|_| ())?).map_err(|_| ())
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
    type Err = ();

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
            _ => Err(()),
        }
    }
}

impl TryFrom<&[u8]> for ColorAttribute {
    type Error = ();

    fn try_from(s: &[u8]) -> Result<Self, Self::Error> {
        Self::from_str(std::str::from_utf8(s).map_err(|_| ())?).map_err(|_| ())
    }
}

#[cfg(test)]
mod normalize {
    use super::normalize_str;
    use std::borrow::Cow;

    #[test]
    fn not_modified_is_borrowed() {
        assert_eq!(normalize_str("hello world"), Cow::Borrowed(b"hello world"));
    }

    #[test]
    fn modified_is_owned() {
        assert_eq!(
            normalize_str("hello \"world\""),
            Cow::<[u8]>::Owned(b"hello world".to_vec())
        );
    }

    #[test]
    fn all_quoted_is_optimized() {
        assert_eq!(
            normalize_str("\"hello world\""),
            Cow::Borrowed(b"hello world")
        );
    }

    #[test]
    fn all_quote_optimization_is_correct() {
        assert_eq!(
            normalize_str(r#""hello" world\""#),
            Cow::Borrowed(b"hello world\"")
        );
    }

    #[test]
    fn quotes_right_next_to_each_other() {
        assert_eq!(
            normalize_str("\"hello\"\" world\""),
            Cow::<[u8]>::Owned(b"hello world".to_vec())
        );
    }

    #[test]
    fn escaped_quotes_are_kept() {
        assert_eq!(
            normalize_str(r#""hello \"\" world""#),
            Cow::<[u8]>::Owned(b"hello \"\" world".to_vec())
        );
    }

    #[test]
    fn empty_string() {
        assert_eq!(normalize_str(""), Cow::Borrowed(b""));
    }

    #[test]
    fn empty_normalized_string_is_optimized() {
        assert_eq!(normalize_str("\"\""), Cow::Borrowed(b""));
    }
}

#[cfg(test)]
mod boolean {
    use super::*;

    #[test]
    fn from_str_false() {
        assert_eq!(Boolean::try_from("no"), Ok(Boolean::False("no".into())));
        assert_eq!(Boolean::try_from("off"), Ok(Boolean::False("off".into())));
        assert_eq!(
            Boolean::try_from("false"),
            Ok(Boolean::False("false".into()))
        );
        assert_eq!(Boolean::try_from("zero"), Ok(Boolean::False("zero".into())));
        assert_eq!(Boolean::try_from("\"\""), Ok(Boolean::False("\"\"".into())));
    }

    #[test]
    fn from_str_true() {
        assert_eq!(
            Boolean::try_from("yes"),
            Ok(Boolean::True(TrueVariant::Explicit("yes".into())))
        );
        assert_eq!(
            Boolean::try_from("on"),
            Ok(Boolean::True(TrueVariant::Explicit("on".into())))
        );
        assert_eq!(
            Boolean::try_from("true"),
            Ok(Boolean::True(TrueVariant::Explicit("true".into())))
        );
        assert_eq!(
            Boolean::try_from("one"),
            Ok(Boolean::True(TrueVariant::Explicit("one".into())))
        );
    }

    #[test]
    fn ignores_case() {
        // Random subset
        for word in &["no", "yes", "off", "true", "zero"] {
            let first: bool = Boolean::try_from(*word).unwrap().into();
            let second: bool = Boolean::try_from(&*word.to_uppercase()).unwrap().into();
            assert_eq!(first, second);
        }
    }

    #[test]
    fn from_str_err() {
        assert!(Boolean::try_from("yesn't").is_err());
        assert!(Boolean::try_from("yesno").is_err());
    }
}

#[cfg(test)]
mod integer {
    use super::*;

    #[test]
    fn from_str_no_suffix() {
        assert_eq!(
            Integer::from_str("1").unwrap(),
            Integer {
                value: 1,
                suffix: None
            }
        );

        assert_eq!(
            Integer::from_str("-1").unwrap(),
            Integer {
                value: -1,
                suffix: None
            }
        );
    }

    #[test]
    fn from_str_with_suffix() {
        assert_eq!(
            Integer::from_str("1k").unwrap(),
            Integer {
                value: 1,
                suffix: Some(IntegerSuffix::Kibi),
            }
        );

        assert_eq!(
            Integer::from_str("1m").unwrap(),
            Integer {
                value: 1,
                suffix: Some(IntegerSuffix::Mebi),
            }
        );

        assert_eq!(
            Integer::from_str("1g").unwrap(),
            Integer {
                value: 1,
                suffix: Some(IntegerSuffix::Gibi),
            }
        );
    }

    #[test]
    fn invalid_from_str() {
        assert!(Integer::from_str("").is_err());
        assert!(Integer::from_str("-").is_err());
        assert!(Integer::from_str("k").is_err());
        assert!(Integer::from_str("m").is_err());
        assert!(Integer::from_str("g").is_err());
        assert!(Integer::from_str("123123123123123123123123").is_err());
        assert!(Integer::from_str("gg").is_err());
    }
}

#[cfg(test)]
mod color_value {
    use super::ColorValue;
    use std::str::FromStr;

    #[test]
    fn non_bright() {
        assert_eq!(ColorValue::from_str("normal"), Ok(ColorValue::Normal));
        assert_eq!(ColorValue::from_str("black"), Ok(ColorValue::Black));
        assert_eq!(ColorValue::from_str("red"), Ok(ColorValue::Red));
        assert_eq!(ColorValue::from_str("green"), Ok(ColorValue::Green));
        assert_eq!(ColorValue::from_str("yellow"), Ok(ColorValue::Yellow));
        assert_eq!(ColorValue::from_str("blue"), Ok(ColorValue::Blue));
        assert_eq!(ColorValue::from_str("magenta"), Ok(ColorValue::Magenta));
        assert_eq!(ColorValue::from_str("cyan"), Ok(ColorValue::Cyan));
        assert_eq!(ColorValue::from_str("white"), Ok(ColorValue::White));
    }

    #[test]
    fn bright() {
        assert_eq!(
            ColorValue::from_str("brightblack"),
            Ok(ColorValue::BrightBlack)
        );
        assert_eq!(ColorValue::from_str("brightred"), Ok(ColorValue::BrightRed));
        assert_eq!(
            ColorValue::from_str("brightgreen"),
            Ok(ColorValue::BrightGreen)
        );
        assert_eq!(
            ColorValue::from_str("brightyellow"),
            Ok(ColorValue::BrightYellow)
        );
        assert_eq!(
            ColorValue::from_str("brightblue"),
            Ok(ColorValue::BrightBlue)
        );
        assert_eq!(
            ColorValue::from_str("brightmagenta"),
            Ok(ColorValue::BrightMagenta)
        );
        assert_eq!(
            ColorValue::from_str("brightcyan"),
            Ok(ColorValue::BrightCyan)
        );
        assert_eq!(
            ColorValue::from_str("brightwhite"),
            Ok(ColorValue::BrightWhite)
        );
    }

    #[test]
    fn ansi() {
        assert_eq!(ColorValue::from_str("255"), Ok(ColorValue::Ansi(255)));
        assert_eq!(ColorValue::from_str("0"), Ok(ColorValue::Ansi(0)));
    }

    #[test]
    fn hex() {
        assert_eq!(
            ColorValue::from_str("#ff0010"),
            Ok(ColorValue::Rgb(255, 0, 16))
        );
        assert_eq!(
            ColorValue::from_str("#ffffff"),
            Ok(ColorValue::Rgb(255, 255, 255))
        );
        assert_eq!(
            ColorValue::from_str("#000000"),
            Ok(ColorValue::Rgb(0, 0, 0))
        );
    }

    #[test]
    fn invalid() {
        assert!(ColorValue::from_str("brightnormal").is_err());
        assert!(ColorValue::from_str("").is_err());
        assert!(ColorValue::from_str("bright").is_err());
        assert!(ColorValue::from_str("256").is_err());
        assert!(ColorValue::from_str("#").is_err());
        assert!(ColorValue::from_str("#fff").is_err());
        assert!(ColorValue::from_str("#gggggg").is_err());
    }
}

#[cfg(test)]
mod color_attribute {
    use super::ColorAttribute;
    use std::str::FromStr;

    #[test]
    fn non_inverted() {
        assert_eq!(ColorAttribute::from_str("bold"), Ok(ColorAttribute::Bold));
        assert_eq!(ColorAttribute::from_str("dim"), Ok(ColorAttribute::Dim));
        assert_eq!(ColorAttribute::from_str("ul"), Ok(ColorAttribute::Ul));
        assert_eq!(ColorAttribute::from_str("blink"), Ok(ColorAttribute::Blink));
        assert_eq!(
            ColorAttribute::from_str("reverse"),
            Ok(ColorAttribute::Reverse)
        );
        assert_eq!(
            ColorAttribute::from_str("italic"),
            Ok(ColorAttribute::Italic)
        );
        assert_eq!(
            ColorAttribute::from_str("strike"),
            Ok(ColorAttribute::Strike)
        );
    }

    #[test]
    fn inverted_no_dash() {
        assert_eq!(
            ColorAttribute::from_str("nobold"),
            Ok(ColorAttribute::NoBold)
        );
        assert_eq!(ColorAttribute::from_str("nodim"), Ok(ColorAttribute::NoDim));
        assert_eq!(ColorAttribute::from_str("noul"), Ok(ColorAttribute::NoUl));
        assert_eq!(
            ColorAttribute::from_str("noblink"),
            Ok(ColorAttribute::NoBlink)
        );
        assert_eq!(
            ColorAttribute::from_str("noreverse"),
            Ok(ColorAttribute::NoReverse)
        );
        assert_eq!(
            ColorAttribute::from_str("noitalic"),
            Ok(ColorAttribute::NoItalic)
        );
        assert_eq!(
            ColorAttribute::from_str("nostrike"),
            Ok(ColorAttribute::NoStrike)
        );
    }

    #[test]
    fn inverted_dashed() {
        assert_eq!(
            ColorAttribute::from_str("no-bold"),
            Ok(ColorAttribute::NoBold)
        );
        assert_eq!(
            ColorAttribute::from_str("no-dim"),
            Ok(ColorAttribute::NoDim)
        );
        assert_eq!(ColorAttribute::from_str("no-ul"), Ok(ColorAttribute::NoUl));
        assert_eq!(
            ColorAttribute::from_str("no-blink"),
            Ok(ColorAttribute::NoBlink)
        );
        assert_eq!(
            ColorAttribute::from_str("no-reverse"),
            Ok(ColorAttribute::NoReverse)
        );
        assert_eq!(
            ColorAttribute::from_str("no-italic"),
            Ok(ColorAttribute::NoItalic)
        );
        assert_eq!(
            ColorAttribute::from_str("no-strike"),
            Ok(ColorAttribute::NoStrike)
        );
    }

    #[test]
    fn invalid() {
        assert!(ColorAttribute::from_str("a").is_err());
        assert!(ColorAttribute::from_str("no bold").is_err());
        assert!(ColorAttribute::from_str("").is_err());
        assert!(ColorAttribute::from_str("no").is_err());
        assert!(ColorAttribute::from_str("no-").is_err());
    }
}
