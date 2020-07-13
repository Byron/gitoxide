use std::path::PathBuf;

pub struct Db {
    pub path: PathBuf,
}

/// Initialization
impl Db {
    pub fn at(path: impl Into<PathBuf>) -> Db {
        Db { path: path.into() }
    }
}

pub mod iter;
pub mod locate;
