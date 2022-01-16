#![deny(unsafe_code, missing_docs, rust_2018_idioms)]
#![allow(missing_docs)]
//! An implementation of the shared parts of git bitmaps used in `git-pack`, `git-index` and `git-worktree`.
//!
//! Note that many tests are performed indirectly by tests in the aforementioned consumer crates.

/// Bitmap utilities for the advanced word-aligned hybrid bitmap
pub mod ewah;

pub(crate) mod decode {
    use quick_error::quick_error;

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            Corrupt(message: &'static str) {
                display("{}", message)
            }
        }
    }

    #[inline]
    pub(crate) fn split_at_pos(data: &[u8], pos: usize) -> Option<(&[u8], &[u8])> {
        if data.len() < pos {
            return None;
        }
        data.split_at(pos).into()
    }

    #[inline]
    pub(crate) fn u32(data: &[u8]) -> Option<(u32, &[u8])> {
        split_at_pos(data, 4).map(|(num, data)| (u32::from_be_bytes(num.try_into().unwrap()), data))
    }
}
