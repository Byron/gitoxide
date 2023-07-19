//! The implementation of creating an archive from a git tree, similar to `git archive`.
#![deny(rust_2018_idioms, missing_docs, unsafe_code)]

use gix_object::bstr::BString;
use std::sync::Arc;

///
pub mod stream;

mod write;
pub use write::write_to;

/// A stream of entries that is produced from an underlying reader.
pub struct Stream {
    read: gix_features::io::pipe::Reader,
    err: Arc<parking_lot::Mutex<Option<stream::Error>>>,
    /// `None` if currently held by an entry.
    path_buf: Option<BString>,
}
