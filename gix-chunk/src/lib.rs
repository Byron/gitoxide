//! Low-level access to reading and writing chunk file based formats.
//!
//! See the [git documentation](https://github.com/git/git/blob/seen/Documentation/technical/chunk-format.txt) for details.
#![deny(missing_docs, rust_2018_idioms, unsafe_code)]

/// An identifier to describe the kind of chunk, unique within a chunk file, typically in ASCII
pub type Id = [u8; 4];

/// A special value denoting the end of the chunk file table of contents.
pub const SENTINEL: Id = [0u8; 4];

///
pub mod range {
    use std::{convert::TryInto, ops::Range};

    use crate::file;

    /// Turn a u64 Range into a usize range safely, to make chunk ranges useful in memory mapped files.
    pub fn into_usize(Range { start, end }: Range<file::Offset>) -> Option<Range<usize>> {
        let start = start.try_into().ok()?;
        let end = end.try_into().ok()?;
        Some(Range { start, end })
    }

    /// Similar to [`into_usize()`], but panics assuming that the memory map couldn't be created if offsets
    /// stored are too high.
    ///
    /// This is only true for correctly formed files, as it's entirely possible to provide out of bounds offsets
    /// which are checked for separately - we wouldn't be here if that was the case.
    pub fn into_usize_or_panic(range: Range<file::Offset>) -> Range<usize> {
        into_usize(range).expect("memory maps can't be created if files are too large")
    }
}

///
pub mod file;
