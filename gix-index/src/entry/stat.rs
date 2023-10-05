use std::{
    cmp::Ordering,
    time::{SystemTime, SystemTimeError},
};

use filetime::FileTime;

use crate::entry::Stat;

impl Stat {
    /// Detect whether this stat entry is racy if stored in a file index with `timestamp`.
    ///
    /// An index entry is considered racy if it's `mtime` is larger or equal to the index `timestamp`.
    /// The index `timestamp` marks the point in time before which we definitely resolved the racy git problem
    /// for all index entries so any index entries that changed afterwards will need to be examined for
    /// changes by actually reading the file from disk at least once.
    pub fn is_racy(
        &self,
        timestamp: FileTime,
        Options {
            check_stat, use_nsec, ..
        }: Options,
    ) -> bool {
        match timestamp.unix_seconds().cmp(&(self.mtime.secs as i64)) {
            Ordering::Less => true,
            Ordering::Equal if use_nsec && check_stat => timestamp.nanoseconds() <= self.mtime.nsecs,
            Ordering::Equal => true,
            Ordering::Greater => false,
        }
    }

    /// Compares the stat information of two index entries.
    ///
    /// Intuitively this is basically equivalent to `self == other`.
    /// However there a lot of nobs in git that tweak whether certain stat information is used when checking
    /// equality, see [`Options`].
    /// This function respects those options while performing the stat comparison and may therefore ignore some fields.
    pub fn matches(
        &self,
        other: &Self,
        Options {
            trust_ctime,
            check_stat,
            use_nsec,
            use_stdev,
        }: Options,
    ) -> bool {
        if self.mtime.secs != other.mtime.secs {
            return false;
        }
        if check_stat && use_nsec && self.mtime.nsecs != other.mtime.nsecs {
            return false;
        }

        if self.size != other.size {
            return false;
        }

        if trust_ctime {
            if self.ctime.secs != other.ctime.secs {
                return false;
            }
            if check_stat && use_nsec && self.ctime.nsecs != other.ctime.nsecs {
                return false;
            }
        }

        if check_stat {
            if use_stdev && self.dev != other.dev {
                return false;
            }
            self.ino == other.ino && self.gid == other.gid && self.uid == other.uid
        } else {
            true
        }
    }

    /// Creates stat information from the result of `symlink_metadata`.
    pub fn from_fs(fstat: &std::fs::Metadata) -> Result<Stat, SystemTimeError> {
        let mtime = fstat.modified().unwrap_or(std::time::UNIX_EPOCH);
        let ctime = fstat.created().unwrap_or(std::time::UNIX_EPOCH);

        #[cfg(not(unix))]
        let res = Stat {
            mtime: mtime.try_into()?,
            ctime: ctime.try_into()?,
            dev: 0,
            ino: 0,
            uid: 0,
            gid: 0,
            // truncation to 32 bits is on purpose (git does the same).
            size: fstat.len() as u32,
        };
        #[cfg(unix)]
        let res = {
            use std::os::unix::fs::MetadataExt;
            Stat {
                mtime: mtime.try_into().unwrap_or_default(),
                ctime: ctime.try_into().unwrap_or_default(),
                // truncating to 32 bits is fine here because
                // that's what the linux syscalls returns
                // just rust upcasts to 64 bits for some reason?
                // numbers this large are impractical anyway (that's a lot of hard-drives).
                dev: fstat.dev() as u32,
                ino: fstat.ino() as u32,
                uid: fstat.uid(),
                gid: fstat.gid(),
                // truncation to 32 bits is on purpose (git does the same).
                size: fstat.len() as u32,
            }
        };

        Ok(res)
    }
}

impl TryFrom<SystemTime> for Time {
    type Error = SystemTimeError;
    fn try_from(s: SystemTime) -> Result<Self, SystemTimeError> {
        let d = s.duration_since(std::time::UNIX_EPOCH)?;
        Ok(Time {
            // truncation to 32 bits is on purpose (we only compare the low bits)
            secs: d.as_secs() as u32,
            nsecs: d.subsec_nanos(),
        })
    }
}

impl From<Time> for SystemTime {
    fn from(s: Time) -> Self {
        std::time::UNIX_EPOCH + std::time::Duration::new(s.secs.into(), s.nsecs)
    }
}

/// The time component in a [`Stat`] struct.
#[derive(Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Time {
    /// The amount of seconds elapsed since EPOCH.
    pub secs: u32,
    /// The amount of nanoseconds elapsed in the current second, ranging from 0 to 999.999.999 .
    pub nsecs: u32,
}

impl From<FileTime> for Time {
    fn from(value: FileTime) -> Self {
        Time {
            secs: value.unix_seconds().try_into().expect("can't represent non-unix times"),
            nsecs: value.nanoseconds(),
        }
    }
}

impl PartialEq<FileTime> for Time {
    fn eq(&self, other: &FileTime) -> bool {
        *self == Time::from(*other)
    }
}

impl PartialOrd<FileTime> for Time {
    fn partial_cmp(&self, other: &FileTime) -> Option<Ordering> {
        self.partial_cmp(&Time::from(*other))
    }
}

/// Configuration for comparing stat entries
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Options {
    /// If true, a files creation time is taken into consideration when checking if a file changed.
    /// Can be set to false in case other tools alter the creation time in ways that interfere with our operation.
    ///
    /// Default `true`.
    pub trust_ctime: bool,
    /// If true, all stat fields will be used when checking for up-to-date'ness of the entry. Otherwise
    /// nano-second parts of mtime and ctime,uid, gid, inode and device number _will not_ be used, leaving only
    /// the whole-second part of ctime and mtime and the file size to be checked.
    ///
    /// Default `true`.
    pub check_stat: bool,
    /// Whether to compare nano secs when comparing timestamps. This currently
    /// leads to many false positives on linux and is therefore disabled there.
    ///
    /// Default `false`
    pub use_nsec: bool,
    /// Whether to compare network devices secs when comparing timestamps.
    /// Disabled by default because this can cause many false positives on network
    /// devices where the device number is not stable
    ///
    /// Default `false`.
    pub use_stdev: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            trust_ctime: true,
            check_stat: true,
            use_nsec: false,
            use_stdev: false,
        }
    }
}
