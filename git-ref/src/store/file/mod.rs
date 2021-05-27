use std::path::PathBuf;

/// A git _ref_ which is stored in a file.
#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Clone)]
pub struct Reference<'a> {
    parent: &'a Store,
    /// The path relative to the stores base at which this reference is located
    pub relative_path: PathBuf,
    state: reference::State,
}

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Clone)]
pub struct Store {
    /// The location at which loose references can be found as per conventions of a typical git repository
    pub base: PathBuf,
}

mod loose;
pub use loose::*;
pub mod reference;
