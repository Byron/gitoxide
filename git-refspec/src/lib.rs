//! Parse git ref-specs and represent them.
#![forbid(unsafe_code, rust_2018_idioms)]
#![deny(missing_docs)]

///
pub mod parse;
pub use parse::function::parse;

///
pub mod instruction;

/// A refspec with references to the memory it was parsed from.
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub struct RefSpecRef<'a> {
    mode: types::Mode,
    op: parse::Operation,
    src: Option<&'a bstr::BStr>,
    dst: Option<&'a bstr::BStr>,
}

/// An owned refspec.
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct RefSpec {
    mode: types::Mode,
    op: parse::Operation,
    src: Option<bstr::BString>,
    dst: Option<bstr::BString>,
}

mod spec;

mod types;
pub use types::Instruction;
