#![deny(unsafe_code)]

#[cfg(any(feature = "fast-sha1", feature = "minimal-sha1"))]
mod hash;
mod zlib;

// possibly put these into a shared crate
mod parallel;
mod progress;

pub mod loose;
pub mod pack;
