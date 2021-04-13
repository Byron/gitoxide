#![allow(missing_docs)]
use crate::compound;

#[allow(missing_docs)]
pub struct Db {
    dbs: Vec<compound::Db>,
}

use std::path::PathBuf;

impl Db {
    #[allow(missing_docs)]
    pub fn at(objects_directory: impl Into<PathBuf>) -> Result<Self, compound::init::Error> {
        unimplemented!("todo")
    }
}
