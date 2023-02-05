/// The stage of an entry, one of 0 = base, 1 = ours, 2 = theirs
pub type Stage = u32;

mod mode;
pub use mode::Mode;

mod flags;
pub(crate) use flags::at_rest;
pub use flags::Flags;

mod write;

/// The time component in a [`Stat`] struct.
#[derive(Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Time {
    /// The amount of seconds elapsed since EPOCH
    pub secs: u32,
    /// The amount of nanoseconds elapsed in the current second, ranging from 0 to 999.999.999 .
    pub nsecs: u32,
}

/// An entry's filesystem stat information.
#[derive(Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd, Clone, Copy)]
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
            state.path_backing[self.path.clone()].as_bstr()
        }

        /// Return an entry's path using the given `backing`.
        pub fn path_in<'backing>(&self, backing: &'backing crate::PathStorageRef) -> &'backing BStr {
            backing[self.path.clone()].as_bstr()
        }

        /// Return an entry's stage.
        pub fn stage(&self) -> entry::Stage {
            self.flags.stage()
        }
    }
}

mod _impls {
    use std::{cmp::Ordering, ops::Add, time::SystemTime};

    use bstr::BStr;

    use crate::{entry::Time, Entry, State};

    impl From<SystemTime> for Time {
        fn from(s: SystemTime) -> Self {
            let d = s
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system time is not before unix epoch!");
            Time {
                secs: d.as_secs() as u32,
                nsecs: d.subsec_nanos(),
            }
        }
    }

    impl From<Time> for SystemTime {
        fn from(s: Time) -> Self {
            std::time::UNIX_EPOCH.add(std::time::Duration::new(s.secs.into(), s.nsecs))
        }
    }

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
