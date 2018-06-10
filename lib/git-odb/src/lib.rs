#![feature(ptr_wrapping_offset_from)]

#[macro_use]
extern crate failure;
extern crate byteorder;
extern crate filebuffer;
extern crate hex;
extern crate miniz_oxide;
extern crate smallvec;
extern crate walkdir;

mod zlib;
mod types;

pub mod object;
pub mod loose;
pub mod pack;

pub use types::*;
