use super::{Flags, Mode, Stat, Time};
use crate::Entry;
use git_hash::ObjectId;
use std::ops::Range;

impl Entry {
    /// Creates a new Entry
    pub fn new(stat: Stat, id: ObjectId, flags: Flags, mode: Mode, path: Range<usize>) -> Entry {
        Entry {
            stat,
            id,
            flags,
            mode,
            path,
        }
    }
}

impl Stat {
    /// Creates a new Stat with all its values set to zero
    pub fn zero() -> Stat {
        Stat {
            mtime: Time { secs: 0, nsecs: 0 },
            ctime: Time { secs: 0, nsecs: 0 },
            dev: 0,
            ino: 0,
            uid: 0,
            gid: 0,
            size: 0,
        }
    }
}
