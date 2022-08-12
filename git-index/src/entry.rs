use crate::Version;
use bitflags::bitflags;

/// The stage of an entry, one of 0 = base, 1 = ours, 2 = theirs
pub type Stage = u32;

bitflags! {
    /// The kind of file of an entry.
    pub struct Mode: u32 {
        /// directory (only used for sparse checkouts), equivalent to a tree
        const DIR = 0o040000;
        /// regular file
        const FILE = 0o100644;
        /// regular file, executable
        const FILE_EXECUTABLE = 0o100755;
        /// Symbolic link
        const SYMLINK = 0o120000;
        /// A git commit for submodules
        const COMMIT = 0o160000;
    }
}

pub(crate) mod mode {
    impl super::Mode {
        /// Return true if this is a sparse entry, as it points to a directory which usually isn't what an unsparse index tracks.
        pub fn is_sparse(&self) -> bool {
            *self == Self::DIR
        }
    }
}

pub(crate) mod at_rest {
    use bitflags::bitflags;

    bitflags! {
        /// Flags how they are serialized to a storage location
        pub struct Flags: u16 {
            /// A portion of a the flags that encodes the length of the path that follows.
            const PATH_LEN = 0x0fff;
            const STAGE_MASK = 0x3000;
            /// If set, there is more extended flags past this one
            const EXTENDED = 0x4000;
            /// If set, the entry be assumed to match with the version on the working tree, as a way to avoid `lstat()`  checks.
            const ASSUME_VALID = 0x8000;
        }
    }

    impl Flags {
        pub fn to_memory(self) -> super::Flags {
            super::Flags::from_bits((self & (Flags::PATH_LEN | Flags::STAGE_MASK | Flags::ASSUME_VALID)).bits as u32)
                .expect("PATHLEN is part of memory representation")
        }
    }

    bitflags! {
        /// Extended flags - add flags for serialization here and offset them down to u16.
        pub struct FlagsExtended: u16 {
            const INTENT_TO_ADD = 1 << (29 - 16);
            const SKIP_WORKTREE = 1 << (30 - 16);
        }
    }

    impl FlagsExtended {
        pub fn to_flags(self) -> Option<super::Flags> {
            super::Flags::from_bits((self.bits as u32) << 16)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn flags_from_bits_with_conflict() {
            let input = 0b1110_0010_1000_1011;
            assert_eq!(Flags::from_bits(input).unwrap().bits, input);
        }
    }
}

bitflags! {
    /// In-memory flags
    pub struct Flags: u32 {
        /// The mask to apply to obtain the stage number of an entry.
        const STAGE_MASK = 0x3000;
        // TODO: could we use the pathlen ourselves to save 8 bytes? And how to handle longer paths than that? 0 as sentinel maybe?
        /// The mask to obtain the length of the path associated with this entry.
        const PATH_LEN = 0x0fff;
        /// If set, the entry be assumed to match with the version on the working tree, as a way to avoid `lstat()`  checks.
        const ASSUME_VALID = 1 << 15;
        /// Indicates that an entry needs to be updated as it's in-memory representation doesn't match what's on disk.
        const UPDATE = 1 << 16;
        /// Indicates an entry should be removed - this typically happens during writing, by simply skipping over them.
        const REMOVE = 1 << 17;
        /// Indicates that an entry is known to be uptodate.
        const UPTODATE = 1 << 18;
        /// Only temporarily used by unpack_trees() (in C)
        const ADDED = 1 << 19;

        /// Whether an up-to-date object hash exists for the entry.
        const HASHED = 1 << 20;
        /// Set if the filesystem monitor is valid.
        const FSMONITOR_VALID = 1 << 21;
        /// Remove in work directory
        const WORKTREE_REMOVE = 1 << 22;
        /// Set to indicate the entry exists in multiple stages at once due to conflicts.
        const CONFLICTED = 1 << 23;

        /// Indicates that the entry was already turned into a tree.
        const UNPACKED = 1 << 24;
        /// Only temporarily used by unpack_trees() (in C)
        const NEW_SKIP_WORKTREE = 1 << 25;

        /// temporarily mark paths matched by a path spec
        const PATHSPEC_MATCHED = 1 << 26;

        /// When the index is split, this indicates the entry is up-to-date in the shared portion of the index.
        const UPDATE_IN_BASE = 1 << 27;
        /// Indicates the entry name is present in the base/shared index, and thus doesn't have to be stored in this one.
        const STRIP_NAME = 1 << 28;

        ///
        /// stored at rest, see at_rest::FlagsExtended
        const INTENT_TO_ADD = 1 << 29;
        /// Stored at rest
        const SKIP_WORKTREE = 1 << 30;

        /// flags that need to be stored on disk in a V3 formatted index.
        const EXTENDED_FLAGS = 1 << 29 | 1 << 30;

        /// For future extension
        const EXTENDED_2 = 1 << 31;
    }
}

impl Flags {
    /// Return the stage as extracted from the bits of this instance.
    pub fn stage(&self) -> Stage {
        (*self & Flags::STAGE_MASK).bits >> 12
    }

    /// Transform ourselves to a storage representation to keep all flags which are to be persisted,
    /// with the caller intending to write `version`.
    pub fn to_storage(&self, version: Version) -> at_rest::Flags {
        assert_eq!(version, Version::V2, "Can only encode V2 flags at the moment");
        at_rest::Flags::from_bits(self.bits() as u16).unwrap()
    }
}

/// The time component in a [`Stat`] struct.
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Time {
    /// The amount of seconds elapsed since EPOCH
    pub secs: u32,
    /// The amount of nanoseconds elapsed in the current second, ranging from 0 to 999.999.999 .
    pub nsecs: u32,
}

/// An entry's filesystem stat information.
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Stat {
    /// Modification time
    pub mtime: Time,
    /// Creation time
    pub ctime: Time,
    /// Device number
    pub dev: u32,
    /// Inode number
    pub ino: u32,
    /// User id of the owner
    pub uid: u32,
    /// Group id of the owning group
    pub gid: u32,
    /// The size of bytes on disk. Capped to u32 so files bigger than that will need thorough additional checking
    pub size: u32,
}

mod access {
    use bstr::{BStr, ByteSlice};

    use crate::{entry, Entry, State};

    impl Entry {
        /// Return an entry's path, relative to the repository, which is extracted from its owning `state`.
        pub fn path<'a>(&self, state: &'a State) -> &'a BStr {
            (&state.path_backing[self.path.clone()]).as_bstr()
        }

        /// Return an entry's path using the given `backing`.
        pub fn path_in<'backing>(&self, backing: &'backing crate::PathStorageRef) -> &'backing BStr {
            (backing[self.path.clone()]).as_bstr()
        }

        /// Return an entry's stage.
        pub fn stage(&self) -> entry::Stage {
            self.flags.stage()
        }
    }
}

mod _impls {
    use std::cmp::Ordering;

    use crate::{Entry, State};

    impl Entry {
        /// Compare one entry to another by their path, by comparing only their common path portion byte by byte, then resorting to
        /// entry length and stage.
        pub fn cmp(&self, other: &Self, state: &State) -> Ordering {
            let lhs = self.path(state);
            let rhs = other.path(state);
            let common_len = lhs.len().min(rhs.len());
            lhs[..common_len]
                .cmp(&rhs[..common_len])
                .then_with(|| lhs.len().cmp(&rhs.len()))
                .then_with(|| self.stage().cmp(&other.stage()))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::entry::at_rest;
    use crate::Version;

    #[test]
    fn in_mem_flags_to_storage_flags_v2() {
        let flag_bytes = u16::from_be_bytes(*b"\x00\x01");
        let flags_at_rest = at_rest::Flags::from_bits(flag_bytes).unwrap();
        let in_memory_flags = flags_at_rest.to_memory();

        let output = in_memory_flags.to_storage(Version::V2);

        assert_eq!(output.bits(), flag_bytes);
    }
}
