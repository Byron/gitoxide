use crate::{
    store::{file, file::log},
    FullName,
};
use std::{convert::TryInto, io::Read, path::PathBuf};

impl file::Store {
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
        Name: TryInto<FullName<'a>, Error = E>,
        crate::name::Error: From<E>,
    {
        let name: FullName<'_> = name.try_into().map_err(|err| Error::RefnameValidation(err.into()))?;
        let file = match std::fs::File::open(self.reflog_path(name)) {
            Ok(file) => file,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(err) => return Err(err.into()),
        };
        Ok(Some(log::iter::reverse(file, buf)?))
    }

    /// Return a reflog forward iterator for the given fully qualified `name` and write its file contents into `buf`.
    ///
    /// The iterator will traverse log entries from oldest to newest.
    /// Return `Ok(None)` if no reflog exists.
    pub fn reflog_iter<'a, 'b, Name, E>(
        &self,
        name: Name,
        buf: &'b mut Vec<u8>,
    ) -> Result<Option<impl Iterator<Item = Result<log::Line<'b>, log::iter::decode::Error>>>, Error>
    where
        Name: TryInto<FullName<'a>, Error = E>,
        crate::name::Error: From<E>,
    {
        let name: FullName<'_> = name.try_into().map_err(|err| Error::RefnameValidation(err.into()))?;
        match std::fs::File::open(self.reflog_path(name)) {
            Ok(mut file) => {
                buf.clear();
                file.read_to_end(buf)?;
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(err) => return Err(err.into()),
        };
        Ok(Some(log::iter::forward(buf)))
    }
}

impl file::Store {
    /// Implements the logic required to transform a fully qualified refname into its log name
    pub(crate) fn reflog_path(&self, name: FullName<'_>) -> PathBuf {
        self.reflog_path_inner(&name.to_path())
    }
}

///
pub mod create_or_update {
    use crate::store::{file, file::WriteReflog};
    use bstr::BStr;
    use git_hash::{oid, ObjectId};
    use std::path::{Path, PathBuf};

    impl file::Store {
        pub(crate) fn reflog_create_or_append(
            &self,
            lock: &git_lock::Marker,
            _previous_oid: Option<ObjectId>,
            _new: &oid,
            _committer: &git_actor::Signature,
            _message: &BStr,
            force_create_reflog: bool,
        ) -> Result<(), Error> {
            let full_name = self.reflock_resource_full_name(lock);
            match self.write_reflog {
                WriteReflog::Normal => {
                    let mut options = std::fs::OpenOptions::new();
                    options.append(true).read(false);
                    // let log_path = self.reflock_resource_to_log_path(lock);
                    let _possibly_file: Option<std::fs::File> =
                        if force_create_reflog || self.should_autocreate_reflog(&full_name) {
                            // git_tempfile::create_dir::all()
                            options.create(true);
                            todo!("open with creation")
                        } else {
                            todo!("open without creation")
                        };
                    todo!("write actual content if file is set")
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
            self.base.join("logs").join(full_name)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::{file::WriteReflog, store::file::log, FullName};
        use bstr::ByteSlice;
        use git_actor::{Sign, Signature, Time};
        use git_lock::acquire::Fail;
        use git_testtools::hex_to_id;
        use std::{convert::TryInto, path::Path};
        use tempfile::TempDir;

        type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

        fn empty_store(writemode: WriteReflog) -> Result<(TempDir, file::Store)> {
            let dir = TempDir::new()?;
            let store = file::Store::at(dir.path(), writemode);
            Ok((dir, store))
        }
        fn reflock(store: &file::Store, full_name: &str) -> Result<git_lock::Marker> {
            let full_name: FullName<'_> = full_name.try_into()?;
            git_lock::Marker::acquire_to_hold_resource(
                store.ref_path(&full_name.to_path()),
                Fail::Immediately,
                Some(store.base.clone()),
            )
            .map_err(Into::into)
        }
        fn reflog_iter(store: &file::Store, name: &str, buf: &mut Vec<u8>) -> Result<Vec<log::mutable::Line>> {
            store
                .reflog_iter(name, buf)?
                .expect("existing reflog")
                .map(|l| l.map(log::mutable::Line::from))
                .collect::<std::result::Result<Vec<_>, _>>()
                .map_err(Into::into)
        }

        const WRITE_MODES: &[WriteReflog] = &[WriteReflog::Normal, WriteReflog::Disable];

        #[test]
        fn reflock_resource_to_log_path() {
            let (_keep, store) = empty_store(WriteReflog::Normal).unwrap();
            for name in &["HEAD", "refs/heads/main"] {
                assert_eq!(
                    store.reflock_resource_to_log_path(&reflock(&store, name).unwrap()),
                    store.reflog_path_inner(Path::new(name))
                );
            }
        }

        #[test]
        fn should_autocreate_is_unaffected_by_writemode() {
            let (_keep, store) = empty_store(WriteReflog::Disable).unwrap();
            for should_create_name in &["HEAD", "refs/heads/main", "refs/remotes/any", "refs/notes/any"] {
                assert!(store.should_autocreate_reflog(Path::new(should_create_name)));
            }
            for should_not_create_name in &["FETCH_HEAD", "SOMETHING", "refs/special/this", "refs/tags/0.1.0"] {
                assert!(!store.should_autocreate_reflog(Path::new(should_not_create_name)));
            }
        }

        #[test]
        #[ignore]
        fn missing_reflog_creates_it_even_if_similarly_named_empty_dir_exists_and_append_log_lines() {
            for mode in WRITE_MODES {
                let (_keep, store) = empty_store(*mode).unwrap();
                let full_name = "refs/heads/main";
                let lock = reflock(&store, full_name).unwrap();
                let new = hex_to_id("28ce6a8b26aa170e1de65536fe8abe1832bd3242");
                let committer = Signature {
                    name: "committer".into(),
                    email: "commiter@example.com".into(),
                    time: Time {
                        time: 1234,
                        offset: 1800,
                        sign: Sign::Plus,
                    },
                };
                store
                    .reflog_create_or_append(&lock, None, &new, &committer, b"the message".as_bstr(), false)
                    .unwrap();

                let mut buf = Vec::new();
                match mode {
                    WriteReflog::Normal => {
                        assert_eq!(
                            reflog_iter(&store, full_name, &mut buf).unwrap(),
                            vec![log::mutable::Line {
                                previous_oid: ObjectId::null_sha1(),
                                new_oid: new,
                                signature: committer.clone(),
                                message: "the message".into()
                            }]
                        );
                        let previous = hex_to_id("0000000000000000000000111111111111111111");
                        store
                            .reflog_create_or_append(
                                &lock,
                                Some(previous.clone()),
                                &new,
                                &committer,
                                b"next message".as_bstr(),
                                false,
                            )
                            .unwrap();

                        let lines = reflog_iter(&store, full_name, &mut buf).unwrap();
                        assert_eq!(lines.len(), 2, "now there is another line");
                        assert_eq!(
                            lines.last().expect("non-empty"),
                            &log::mutable::Line {
                                previous_oid: previous,
                                new_oid: new,
                                signature: committer.clone(),
                                message: "other message".into()
                            }
                        );
                    }
                    WriteReflog::Disable => {
                        assert!(
                            store.reflog_iter(full_name, &mut buf).unwrap().is_none(),
                            "there is no logs in disabled mode"
                        );
                    }
                };

                // create onto existing directory
                let full_name = "refs/heads/other";
                let lock = reflock(&store, full_name).unwrap();
                let reflog_path = store.reflog_path_inner(Path::new(full_name));
                std::fs::create_dir(&reflog_path).unwrap();

                store
                    .reflog_create_or_append(
                        &lock,
                        None,
                        &new,
                        &committer,
                        b"more complicated reflog creation".as_bstr(),
                        false,
                    )
                    .unwrap();

                match mode {
                    WriteReflog::Normal => {
                        assert_eq!(
                            reflog_iter(&store, full_name, &mut buf).unwrap().len(),
                            1,
                            "reflog was written despite directory"
                        );
                        assert!(
                            reflog_path.is_file(),
                            "the empty directory was replaced with the reflog file"
                        );
                    }
                    WriteReflog::Disable => {
                        assert!(
                            store.reflog_iter(full_name, &mut buf).unwrap().is_none(),
                            "reflog still doesn't exist"
                        );
                        assert!(reflog_path.is_dir(), "reflog directory wasn't touched");
                    }
                }
            }
        }
    }

    mod error {
        use quick_error::quick_error;

        quick_error! {
            /// The error returned when creating or appending to a reflog
            #[derive(Debug)]
            #[allow(missing_docs)]
            pub enum Error {
                MessageWithNewlines
            }
        }
    }
    pub use error::Error;
}

mod error {
    use quick_error::quick_error;
    use std::io;

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
