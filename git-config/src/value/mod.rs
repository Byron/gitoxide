mod normalize;
pub use normalize::{normalize, normalize_bstr, normalize_bstring};

mod string;
pub use string::String;

/// Any value that may contain a foreground color, background color, a
/// collection of color (text) modifiers, or a combination of any of the
/// aforementioned values.
///
/// Note that `git-config` allows color values to simply be a collection of
/// [`color::Attribute`]s, and does not require a [`color::Name`] for either the
/// foreground or background color.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Color {
    /// A provided foreground color
    pub foreground: Option<color::Name>,
    /// A provided background color
    pub background: Option<color::Name>,
    /// A potentially empty list of text attributes
    pub attributes: Vec<color::Attribute>,
}
pub mod color;

/// Any value that can be interpreted as an integer.
///
/// This supports any numeric value that can fit in a [`i64`], excluding the
/// suffix. The suffix is parsed separately from the value itself, so if you
/// wish to obtain the true value of the integer, you must account for the
/// suffix after fetching the value. [`integer::Suffix`] provides
/// [`bitwise_offset`] to help with the math, but do be warned that if the value
/// is very large, you may run into overflows.
///
/// [`BStr`]: bstr::BStr
/// [`bitwise_offset`]: integer::Suffix::bitwise_offset
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Integer {
    /// The value, without any suffix modification
    pub value: i64,
    /// A provided suffix, if any.
    pub suffix: Option<integer::Suffix>,
}
///
pub mod integer;

/// Any value that can be interpreted as a boolean.
///
/// Note that while values can effectively be any byte string, the `git-config`
/// documentation has a strict subset of values that may be interpreted as a
/// boolean value, all of which are ASCII and thus UTF-8 representable.
/// Consequently, variants hold [`str`]s rather than [`[u8]`]s.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[allow(missing_docs)]
pub enum Boolean<'a> {
    True(boolean::True<'a>),
    False(std::borrow::Cow<'a, bstr::BStr>),
}
///
pub mod boolean;

/// Any value that can be interpreted as a file path.
///
/// Git represents file paths as byte arrays, modeled here as owned or borrowed byte sequences.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Path<'a> {
    /// The path string, un-interpolated
    pub value: std::borrow::Cow<'a, bstr::BStr>,
}
///
pub mod path;

pub mod parse {
    use bstr::BString;

    /// The error returned when creating `Integer` from byte string.
    #[derive(Debug, thiserror::Error, Eq, PartialEq)]
    #[allow(missing_docs)]
    #[error("Could not decode '{}': {}", .input, .message)]
    pub struct Error {
        pub message: &'static str,
        pub input: BString,
        #[source]
        pub utf8_err: Option<std::str::Utf8Error>,
    }

    impl Error {
        pub(crate) fn new(message: &'static str, input: impl Into<BString>) -> Self {
            Error {
                message,
                input: input.into(),
                utf8_err: None,
            }
        }

        pub(crate) fn with_err(mut self, err: std::str::Utf8Error) -> Self {
            self.utf8_err = Some(err);
            self
        }
    }
}
