#![forbid(unsafe_code, rust_2018_idioms)]

pub mod fs;
/// Hash functions and hash utilities
pub mod hash;
/// Process-global interrupt handling
pub mod interrupt;
pub mod parallel;
#[cfg(feature = "pipe")]
pub mod pipe;
pub mod progress;
