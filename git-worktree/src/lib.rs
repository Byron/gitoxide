#![forbid(unsafe_code, rust_2018_idioms)]

pub mod index {
    use git_hash::oid;

    pub mod checkout {
        use quick_error::quick_error;

        #[derive(Clone, Copy)]
        pub struct Options {
            pub symlinks: bool,
        }

        impl Default for Options {
            fn default() -> Self {
                Options { symlinks: true }
            }
        }

        quick_error! {
            #[derive(Debug)]
            pub enum Error {
                PathToUtf8(err: git_object::bstr::Utf8Error) {
                    from()
                    source(err)
                    display("Could not convert path to UTF8")
                }
                Time(err: std::time::SystemTimeError) {
                    from()
                    source(err)
                    display("The clock was off when reading file related metadata after updating a file on disk")
                }
                Io(err: std::io::Error) {
                    from()
                    source(err)
                    display("IO error while writing blob or reading file metadata or changing filetype")
                }
                ObjectNotFound(oid: git_hash::ObjectId, path: std::path::PathBuf) {
                    display("object {} for checkout at {} not found in object database", oid.to_hex(), path.display())
                }
            }
        }
    }

    pub fn checkout<Find>(
        index: &mut git_index::State,
        path: impl AsRef<std::path::Path>,
        mut find: Find,
        options: checkout::Options,
    ) -> Result<(), checkout::Error>
    where
        Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Option<git_object::BlobRef<'a>>,
    {
        let root = path.as_ref();
        let mut buf = Vec::new();
        for (entry, entry_path) in index.entries_mut_with_paths() {
            if entry.flags.contains(git_index::entry::Flags::SKIP_WORKTREE) {
                continue;
            }

            entry::checkout(entry, entry_path, &mut find, root, options, &mut buf)?;
        }
        Ok(())
    }

    pub(crate) mod entry {
        use std::{
            convert::TryInto,
            fs::{create_dir_all, OpenOptions},
            io::Write,
            time::Duration,
        };

        use git_hash::oid;
        use git_index::Entry;
        use git_object::bstr::{BStr, ByteSlice};

        use crate::index;

        pub fn checkout<Find>(
            entry: &mut Entry,
            entry_path: &BStr,
            find: &mut Find,
            root: &std::path::Path,
            index::checkout::Options { symlinks }: index::checkout::Options,
            mut buf: &mut Vec<u8>,
        ) -> Result<(), index::checkout::Error>
        where
            Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Option<git_object::BlobRef<'a>>,
        {
            let dest = root.join(entry_path.to_path()?); // TODO: try to use os_str_bytes to avoid UTF8 conversion. Put that into git-ref too
            create_dir_all(dest.parent().expect("entry paths are never empty"))?; // TODO: can this be avoided to create dirs when needed only?

            match entry.mode {
                git_index::entry::Mode::FILE | git_index::entry::Mode::FILE_EXECUTABLE => {
                    let obj = find(&entry.id, &mut buf)
                        .ok_or_else(|| index::checkout::Error::ObjectNotFound(entry.id, root.to_path_buf()))?;
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
                        .map_or(Ok(Duration::default()), |x| x.duration_since(std::time::UNIX_EPOCH))?;
                    let mtime = met
                        .modified()
                        .map_or(Ok(Duration::default()), |x| x.duration_since(std::time::UNIX_EPOCH))?;

                    update_fstat(entry, ctime, mtime)?;
                }
                git_index::entry::Mode::SYMLINK => {
                    let obj = find(&entry.id, &mut buf)
                        .ok_or_else(|| index::checkout::Error::ObjectNotFound(entry.id, root.to_path_buf()))?;
                    let symlink_destination = obj.data.to_path()?;
                    if symlinks {
                        #[cfg(unix)]
                        std::os::unix::fs::symlink(symlink_destination, &dest)?;
                        #[cfg(windows)]
                        if dest.exists() {
                            if dest.is_file() {
                                std::os::windows::fs::symlink_file(symlink_destination, &dest)?;
                            } else {
                                std::os::windows::fs::symlink_dir(symlink_destination, &dest)?;
                            }
                        }
                    } else {
                        std::fs::write(&dest, obj.data)?;
                    }
                    let met = std::fs::symlink_metadata(&dest)?;
                    let ctime = met
                        .created()
                        .map_or(Ok(Duration::default()), |x| x.duration_since(std::time::UNIX_EPOCH))?;
                    let mtime = met
                        .modified()
                        .map_or(Ok(Duration::default()), |x| x.duration_since(std::time::UNIX_EPOCH))?;
                    update_fstat(entry, ctime, mtime)?;
                }
                git_index::entry::Mode::DIR => todo!(),
                git_index::entry::Mode::COMMIT => todo!(),
                _ => unreachable!(),
            }
            Ok(())
        }

        fn update_fstat(entry: &mut Entry, ctime: Duration, mtime: Duration) -> Result<(), index::checkout::Error> {
            let stat = &mut entry.stat;
            stat.mtime.secs = mtime
                .as_secs()
                .try_into()
                .expect("by 2038 we found a solution for this");
            stat.mtime.nsecs = mtime.subsec_nanos();
            stat.ctime.secs = ctime
                .as_secs()
                .try_into()
                .expect("by 2038 we found a solution for this");
            stat.ctime.nsecs = ctime.subsec_nanos();
            Ok(())
        }
    }
}
