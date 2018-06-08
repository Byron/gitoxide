extern crate failure;

use std::path::PathBuf;
use std::iter::once;

type ObjectId = [u8; 20];

pub struct LooseObjectDb {
    path: PathBuf,
}

impl LooseObjectDb {
    pub fn at(path: impl Into<PathBuf>) -> LooseObjectDb {
        LooseObjectDb { path: path.into() }
    }

    pub fn iter(&self) -> impl Iterator<Item = ObjectId> {
        once([0; 20])
    }
}
