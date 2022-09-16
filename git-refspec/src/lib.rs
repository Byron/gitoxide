//! Parse git ref-specs and represent them.
#![deny(missing_docs, rust_2018_idioms)]
#![forbid(unsafe_code)]

///
pub mod parse;
pub use parse::function::parse;

///
pub mod instruction;

/// A refspec with references to the memory it was parsed from.
#[derive(Eq, Copy, Clone, Debug)]
pub struct RefSpecRef<'a> {
    mode: types::Mode,
    op: parse::Operation,
    src: Option<&'a bstr::BStr>,
    dst: Option<&'a bstr::BStr>,
}

/// An owned refspec.
#[derive(Eq, Clone, Debug)]
pub struct RefSpec {
    mode: types::Mode,
    op: parse::Operation,
    src: Option<bstr::BString>,
    dst: Option<bstr::BString>,
}

mod spec;

mod write;

///
pub mod match_group;
pub use match_group::types::MatchGroup;

mod types;
pub use types::Instruction;
