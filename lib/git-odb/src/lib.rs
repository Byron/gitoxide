#[macro_use]
extern crate failure;
extern crate hex;
extern crate miniz_oxide;
extern crate smallvec;
extern crate walkdir;

mod deflate;

pub mod object;
pub mod loose;
