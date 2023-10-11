use std::fs::File;
use std::io;

use crate::{FetchHead, FetchHeadEntry};

impl FetchHead {
    pub fn append(_entry: FetchHeadEntry) /*Result type?*/ {}

    pub fn write() -> io::Result<File> {
        todo!();
    }
}
