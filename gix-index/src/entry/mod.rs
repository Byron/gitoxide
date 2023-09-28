/// The stage of an entry, one of…
/// * 0 = no conflict,
/// * 1 = base,
/// * 2 = ours,
/// * 3 = theirs
pub type Stage = u32;

///
pub mod mode;

mod flags;
pub(crate) use flags::at_rest;
pub use flags::Flags;

///
pub mod stat;
mod write;

use bitflags::bitflags;

// TODO: we essentially treat this as an enum withj the only exception being
// that `FILE_EXECUTABLE.contains(FILE)` works might want to turn this into an
// enum proper
bitflags! {
    /// The kind of file of an entry.
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct Mode: u32 {
        /// directory (only used for sparse checkouts), equivalent to a tree, which is _excluded_ from the index via
        /// cone-mode.
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

/// An entry's filesystem stat information.
#[derive(Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Stat {
    /// Modification time
    pub mtime: stat::Time,
    /// Creation time
    pub ctime: stat::Time,
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
            state.path_backing[self.path.clone()].as_bstr()
        }

        /// Return an entry's path using the given `backing`.
        pub fn path_in<'backing>(&self, backing: &'backing crate::PathStorageRef) -> &'backing BStr {
            backing[self.path.clone()].as_bstr()
        }

        /// Return an entry's stage. See [entry::Stage] for possible values.
        pub fn stage(&self) -> entry::Stage {
            self.flags.stage()
        }
    }
}

mod _impls {
    use std::cmp::Ordering;

    use bstr::BStr;

    use crate::{Entry, State};

    impl Entry {
        /// Compare one entry to another by their path, by comparing only their common path portion byte by byte, then resorting to
        /// entry length and stage.
        pub fn cmp(&self, other: &Self, state: &State) -> Ordering {
            let lhs = self.path(state);
            let rhs = other.path(state);
            Entry::cmp_filepaths(lhs, rhs).then_with(|| self.stage().cmp(&other.stage()))
        }

        /// Compare one entry to another by their path, by comparing only their common path portion byte by byte, then resorting to
        /// entry length.
        pub fn cmp_filepaths(a: &BStr, b: &BStr) -> Ordering {
            let common_len = a.len().min(b.len());
            a[..common_len]
                .cmp(&b[..common_len])
                .then_with(|| a.len().cmp(&b.len()))
        }
    }
}
