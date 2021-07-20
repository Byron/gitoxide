use crate::{file::Reference, Kind};

impl<'a> Reference<'a> {
    /// Return the kind of ref.
    pub fn kind(&self) -> Kind {
        self.target.kind()
    }
}

mod logiter;

///
pub mod peel;

///
pub mod decode;
