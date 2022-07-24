use std::{convert::TryInto, io::Read, path::PathBuf};

use crate::{
    store_impl::{file, file::log},
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
        Name: TryInto<&'a FullNameRef, Error = E>,
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
        Name: TryInto<&'a FullNameRef, Error = E>,
        crate::name::Error: From<E>,
    {
        let name: &FullNameRef = name.try_into().map_err(|err| Error::RefnameValidation(err.into()))?;
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
        Name: TryInto<&'a FullNameRef, Error = E>,
        crate::name::Error: From<E>,
    {
        let name: &FullNameRef = name.try_into().map_err(|err| Error::RefnameValidation(err.into()))?;
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
            #[cfg(windows)]
            Err(err) if err.kind() == std::io::ErrorKind::PermissionDenied => Ok(None),
            Err(err) => Err(err.into()),
        }
    }
}

impl file::Store {
    /// Implements the logic required to transform a fully qualified refname into its log name
    pub(crate) fn reflog_path(&self, name: &FullNameRef) -> PathBuf {
        let (base, rela_path) = self.reflog_base_and_relative_path(name);
        base.join(rela_path)
    }
}

///
pub mod create_or_update {
    use std::{
        borrow::Cow,
        io::Write,
        path::{Path, PathBuf},
    };

    use git_hash::{oid, ObjectId};
    use git_object::bstr::BStr;

    use crate::store_impl::{file, file::WriteReflog};

    impl file::Store {
        #[allow(clippy::too_many_arguments)]
        pub(crate) fn reflog_create_or_append(
            &self,
            name: &FullNameRef,
            _lock: &git_lock::Marker,
            previous_oid: Option<ObjectId>,
            new: &oid,
            committer: git_actor::SignatureRef<'_>,
            message: &BStr,
            mut force_create_reflog: bool,
        ) -> Result<(), Error> {
            let (reflog_base, full_name) = self.reflog_base_and_relative_path(name);
            match self.write_reflog {
                WriteReflog::Normal | WriteReflog::Always => {
                    if self.write_reflog == WriteReflog::Always {
                        force_create_reflog = true;
                    }
                    let mut options = std::fs::OpenOptions::new();
                    options.append(true).read(false);
                    let log_path = reflog_base.join(&full_name);

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
                                        reflog_path: self.reflog_path(name),
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
                        write!(file, "{} {} ", previous_oid.unwrap_or_else(|| new.kind().null()), new)
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
                                reflog_path: self.reflog_path(name),
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
                || full_name.starts_with("refs/worktree/") // NOTE: git does not write reflogs for worktree private refs
                || full_name == Path::new("HEAD")
        }

        /// Returns the base paths for all reflogs
        pub(in crate::store_impl::file) fn reflog_base_and_relative_path<'a>(
            &self,
            name: &'a FullNameRef,
        ) -> (PathBuf, Cow<'a, Path>) {
            let is_reflog = true;
            let (base, name) = self.to_base_dir_and_relative_name(name, is_reflog);
            (
                base.join("logs"),
                match &self.namespace {
                    None => git_path::to_native_path_on_windows(name.as_bstr()),
                    Some(namespace) => git_path::to_native_path_on_windows(
                        namespace.to_owned().into_namespaced_name(name).into_inner(),
                    ),
                },
            )
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

    use crate::FullNameRef;
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
