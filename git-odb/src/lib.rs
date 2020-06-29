#![deny(unsafe_code)]

#[cfg(any(feature = "fast-sha1", feature = "minimal-sha1"))]
mod hash;
mod parallel;
mod zlib;

pub mod loose;
pub mod pack;
