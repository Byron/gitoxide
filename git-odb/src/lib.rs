#[macro_use]
extern crate failure;
extern crate byteorder;
extern crate filebuffer;
extern crate hex;
extern crate miniz_oxide;
extern crate smallvec;
extern crate walkdir;
#[macro_use]
extern crate itertools;

mod types;
mod zlib;

pub mod loose;
pub mod object;
pub mod pack;

pub use types::*;
