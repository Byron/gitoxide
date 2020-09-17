#![deny(unsafe_code, rust_2018_idioms)]

mod zlib;

pub mod compound;
pub mod loose;
pub mod pack;

mod sink;
pub use sink::{sink, Sink};

pub(crate) mod hash;
mod traits;

pub mod borrowed;

pub use traits::*;
