//! Parse [ref specs]() and represent them.
#![forbid(unsafe_code, rust_2018_idioms)]
#![allow(missing_docs)]

pub mod parse;
pub use parse::function::parse;

/// A refspec with references to the memory it was parsed from.
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub struct RefSpecRef<'a> {
    mode: Mode,
    op: Operation,
    src: Option<&'a bstr::BStr>,
    dest: Option<&'a bstr::BStr>,
}

/// An owned refspec.
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct RefSpec {
    mode: Mode,
    op: Operation,
    src: Option<bstr::BString>,
    dest: Option<bstr::BString>,
}

mod types;
pub use types::{Instruction, Mode, Operation};

mod spec {
    use crate::{Instruction, Mode, RefSpec, RefSpecRef};

    /// Access
    impl RefSpecRef<'_> {
        /// Return the refspec mode.
        pub fn mode(&self) -> Mode {
            self.mode
        }

        /// Transform the state of the refspec into an instruction making clear what to do with it.
        pub fn instruction(&self) -> Instruction<'_> {
            todo!()
        }
    }

    /// Conversion
    impl RefSpecRef<'_> {
        /// Convert this ref into a standalone, owned copy.
        pub fn to_owned(&self) -> RefSpec {
            RefSpec {
                mode: self.mode,
                op: self.op,
                src: self.src.map(ToOwned::to_owned),
                dest: self.dest.map(ToOwned::to_owned),
            }
        }
    }
}
