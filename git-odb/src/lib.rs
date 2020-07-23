#![forbid(unsafe_code)]

mod zlib;

pub mod loose;
pub mod pack;

mod sink;
pub use sink::{sink, Sink};

mod traits;
pub use traits::*;
