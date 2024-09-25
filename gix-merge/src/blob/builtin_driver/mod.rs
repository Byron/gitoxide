use crate::blob::BuiltinDriver;

impl BuiltinDriver {
    /// Return the name of this instance.
    pub fn as_str(&self) -> &str {
        match self {
            BuiltinDriver::Text => "text",
            BuiltinDriver::Binary => "binary",
            BuiltinDriver::Union => "union",
        }
    }

    /// Get all available built-in drivers.
    pub fn all() -> &'static [Self] {
        &[BuiltinDriver::Text, BuiltinDriver::Binary, BuiltinDriver::Union]
    }

    /// Try to match one of our variants to `name`, case-sensitive, and return its instance.
    pub fn by_name(name: &str) -> Option<Self> {
        Self::all().iter().find(|variant| variant.as_str() == name).copied()
    }
}

///
pub mod binary;
pub use binary::function::merge as binary;

///
pub mod text;
pub use text::function::merge as text;
