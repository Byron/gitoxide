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
    use crate::{store::file, transaction::LogChange};
    use git_hash::{oid, ObjectId};

    impl file::Store {
        pub(crate) fn create_or_append_reflog(
            &self,
            _lock: &git_lock::Marker,
            _previous_oid: Option<ObjectId>,
            _new: &oid,
            _log: &LogChange,
        ) -> Result<(), Error> {
            todo!("implement creation or appending to a ref log")
        }

        fn reflock_resource_to_log_path(&self, reflock: &git_lock::Marker) -> PathBuf {
            self.reflog_path_inner(
                reflock
                    .resource_path()
                    .strip_prefix(&self.base)
                    .expect("lock must be held within this store"),
            )
        }

        pub(in crate::store::file::loose::reflog) fn reflog_path_inner(&self, name: &Path) -> PathBuf {
            self.base.join("logs").join(name)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::{file::WriteReflog, FullName};
        use git_lock::acquire::Fail;
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
        #[ignore]
        fn missing_reflog_creates_it() {
            for mode in WRITE_MODES {
                let (_keep, store) = empty_store(*mode).unwrap();
                let lock = reflock(&store, "refs/heads/main").unwrap();
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
                TBD
            }
        }
    }
    pub use error::Error;
    use std::path::{Path, PathBuf};
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
