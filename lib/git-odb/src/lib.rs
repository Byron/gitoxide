#![feature(offset_to)]

#[macro_use]
extern crate failure;
extern crate hex;
extern crate miniz_oxide;
extern crate smallvec;
extern crate walkdir;

mod zlib;

pub mod object;
pub mod loose;
