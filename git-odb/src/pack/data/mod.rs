use filebuffer::FileBuffer;
use std::path::Path;

pub mod decode;
mod decoded;
pub use decoded::*;
pub mod verify;

mod file;
pub use file::*;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub enum Kind {
    V2,
    V3,
}

pub struct File {
    data: FileBuffer,
    path: std::path::PathBuf,
    kind: Kind,
    num_objects: u32,
}

impl File {
    pub fn kind(&self) -> Kind {
        self.kind.clone()
    }
    pub fn num_objects(&self) -> u32 {
        self.num_objects
    }
    pub fn data_len(&self) -> usize {
        self.data.len()
    }
    pub fn path(&self) -> &Path {
        &self.path
    }
}
