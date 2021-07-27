use crate::{transaction::RefEdit, FullName};
use bstr::{BStr, BString};
use filebuffer::FileBuffer;
use git_hash::ObjectId;
use std::path::PathBuf;

enum Backing {
    /// The buffer is loaded entirely in memory, along with the `offset` to the first record past the header.
    InMemory(Vec<u8>),
    /// The buffer is mapping the file on disk, along with the offset to the first record past the header
    Mapped(FileBuffer),
}

/// A buffer containing a packed-ref file that is either memory mapped or fully in-memory depending on a cutoff.
///
/// The buffer is guaranteed to be sorted as per the packed-ref rules which allows some operations to be more efficient.
pub struct Buffer {
    data: Backing,
    /// The offset to the first record, how many bytes to skip past the header
    offset: usize,
    /// The path from which we were loaded
    path: PathBuf,
}

/// A transaction for editing packed references
pub struct Transaction {
    /// Probably soon private and returned as part of a commit
    buffer: Option<Buffer>,
    edits: Option<Vec<RefEdit>>,
    lock: Option<git_lock::File>,
    closed_lock: Option<git_lock::Marker>,
}

/// A reference as parsed from the `packed-refs` file
#[derive(Debug, PartialEq, Eq)]
pub struct Reference<'a> {
    /// The unvalidated full name of the reference.
    pub name: FullName<'a>,
    /// The target object id of the reference, hex encoded.
    pub target: &'a BStr,
    /// The fully peeled object id, hex encoded, that the ref is ultimately pointing to
    /// i.e. when all indirections are removed.
    pub object: Option<&'a BStr>,
}

impl<'a> Reference<'a> {
    /// Decode the target as object
    pub fn target(&self) -> ObjectId {
        git_hash::ObjectId::from_hex(self.target).expect("parser validation")
    }

    /// Decode the object this reference is ultimately pointing to. Note that this is
    /// the [`target()`][Reference::target()] if this is not a fully peeled reference like a tag.
    pub fn object(&self) -> ObjectId {
        self.object.map_or_else(
            || self.target(),
            |id| ObjectId::from_hex(id).expect("parser validation"),
        )
    }
}

/// An iterator over references in a packed refs file
pub struct Iter<'a> {
    /// The position at which to parse the next reference
    cursor: &'a [u8],
    /// The next line, starting at 1
    current_line: usize,
    /// If set, references returned will match the prefix, the first failed match will stop all iteration.
    prefix: Option<BString>,
}

mod decode;

///
pub mod iter;

///
pub mod buffer;

///
pub mod find;

///
pub mod transaction;
