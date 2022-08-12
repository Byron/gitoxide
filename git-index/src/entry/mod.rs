/// The stage of an entry, one of 0 = base, 1 = ours, 2 = theirs
pub type Stage = u32;

mod mode;
pub use mode::Mode;

mod flags;
pub(crate) use flags::at_rest;
pub use flags::Flags;

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
mod tests;
