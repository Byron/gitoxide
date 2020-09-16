#![forbid(unsafe_code, rust_2018_idioms)]

pub mod hash;
pub mod interrupt;
pub mod parallel;
#[cfg(feature = "pipe")]
pub mod pipe;
pub mod progress;
