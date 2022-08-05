//! Parse [ref specs]() and represent them.
#![forbid(unsafe_code, rust_2018_idioms)]
#![allow(missing_docs)]

use bstr::{BStr, BString};

/// The way to interpret a refspec.
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub enum Mode {
    /// Apply standard rules for refspecs which are including refs with specific rules related to allowing fast forwards of destinations.
    Normal,
    /// Even though according to normal rules a non-fastforward would be denied, override this and reset a ref forcefully in the destination.
    Force,
    /// Instead of considering matching refs included, we consider them excluded. This applies only to the source side of a refspec.
    Negative,
}

/// What operation to perform with the refspec.
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub enum Operation {
    /// The `src` side is local and the `dst` side is remote.
    Push,
    /// The `src` side is remote and the `dst` side is local.
    Fetch,
}

/// A refspec with references to the memory it was parsed from.
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub struct RefSpecRef<'a> {
    mode: Mode,
    op: Operation,
    src: Option<&'a BStr>,
    dest: Option<&'a BStr>,
}

/// An owned refspec.
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct RefSpec {
    mode: Mode,
    op: Operation,
    src: Option<BString>,
    dest: Option<BString>,
}

pub mod parse;
pub use parse::function::parse;

mod spec {
    use crate::{RefSpec, RefSpecRef};

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
