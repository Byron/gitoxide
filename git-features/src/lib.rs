#![forbid(unsafe_code, rust_2018_idioms)]

pub mod fs;
pub mod hash;
pub mod interrupt;
#[cfg(feature = "io-pipe")]
pub mod io;
pub mod parallel;
pub mod progress;
