mod string;
pub use string::String;

mod color;
pub use color::{Color, ColorAttribute, ColorValue};

mod integer;
pub use integer::{Integer, IntegerSuffix};

mod boolean;
pub use boolean::{Boolean, TrueVariant};

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
