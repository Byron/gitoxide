///
pub mod convert_to_git;
pub use convert_to_git::function::convert_to_git;

mod convert_to_worktree;
pub use convert_to_worktree::convert_to_worktree;

mod utils;

/// The kind of end of lines to set.
///
/// The default is implemented to be the native line ending for the current platform.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Mode {
    /// Equivalent to `git` (`\n`) line-endings.
    Lf,
    /// Equivalent to `windows` (`\r\n`) line-endings.
    CrLf,
}

/// Possible states for the `core.autocrlf`.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub enum AutoCrlf {
    /// The same as if the `text eol=lf` attribute is set.
    Input,
    /// The same as if the `text eol=crlf` attribute is set.
    Enabled,
    /// No conversion is performed.
    #[default]
    Disabled,
}

/// The combination of `crlf`, `text` and `eol` attributes into one neat package.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AttributesDigest {
    /// Equivalent to the `-text` attribute.
    Binary,
    /// Equivalent to the `text` attribute.
    Text,
    /// Equivalent to the `text eol=lf` attributes.
    TextInput,
    /// Equivalent to the `text eol=crlf` attributes.
    TextCrlf,
    /// Equivalent to the `text=auto` attributes.
    TextAuto,
    /// Equivalent to the `text=auto eol=crlf` attributes.
    TextAutoCrlf,
    /// Equivalent to the `text=auto eol=lf` attributes.
    TextAutoInput,
}

impl From<Mode> for AttributesDigest {
    fn from(value: Mode) -> Self {
        match value {
            Mode::Lf => AttributesDigest::TextInput,
            Mode::CrLf => AttributesDigest::TextCrlf,
        }
    }
}

impl From<AutoCrlf> for AttributesDigest {
    fn from(value: AutoCrlf) -> Self {
        match value {
            AutoCrlf::Input => AttributesDigest::TextAutoInput,
            AutoCrlf::Enabled => AttributesDigest::TextAutoCrlf,
            AutoCrlf::Disabled => AttributesDigest::Binary,
        }
    }
}

/// Git Configuration that affects how CRLF conversions are applied.
#[derive(Default, Debug, Copy, Clone)]
pub struct Configuration {
    /// Corresponds to `core.autocrlf`.
    pub auto_crlf: AutoCrlf,
    /// Corresponds to `core.eol`, and is `None` if unset or set to `native`, or `Some(<mode>)` respectively.
    pub eol: Option<Mode>,
}

/// Statistics about a buffer that helps to safely perform EOL conversions
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Stats {
    /// The amount of null bytes.
    pub null: usize,
    /// The amount of lone carriage returns (`\r`).
    pub lone_cr: usize,
    /// The amount of lone line feeds (`\n`).
    pub lone_lf: usize,
    /// The amount carriage returns followed by line feeds
    pub crlf: usize,
    /// The estimate of printable characters.
    pub printable: usize,
    /// The estimate of characters that can't be printed.
    pub non_printable: usize,
}
