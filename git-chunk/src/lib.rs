//! Low-level access to reading and writing chunk file based formats.
//!
//! See the [git documentation](https://github.com/git/git/blob/seen/Documentation/technical/chunk-format.txt) for details.
#![deny(unsafe_code)]
#![deny(rust_2018_idioms, missing_docs)]

use std::convert::TryInto;
use std::ops::Range;

/// An identifier to describe the kind of chunk, unique within a chunk file, typically in ASCII
pub type Id = [u8; 4];

/// A special value denoting the end of the chunk file table of contents.
pub const SENTINEL: Id = [0u8; 4];

/// Turn a u64 Range into a usize range safely, to make chunk ranges useful in memory mapped files.
pub fn into_usize_range(Range { start, end }: Range<file::Offset>) -> Option<Range<usize>> {
    let start = start.try_into().ok()?;
    let end = end.try_into().ok()?;
    Some(Range { start, end })
}

///
pub mod file;
