#![forbid(unsafe_code)]

/// For convenience to allow using `bstr` without adding it to own cargo manifest
pub use bstr;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Protocol {
    Ssh,
}

pub mod borrowed;
#[doc(inline)]
pub use borrowed::Url as Borrowed;

pub mod parse;
#[doc(inline)]
pub use parse::parse;
