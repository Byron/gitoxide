//! This module contains a `Metadata` implementation that must be used instead of `std::fs::Metadata` to assure
//! that the `ctime` information is populated exactly like the one in `git`, which wouldn't be the case on unix.
#![allow(clippy::useless_conversion)] // on some MacOOS conversions are required, but on linux usually not.
#![allow(clippy::unnecessary_cast)]

// it's allowed for good measure, in case there are systems that use different types for that.
use std::{path::Path, time::SystemTime};

/// A structure to partially mirror [`std::fs::Metadata`].
#[cfg(not(windows))]
pub struct Metadata(rustix::fs::Stat);

#[cfg(windows)]
/// A structure to partially mirror [`std::fs::Metadata`].
pub struct Metadata(std::fs::Metadata);

/// Lifecycle
impl Metadata {
    /// Obtain the metadata at `path` without following symlinks.
    pub fn from_path_no_follow(path: &Path) -> Result<Self, std::io::Error> {
        #[cfg(not(windows))]
        {
            rustix::fs::lstat(path).map(Metadata).map_err(Into::into)
        }
        #[cfg(windows)]
        path.symlink_metadata().map(Metadata)
    }

    /// Obtain the metadata at `path` without following symlinks.
    pub fn from_file(file: &std::fs::File) -> Result<Self, std::io::Error> {
        #[cfg(not(windows))]
        {
            rustix::fs::fstat(file).map(Metadata).map_err(Into::into)
        }
        #[cfg(windows)]
        file.metadata().map(Metadata)
    }
}

/// Access
#[allow(clippy::len_without_is_empty)]
impl Metadata {
    /// Return true if the metadata belongs to a directory
    pub fn is_dir(&self) -> bool {
        #[cfg(not(windows))]
        {
            (self.0.st_mode as u32 & libc::S_IFMT as u32) == libc::S_IFDIR as u32
        }
        #[cfg(windows)]
        self.0.is_dir()
    }

    /// Return the time at which the underlying file was modified.
    pub fn modified(&self) -> Option<SystemTime> {
        #[cfg(not(windows))]
        {
            Some(system_time_from_secs_nanos(
                self.0.st_mtime.try_into().ok()?,
                #[cfg(not(target_os = "netbsd"))]
                self.0.st_mtime_nsec.try_into().ok()?,
                #[cfg(target_os = "netbsd")]
                self.0.st_mtimensec.try_into().ok()?,
            ))
        }
        #[cfg(windows)]
        self.0.modified().ok()
    }

    /// Return the time at which the underlying file was created.
    ///
    /// Note that this differes from [`std::fs::Metadata::created()`] which would return
    /// the inode birth time, which is notably different to what `git` does.
    pub fn created(&self) -> Option<SystemTime> {
        #[cfg(not(windows))]
        {
            Some(system_time_from_secs_nanos(
                self.0.st_ctime.try_into().ok()?,
                #[cfg(not(target_os = "netbsd"))]
                self.0.st_ctime_nsec.try_into().ok()?,
                #[cfg(target_os = "netbsd")]
                self.0.st_ctimensec.try_into().ok()?,
            ))
        }
        #[cfg(windows)]
        self.0.created().ok()
    }

    /// Return the size of the file in bytes.
    pub fn len(&self) -> u64 {
        #[cfg(not(windows))]
        {
            self.0.st_size as u64
        }
        #[cfg(windows)]
        self.0.len()
    }

    /// Return the device id on which the file is located, or 0 on windows.
    pub fn dev(&self) -> u64 {
        #[cfg(not(windows))]
        {
            self.0.st_dev as u64
        }
        #[cfg(windows)]
        0
    }

    /// Return the inode id tracking the file, or 0 on windows.
    pub fn ino(&self) -> u64 {
        #[cfg(not(windows))]
        {
            self.0.st_ino as u64
        }
        #[cfg(windows)]
        0
    }

    /// Return the user-id of the file or 0 on windows.
    pub fn uid(&self) -> u32 {
        #[cfg(not(windows))]
        {
            self.0.st_uid as u32
        }
        #[cfg(windows)]
        0
    }

    /// Return the group-id of the file or 0 on windows.
    pub fn gid(&self) -> u32 {
        #[cfg(not(windows))]
        {
            self.0.st_gid as u32
        }
        #[cfg(windows)]
        0
    }

    /// Return `true` if the file's executable bit is set, or `false` on windows.
    pub fn is_executable(&self) -> bool {
        #[cfg(not(windows))]
        {
            (self.0.st_mode as u32 & libc::S_IFMT as u32) == libc::S_IFREG as u32
                && self.0.st_mode as u32 & libc::S_IXUSR as u32 == libc::S_IXUSR as u32
        }
        #[cfg(windows)]
        gix_fs::is_executable(&self.0)
    }

    /// Return `true` if the file's is a symbolic link.
    pub fn is_symlink(&self) -> bool {
        #[cfg(not(windows))]
        {
            (self.0.st_mode as u32 & libc::S_IFMT as u32) == libc::S_IFLNK as u32
        }
        #[cfg(windows)]
        self.0.is_symlink()
    }

    /// Return `true` if this is a regular file, executable or not.
    pub fn is_file(&self) -> bool {
        #[cfg(not(windows))]
        {
            (self.0.st_mode as u32 & libc::S_IFMT as u32) == libc::S_IFREG as u32
        }
        #[cfg(windows)]
        self.0.is_file()
    }
}

#[cfg(not(windows))]
fn system_time_from_secs_nanos(secs: u64, nanos: u32) -> SystemTime {
    std::time::UNIX_EPOCH + std::time::Duration::new(secs, nanos)
}
