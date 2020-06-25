#![deny(unsafe_code)]

#[cfg(any(feature = "fast-sha1", feature = "minimal-sha1"))]
mod sha1;
mod zlib;

pub mod loose;
pub mod pack;
