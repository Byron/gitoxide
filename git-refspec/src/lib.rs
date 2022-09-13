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

///
pub mod matcher {
    use bstr::BStr;
    use git_hash::oid;

    /// An item to match
    pub struct Item<'a> {
        /// The full name of the references, like `refs/heads/main`
        pub full_ref_name: &'a BStr,
        /// The peeled id it points to that we should match against.
        pub target: &'a oid,
        /// The tag object's id if this is a tag
        pub tag: Option<&'a oid>,
    }
}

mod types;
pub use types::{Instruction, Matcher};
