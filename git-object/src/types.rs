use std::fmt;

use quick_error::quick_error;

/// The four types of objects that git differentiates.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
#[allow(missing_docs)]
pub enum Kind {
    Tree,
    Blob,
    Commit,
    Tag,
}
quick_error! {
    /// The Error used in [`Kind::from_bytes()`].
    #[derive(Debug, Clone)]
    #[allow(missing_docs)]
    pub enum Error {
        InvalidObjectKind(kind: crate::BString) {
            display("Unknown object kind: {:?}", std::str::from_utf8(kind))
        }
    }
}

impl Kind {
    /// Parse a `Kind` from its serialized loose git objects.
    pub fn from_bytes(s: &[u8]) -> Result<Kind, Error> {
        Ok(match s {
            b"tree" => Kind::Tree,
            b"blob" => Kind::Blob,
            b"commit" => Kind::Commit,
            b"tag" => Kind::Tag,
            _ => return Err(Error::InvalidObjectKind(s.into())),
        })
    }

    /// Return the name of `self` for use in serialized loose git objects.
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Kind::Tree => b"tree",
            Kind::Commit => b"commit",
            Kind::Blob => b"blob",
            Kind::Tag => b"tag",
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(std::str::from_utf8(self.as_bytes()).expect("Converting Kind name to utf8"))
    }
}

///
pub mod tree {
    /// The mode of items storable in a tree, similar to the file mode on a unix file system.
    ///
    /// Used in [mutable::Entry][crate::mutable::tree::Entry] and [tree::Entry][crate::immutable::tree::Entry].
    #[derive(Clone, Copy, PartialEq, Eq, Debug, Ord, PartialOrd, Hash)]
    #[repr(u16)]
    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    #[allow(missing_docs)]
    pub enum EntryMode {
        Tree = 0o040000u16,
        Blob = 0o100644,
        BlobExecutable = 0o100755,
        Link = 0o120000,
        Commit = 0o160000,
    }

    impl EntryMode {
        /// Return true if this entry mode represents a Tree/directory
        pub fn is_tree(&self) -> bool {
            *self == EntryMode::Tree
        }

        /// Return true if this entry mode represents anything BUT Tree/directory
        pub fn is_no_tree(&self) -> bool {
            *self != EntryMode::Tree
        }
    }
}
