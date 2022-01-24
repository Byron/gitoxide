use bitflags::bitflags;

bitflags! {
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

    impl Flags {
        pub fn to_memory(self) -> super::Flags {
            super::Flags::from_bits((self & (Flags::PATH_LEN | Flags::STAGE_MASK | Flags::ASSUME_VALID)).bits as u32)
                .expect("PATHLEN is part of memory representation")
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
        const STAGE_MASK = 0x3000;
        // TODO: could we use the pathlen ourselves to save 8 bytes? And how to handle longer paths than that? 0 as sentinel maybe?
        const PATH_LEN = 0x0fff;
        /// If set, the entry be assumed to match with the version on the working tree, as a way to avoid `lstat()`  checks.
        const ASSUME_VALID = 1 << 15;
        const UPDATE = 1 << 16;
        const REMOVE = 1 << 17;
        const UPTODATE = 1 << 18;
        const ADDED = 1 << 19;

        const HASHED = 1 << 20;
        const FSMONITOR_VALID = 1 << 21;
        /// Remove in work directory
        const WORKTREE_REMOVE = 1 << 22;
        const CONFLICTED = 1 << 23;

        const UNPACKED = 1 << 24;
        const NEW_SKIP_WORKTREE = 1 << 25;

        /// temporarily mark paths matched by a path spec
        const PATHSPEC_MATCHED = 1 << 26;

        const UPDATE_IN_BASE = 1 << 27;
        const STRIP_NAME = 1 << 28;

        const INTENT_TO_ADD = 1 << 29; // stored at rest, see at_rest::FlagsExtended
        const SKIP_WORKTREE = 1 << 30; // stored at rest
    }
}

impl Flags {
    pub fn stage(&self) -> u32 {
        (*self & Flags::STAGE_MASK).bits >> 12
    }
}

#[derive(PartialEq, Eq, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Time {
    pub secs: u32,
    pub nsecs: u32,
}

#[derive(PartialEq, Eq, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Stat {
    pub mtime: Time,
    pub ctime: Time,
    pub dev: u32,
    pub ino: u32,
    pub uid: u32,
    pub gid: u32,
    /// The size of bytes on disk. Capped to u32 so files bigger than that will need thorough checking (and hopefully never make it)
    pub size: u32,
}

mod access {
    use bstr::{BStr, ByteSlice};

    use crate::{Entry, State};

    impl Entry {
        pub fn path<'a>(&self, state: &'a State) -> &'a BStr {
            (&state.path_backing[self.path.clone()]).as_bstr()
        }
    }
}
