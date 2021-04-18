#![forbid(missing_docs, rust_2018_idioms)]
#![deny(unsafe_code)]
//! A crate providing foundational capabilities to other `git-*` crates with trade-offs between compile time, binary size or speed
//! selectable using cargo feature toggles.
//!
//! It's designed to allow the application level crate to configure feature toggles, affecting all other `git-*` crates using
//! this one.
//!
//! Thus all features provided here commonly have a 'cheap' base implementation, with the option to pull in
//! counterparts with higher performance.

pub mod fs;
pub mod hash;
pub mod interrupt;
#[cfg(feature = "io-pipe")]
pub mod io;
pub mod parallel;
pub mod progress;
