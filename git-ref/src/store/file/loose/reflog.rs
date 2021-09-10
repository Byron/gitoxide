use std::{convert::TryInto, io::Read, path::PathBuf};

use crate::{
    store::{file, file::log},
    FullNameRef,
};

impl file::Store {
    /// Returns true if a reflog exists for the given reference `name`.
    ///
    /// Please note that this method shouldn't be used to check if a log exists before trying to read it, but instead
    /// is meant to be the fastest possible way to determine if a log exists or not.
    /// If the caller needs to know if it's readable, try to read the log instead with a reverse or forward iterator.
    pub fn reflog_exists<'a, Name, E>(&self, name: Name) -> Result<bool, E>
    where
        Name: TryInto<FullNameRef<'a>, Error = E>,
        crate::name::Error: From<E>,
    {
        Ok(self.reflog_path(name.try_into()?).is_file())
    }

    /// Return a reflog reverse iterator for the given fully qualified `name`, reading chunks from the back into the fixed buffer `buf`.
    ///
    /// The iterator will traverse log entries from most recent to oldest, reading the underlying file in chunks from the back.
    /// Return `Ok(None)` if no reflog exists.
    pub fn reflog_iter_rev<'a, 'b, Name, E>(
        &self,
        name: Name,
        buf: &'b mut [u8],
    ) -> Result<Option<log::iter::Reverse<'b, std::fs::File>>, Error>
    where
        Name: TryInto<FullNameRef<'a>, Error = E>,
        crate::name::Error: From<E>,
    {
        let name: FullNameRef<'_> = name.try_into().map_err(|err| Error::RefnameValidation(err.into()))?;
        let path = self.reflog_path(name);
        if path.is_dir() {
            return Ok(None);
        }
        match std::fs::File::open(&path) {
            Ok(file) => Ok(Some(log::iter::reverse(file, buf)?)),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err.into()),
        }
    }

    /// Return a reflog forward iterator for the given fully qualified `name` and write its file contents into `buf`.
    ///
    /// The iterator will traverse log entries from oldest to newest.
    /// Return `Ok(None)` if no reflog exists.
    pub fn reflog_iter<'a, 'b, Name, E>(
        &self,
        name: Name,
        buf: &'b mut Vec<u8>,
    ) -> Result<Option<log::iter::Forward<'b>>, Error>
    where
        Name: TryInto<FullNameRef<'a>, Error = E>,
        crate::name::Error: From<E>,
    {
        let name: FullNameRef<'_> = name.try_into().map_err(|err| Error::RefnameValidation(err.into()))?;
        let path = self.reflog_path(name);
        match std::fs::File::open(&path) {
            Ok(mut file) => {
                buf.clear();
                if let Err(err) = file.read_to_end(buf) {
                    return if path.is_dir() { Ok(None) } else { Err(err.into()) };
                }
                Ok(Some(log::iter::forward(buf)))
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
            #[cfg(target_os = "windows")]
            Err(err) if err.kind() == std::io::ErrorKind::PermissionDenied => Ok(None),
            Err(err) => Err(err.into()),
        }
    }
}

impl file::Store {
    /// Implements the logic required to transform a fully qualified refname into its log name
    pub(crate) fn reflog_path(&self, name: FullNameRef<'_>) -> PathBuf {
        self.reflog_path_inner(&name.to_path())
    }
}

///
pub mod create_or_update {
    use std::{
        io::Write,
        path::{Path, PathBuf},
    };

    use git_hash::{oid, ObjectId};
    use git_object::bstr::BStr;

    use crate::store::{file, file::WriteReflog};

    impl file::Store {
        pub(crate) fn reflog_create_or_append(
            &self,
            lock: &git_lock::Marker,
            previous_oid: Option<ObjectId>,
            new: &oid,
            committer: &git_actor::Signature,
            message: &BStr,
            force_create_reflog: bool,
        ) -> Result<(), Error> {
            let full_name = self.reflock_resource_full_name(lock);
            match self.write_reflog {
                WriteReflog::Normal => {
                    let mut options = std::fs::OpenOptions::new();
                    options.append(true).read(false);
                    let log_path = self.reflock_resource_to_log_path(lock);

                    if force_create_reflog || self.should_autocreate_reflog(&full_name) {
                        let parent_dir = log_path.parent().expect("always with parent directory");
                        git_tempfile::create_dir::all(parent_dir, Default::default()).map_err(|err| {
                            Error::CreateLeadingDirectories {
                                err,
                                reflog_directory: parent_dir.to_owned(),
                            }
                        })?;
                        options.create(true);
                    };

                    let file_for_appending = match options.open(&log_path) {
                        Ok(f) => Some(f),
                        Err(err) if err.kind() == std::io::ErrorKind::NotFound => None,
                        Err(err) => {
                            // TODO: when Kind::IsADirectory becomes stable, use that.
                            if log_path.is_dir() {
                                git_tempfile::remove_dir::empty_depth_first(&log_path)
                                    .and_then(|_| options.open(&log_path))
                                    .map(Some)
                                    .map_err(|_| Error::Append {
                                        err,
                                        reflog_path: self.reflock_resource_to_log_path(lock),
                                    })?
                            } else {
                                return Err(Error::Append {
                                    err,
                                    reflog_path: log_path,
                                });
                            }
                        }
                    };

                    if let Some(mut file) = file_for_appending {
                        write!(
                            file,
                            "{} {} ",
                            previous_oid.unwrap_or_else(|| ObjectId::null(new.kind())),
                            new
                        )
                        .and_then(|_| committer.write_to(&mut file))
                        .and_then(|_| {
                            if !message.is_empty() {
                                writeln!(file, "\t{}", message)
                            } else {
                                writeln!(file)
                            }
                        })
                        .map_err(|err| Error::Append {
                            err,
                            reflog_path: self.reflock_resource_to_log_path(lock),
                        })?;
                    }
                    Ok(())
                }
                WriteReflog::Disable => Ok(()),
            }
        }

        fn should_autocreate_reflog(&self, full_name: &Path) -> bool {
            full_name.starts_with("refs/heads/")
                || full_name.starts_with("refs/remotes/")
                || full_name.starts_with("refs/notes/")
                || full_name == Path::new("HEAD")
        }

        fn reflock_resource_full_name(&self, reflock: &git_lock::Marker) -> PathBuf {
            reflock
                .resource_path()
                .strip_prefix(&self.base)
                .expect("lock must be held within this store")
                .to_owned()
        }

        fn reflock_resource_to_log_path(&self, reflock: &git_lock::Marker) -> PathBuf {
            self.reflog_path_inner(
                reflock
                    .resource_path()
                    .strip_prefix(&self.base)
                    .expect("lock must be held within this store"),
            )
        }

        /// Returns the base and a full path (including the base) to the reflog for a ref of the given `full_name`
        pub(in crate::store::file::loose::reflog) fn reflog_path_inner(&self, full_name: &Path) -> PathBuf {
            self.reflog_root().join(full_name)
        }

        /// Returns the base paths for all reflogs
        pub(in crate::store::file) fn reflog_root(&self) -> PathBuf {
            self.base.join("logs")
        }
    }

    #[cfg(test)]
    mod tests;

    mod error {
        use std::path::PathBuf;

        use quick_error::quick_error;

        quick_error! {
            /// The error returned when creating or appending to a reflog
            #[derive(Debug)]
            #[allow(missing_docs)]
            pub enum Error {
                CreateLeadingDirectories { err: std::io::Error, reflog_directory: PathBuf } {
                    display("Could create one or more directories in '{}' to contain reflog file", reflog_directory.display())
                    source(err)
                }
                Append { err: std::io::Error, reflog_path: PathBuf } {
                    display("Could not open reflog file at '{}' for appending", reflog_path.display())
                    source(err)
                }
                MessageWithNewlines {
                    display("tbd")
                }
            }
        }
    }
    pub use error::Error;
}

mod error {
    use std::io;

    use quick_error::quick_error;

    quick_error! {
        /// The error returned by [crate::file::Store::reflog_iter()].
        #[derive(Debug)]
        #[allow(missing_docs)]
        pub enum Error {
            RefnameValidation(err: crate::name::Error) {
                display("The reflog name or path is not a valid ref name")
                from()
                source(err)
            }
            Io(err: io::Error) {
                display("The reflog file could not read")
                from()
                source(err)
            }
        }
    }
}
pub use error::Error;
