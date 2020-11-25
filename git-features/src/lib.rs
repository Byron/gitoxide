#![forbid(unsafe_code, rust_2018_idioms)]

pub mod fs;
pub mod hash;
pub mod interrupt;
/// A unidirectional pipe for bytes, analogous to a unix pipe. Available with the `io-pipe` feature toggle.
#[cfg(feature = "io-pipe")]
pub mod io;
pub mod parallel;
pub mod progress;
