use std::cmp::Ordering;
use std::time::{SystemTime, SystemTimeError};

use filetime::FileTime;

use crate::entry::Stat;

impl Stat {
    /// detect whether this stat entry is racy given the timestamp
    /// of the index it belongs to
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

    /// Compares stat information given a set of git setting
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

    /// Creates stat information from the result of symlink_metadata
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
            // truncation to 32 bits is on purpose (git does the same)
            size: fstat.len() as u32,
        };
        #[cfg(unix)]
        use std::os::unix::fs::MetadataExt;
        #[cfg(unix)]
        let res = Stat {
            mtime: mtime.try_into()?,
            ctime: ctime.try_into()?,
            // truncating to 32 bits is fine here because
            // that's what the linux syscalls returns
            // just rust upcasts to 64 bits for some reason?
            // numbers this large are impractical anyway (that's a lot of harddrvies)
            dev: fstat.dev() as u32,
            ino: fstat.ino() as u32,
            uid: fstat.uid(),
            gid: fstat.gid(),
            // truncation to 32 bits is on purpose (git does the same)
            size: fstat.len() as u32,
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
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Time {
    /// The amount of seconds elapsed since EPOCH
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
    /// Default true.
    pub trust_ctime: bool,
    /// If true, all stat fields will be used when checking for up-to-date'ness of the entry. Otherwise
    /// nano-second parts of mtime and ctime,uid, gid, inode and device number _will not_ be used, leaving only
    /// the whole-second part of ctime and mtime and the file size to be checked.
    ///
    /// Default true.
    pub check_stat: bool,
    // TODO: enable by default? Seems to be disabled in git??
    // documentation only talks about this being disabled for linux but I can't
    // see it being enabled on other OS
    /// Whether to compare nano secs when comparing timestamps. This currently
    /// leads to many false positives on linux and is therefore disabled there.
    ///
    /// Default false
    pub use_nsec: bool,
    /// Whether to compare network devices secs when comparing timestamps.
    /// Disabled by default because this can cause many false positives on network
    /// devices where the device number is not stable
    ///
    /// Default false
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

#[cfg(test)]
mod test {
    use filetime::FileTime;

    use crate::entry::stat::{Options, Time};
    use crate::entry::Stat;

    #[test]
    fn use_nsec() {
        let stat1 = Stat {
            mtime: Time { secs: 0, nsecs: 0 },
            ctime: Time { secs: 0, nsecs: 0 },
            dev: 0,
            ino: 0,
            uid: 0,
            gid: 0,
            size: 0,
        };
        let stat2 = Stat {
            mtime: Time { secs: 0, nsecs: 10 },
            ctime: Time { secs: 0, nsecs: 0 },
            dev: 0,
            ino: 0,
            uid: 0,
            gid: 0,
            size: 0,
        };

        assert!(stat1.matches(&stat2, Options::default()));
        assert!(!stat1.matches(
            &stat2,
            Options {
                use_nsec: true,
                ..Default::default()
            }
        ));
        assert!(stat1.matches(
            &stat2,
            Options {
                use_nsec: true,
                check_stat: false,
                ..Default::default()
            }
        ));
    }

    #[test]
    fn use_ctime() {
        let stat1 = Stat {
            mtime: Time { secs: 0, nsecs: 0 },
            ctime: Time { secs: 1, nsecs: 2 },
            dev: 0,
            ino: 0,
            uid: 0,
            gid: 0,
            size: 0,
        };
        let stat2 = Stat {
            mtime: Time { secs: 0, nsecs: 0 },
            ctime: Time { secs: 3, nsecs: 4 },
            dev: 0,
            ino: 0,
            uid: 0,
            gid: 0,
            size: 0,
        };

        assert!(!stat1.matches(&stat2, Options::default()));
        assert!(!stat1.matches(
            &stat2,
            Options {
                use_nsec: true,
                ..Default::default()
            }
        ));
        assert!(stat1.matches(
            &stat2,
            Options {
                trust_ctime: false,
                ..Default::default()
            }
        ));
    }

    #[test]
    fn use_stdev() {
        let stat1 = Stat {
            mtime: Time { secs: 0, nsecs: 0 },
            ctime: Time { secs: 0, nsecs: 0 },
            dev: 0,
            ino: 0,
            uid: 0,
            gid: 0,
            size: 0,
        };
        let stat2 = Stat {
            mtime: Time { secs: 0, nsecs: 0 },
            ctime: Time { secs: 0, nsecs: 0 },
            dev: 1,
            ino: 0,
            uid: 0,
            gid: 0,
            size: 0,
        };

        assert!(stat1.matches(&stat2, Options::default()));
        assert!(!stat1.matches(
            &stat2,
            Options {
                use_stdev: true,
                ..Default::default()
            }
        ));
    }

    #[test]
    fn check_stat() {
        let stat1 = Stat {
            mtime: Time { secs: 0, nsecs: 0 },
            ctime: Time { secs: 0, nsecs: 0 },
            dev: 0,
            ino: 0,
            uid: 0,
            gid: 0,
            size: 0,
        };
        let mut stat2 = stat1;
        assert!(stat1.matches(&stat2, Options::default()));
        assert!(stat1.matches(
            &stat2,
            Options {
                check_stat: false,
                ..Default::default()
            }
        ));
        stat2.ino = 1;
        assert!(!stat1.matches(&stat2, Options::default()));
        assert!(stat1.matches(
            &stat2,
            Options {
                check_stat: false,
                ..Default::default()
            }
        ));
        stat2 = stat1;
        stat2.uid = 1;
        assert!(!stat1.matches(&stat2, Options::default()));
        assert!(stat1.matches(
            &stat2,
            Options {
                check_stat: false,
                ..Default::default()
            }
        ));
        stat2 = stat1;
        stat2.gid = 1;
        assert!(!stat1.matches(&stat2, Options::default()));
        assert!(stat1.matches(
            &stat2,
            Options {
                check_stat: false,
                ..Default::default()
            }
        ));
        stat2 = stat1;
        stat2.size = 1;
        assert!(!stat1.matches(&stat2, Options::default()));
        assert!(!stat1.matches(
            &stat2,
            Options {
                check_stat: false,
                ..Default::default()
            }
        ));
    }

    #[test]
    fn racy_timestamp() {
        let stat1 = Stat {
            mtime: Time { secs: 1, nsecs: 10 },
            ctime: Time { secs: 0, nsecs: 0 },
            dev: 0,
            ino: 0,
            uid: 0,
            gid: 0,
            size: 0,
        };
        assert!(stat1.is_racy(FileTime::from_unix_time(1, 0), Options::default()));
        assert!(stat1.is_racy(
            FileTime::from_unix_time(1, 0),
            Options {
                use_nsec: true,
                ..Default::default()
            }
        ));
        assert!(stat1.is_racy(FileTime::from_unix_time(1, 10), Options::default()));
        assert!(stat1.is_racy(
            FileTime::from_unix_time(1, 10),
            Options {
                use_nsec: true,
                ..Default::default()
            }
        ));
        assert!(stat1.is_racy(FileTime::from_unix_time(1, 20), Options::default()));
        assert!(!stat1.is_racy(
            FileTime::from_unix_time(1, 20),
            Options {
                use_nsec: true,
                ..Default::default()
            }
        ));
        assert!(!stat1.is_racy(FileTime::from_unix_time(2, 0), Options::default()));
        assert!(!stat1.is_racy(
            FileTime::from_unix_time(2, 0),
            Options {
                use_nsec: true,
                ..Default::default()
            }
        ));
    }
}
