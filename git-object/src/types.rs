use crate::mutable::SPACE;
use quick_error::quick_error;
use std::{fmt, io};

/// Indicates if a number is positive or negative for use in [`Time`].
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
#[allow(missing_docs)]
pub enum Sign {
    Plus,
    Minus,
}

/// A timestamp with timezone.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Time {
    /// time in seconds from epoch.
    pub time: u32,
    /// time offset in seconds, may be negative to match the `sign` field.
    pub offset: i32,
    /// the sign of `offset`, used to encode `-0000` which would otherwise loose sign information.
    pub sign: Sign,
}

impl Time {
    /// Serialize this instance to `out` in a format suitable for use in header fields of serialized git commits or tags.
    pub fn write_to(&self, mut out: impl io::Write) -> io::Result<()> {
        itoa::write(&mut out, self.time)?;
        out.write_all(SPACE)?;
        out.write_all(&[match self.sign {
            Sign::Plus => b'+',
            Sign::Minus => b'-',
        }])?;

        const ZERO: &[u8; 1] = b"0";

        const SECONDS_PER_HOUR: i32 = 60 * 60;
        let offset = self.offset.abs();
        let hours = offset / SECONDS_PER_HOUR;
        assert!(hours < 25, "offset is more than a day: {}", hours);
        let minutes = (offset - (hours * SECONDS_PER_HOUR)) / 60;

        if hours < 10 {
            out.write_all(ZERO)?;
        }
        itoa::write(&mut out, hours)?;

        if minutes < 10 {
            out.write_all(ZERO)?;
        }
        itoa::write(&mut out, minutes).map(|_| ())
    }
}

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
    #[derive(Debug)]
    #[allow(missing_docs)]
    pub enum Error {
        InvalidObjectKind(kind: crate::BString) {
            display("Unknown object kind: {:?}", std::str::from_utf8(&kind))
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
    pub fn to_bytes(&self) -> &[u8] {
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
        f.write_str(std::str::from_utf8(self.to_bytes()).expect("Converting Kind name to utf8"))
    }
}

///
pub mod tree {
    /// The mode of items storable in a tree, similar to the file mode on a unix file system.
    ///
    /// Used in [mutable::Entry][crate::mutable::tree::Entry] and [immutable::Entry][crate::immutable::tree::Entry].
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
