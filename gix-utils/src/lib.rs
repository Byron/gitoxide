//! A crate with utilities that don't need feature toggles.
//!
//! If they would need feature toggles, they should be in `gix-features` instead.
#![deny(rust_2018_idioms, missing_docs)]
#![forbid(unsafe_code)]

///
pub mod backoff;

///
pub mod buffers;

///
pub mod str;

///
pub mod btoi;

/// A utility to do buffer-swapping with.
///
/// Use `src` to read from and `dest` to write to, and after actually changing data, call [Buffers::swap()].
/// To be able to repeat the process, this time using what was `dest` as `src`, freeing up `dest` for writing once more.
///
/// Note that after each [`Buffers::swap()`], `src` is the most recent version of the data, just like before each swap.
#[derive(Default, Clone)]
pub struct Buffers {
    /// The source data, as basis for processing.
    pub src: Vec<u8>,
    /// The data produced after processing `src`.
    pub dest: Vec<u8>,
}
