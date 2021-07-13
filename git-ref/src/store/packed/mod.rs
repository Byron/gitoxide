#![allow(missing_docs, dead_code)]

use bstr::BStr;
use git_hash::ObjectId;

#[derive(Debug, PartialEq, Eq)]
enum Peeled {
    Unspecified,
    Partial,
    Fully,
}

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
    /// the [`target()`] if this is not a fully peeled reference like a tag.
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
