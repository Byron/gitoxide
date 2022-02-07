#![forbid(unsafe_code, rust_2018_idioms)]

use git_hash::oid;
use git_object::bstr::ByteSlice;
use quick_error::quick_error;
use std::convert::TryFrom;
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Duration;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Utf8(err: git_object::bstr::Utf8Error) {
            from()
            display("Could not convert path to UTF8: {}", err)
        }
        Time(err: std::time::SystemTimeError) {
            from()
            display("Could not read file time in proper format: {}", err)
        }
        U32Conversion(err: std::num::TryFromIntError) {
            from()
            display("Could not convert seconds to u32: {}", err)
        }
        Io(err: std::io::Error) {
            from()
            display("IO error while writing blob or reading file metadata or changing filetype: {}", err)
        }
        NotFound(oid: git_hash::ObjectId, path: PathBuf) {
            display("unable find object of {} ({})", path.display(), oid.to_hex())
        }
    }
}

/// Copy index to `path`
pub fn copy_index<Find>(
    index: &mut git_index::State,
    path: impl AsRef<Path>,
    mut find: Find,
    opts: Options,
) -> Result<(), Error>
where
    Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Option<git_object::BlobRef<'a>>,
{
    let path = path.as_ref();
    let mut buf = Vec::new();
    let mut entry_time = Vec::new(); // Entries whose timestamps have to be updated
    for (i, entry) in index.entries().iter().enumerate() {
        if entry.flags.contains(git_index::entry::Flags::SKIP_WORKTREE) {
            continue;
        }
        let entry_path = entry.path(index).to_path()?;
        let dest = path.join(entry_path);
        create_dir_all(dest.parent().expect("entry paths are never empty"))?;

        match entry.mode {
            git_index::entry::Mode::FILE | git_index::entry::Mode::FILE_EXECUTABLE => {
                let obj = find(&entry.id, &mut buf).ok_or_else(|| Error::NotFound(entry.id, path.to_path_buf()))?;
                let mut options = OpenOptions::new();
                options.write(true).create_new(true);
                #[cfg(unix)]
                if entry.mode == git_index::entry::Mode::FILE_EXECUTABLE {
                    use std::os::unix::fs::OpenOptionsExt;
                    options.mode(0o777);
                }
                let mut file = options.open(&dest)?;
                file.write_all(obj.data)?;
                let met = file.metadata()?;
                let ctime = met
                    .created()
                    .map_or(Ok(Duration::default()), |x| x.duration_since(std::time::UNIX_EPOCH));
                let mtime = met
                    .modified()
                    .map_or(Ok(Duration::default()), |x| x.duration_since(std::time::UNIX_EPOCH));
                entry_time.push((ctime?, mtime?, i));
            }
            git_index::entry::Mode::SYMLINK => {
                let obj = find(&entry.id, &mut buf).ok_or_else(|| Error::NotFound(entry.id, path.to_path_buf()))?;
                let linked_to = obj.data.to_path()?;
                if opts.symlinks {
                    #[cfg(unix)]
                    std::os::unix::fs::symlink(linked_to, &dest)?;
                    #[cfg(windows)]
                    if dest.exists() {
                        if dest.is_file() {
                            std::os::windows::fs::symlink_file(linked_to, &dest)?;
                        } else {
                            std::os::windows::fs::symlink_dir(linked_to, &dest)?;
                        }
                    }
                } else {
                    std::fs::write(&dest, obj.data)?;
                }
                let met = std::fs::symlink_metadata(&dest)?;
                let ctime = met
                    .created()
                    .map_or(Ok(Duration::default()), |x| x.duration_since(std::time::UNIX_EPOCH));
                let mtime = met
                    .modified()
                    .map_or(Ok(Duration::default()), |x| x.duration_since(std::time::UNIX_EPOCH));
                entry_time.push((ctime?, mtime?, i));
            }
            git_index::entry::Mode::DIR => todo!(),
            git_index::entry::Mode::COMMIT => todo!(),
            _ => unreachable!(),
        }
    }
    let entries = index.entries_mut();
    for (ctime, mtime, i) in entry_time {
        let stat = &mut entries[i].stat;
        stat.mtime.secs = u32::try_from(mtime.as_secs())?;
        stat.mtime.nsecs = mtime.subsec_nanos();
        stat.ctime.secs = u32::try_from(ctime.as_secs())?;
        stat.ctime.nsecs = ctime.subsec_nanos();
    }
    Ok(())
}

/// Options for [copy_index](crate::copy_index)
pub struct Options {
    /// Enable/disable symlinks
    pub symlinks: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options { symlinks: true }
    }
}
