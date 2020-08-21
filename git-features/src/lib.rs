#![forbid(unsafe_code)]

pub mod hash;
pub mod interrupt;
pub mod parallel;
#[cfg(feature = "pipe")]
pub mod pipe;
pub mod progress;
