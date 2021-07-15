use bstr::BStr;
use filebuffer::FileBuffer;
use git_hash::ObjectId;

/// A buffer that is either memory mapped or fully in-memory depending on a cutoff.
pub enum Buffer {
    /// The buffer is loaded entirely in memory
    InMemory(Vec<u8>),
    /// The buffer is mapping the file on disk.
    Mapped(FileBuffer),
}

/// A reference as parsed from the `packed-refs` file
#[derive(Debug, PartialEq, Eq)]
pub struct Reference<'a> {
    /// The unvalidated full name of the reference.
    pub full_name: &'a BStr,
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
}

mod decode;

///
pub mod iter;

///
pub mod buffer;

///
pub mod find;
