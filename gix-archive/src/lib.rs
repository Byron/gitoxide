//! The implementation of creating an archive from a git tree, similar to `git archive`, but using an internal format.
//!
//! This crate can effectively be used to manipulate worktrees as streams of bytes, which can be decoded using the [`Stream`] type.
#![deny(rust_2018_idioms, missing_docs, unsafe_code)]

use gix_object::bstr::BString;

///
pub mod stream;

mod write;
pub use write::write_to;

/// A stream of entries that originate from a git tree and optionally from additional entries.
///
/// Note that a git tree is mandatory, but the empty tree can be used to effectively disable it.
pub struct Stream {
    read: stream::utils::Read,
    err: stream::SharedErrorSlot,
    extra_entries: Option<std::sync::mpsc::Sender<stream::AdditionalEntry>>,
    // additional_entries: Vec,
    /// `None` if currently held by an entry.
    path_buf: Option<BString>,
    /// Another buffer to partially act like a buf-reader.
    buf: Vec<u8>,
    /// The offset into `buf` for entries being able to act like a buf reader.
    pos: usize,
    /// The amount of bytes usable from `buf` (even though it always has a fixed size)
    filled: usize,
}
