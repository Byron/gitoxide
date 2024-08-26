pub(crate) type CommitsStorage =
    gix_features::threading::OwnShared<gix_fs::SharedFileSnapshotMut<Vec<gix_hash::ObjectId>>>;
/// A lazily loaded and auto-updated list of commits which are at the shallow boundary (behind which there are no commits available),
/// sorted to allow bisecting.
pub type Commits = gix_fs::SharedFileSnapshot<Vec<gix_hash::ObjectId>>;

///
#[cfg(any(feature = "blocking-network-client", feature = "async-network-client"))]
pub mod write {
    pub(crate) mod function {
        use std::io::Write;

        use gix_protocol::fetch::response::ShallowUpdate;

        use crate::shallow::{write::Error, Commits};

        /// Write the previously obtained (possibly non-existing) `shallow_commits` to the shallow `file`
        /// after applying all `updates`.
        ///
        /// If this leaves the list of shallow commits empty, the file is removed.
        ///
        /// ### Deviation
        ///
        /// Git also prunes the set of shallow commits while writing, we don't until we support some sort of pruning.
        pub fn write(
            mut file: gix_lock::File,
            shallow_commits: Option<Commits>,
            updates: &[ShallowUpdate],
        ) -> Result<(), Error> {
            let mut shallow_commits = shallow_commits.map(|sc| (**sc).to_owned()).unwrap_or_default();
            for update in updates {
                match update {
                    ShallowUpdate::Shallow(id) => {
                        shallow_commits.push(*id);
                    }
                    ShallowUpdate::Unshallow(id) => shallow_commits.retain(|oid| oid != id),
                }
            }
            if shallow_commits.is_empty() {
                std::fs::remove_file(file.resource_path())?;
                drop(file);
                return Ok(());
            }

            if shallow_commits.is_empty() {
                if let Err(err) = std::fs::remove_file(file.resource_path()) {
                    if err.kind() != std::io::ErrorKind::NotFound {
                        return Err(err.into());
                    }
                }
            } else {
                shallow_commits.sort();
                let mut buf = Vec::<u8>::new();
                for commit in shallow_commits {
                    commit.write_hex_to(&mut buf).map_err(Error::Io)?;
                    buf.push(b'\n');
                }
                file.write_all(&buf).map_err(Error::Io)?;
                file.flush()?;
            }
            file.commit()?;
            Ok(())
        }
    }

    /// The error returned by [`write()`][crate::shallow::write()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        Commit(#[from] gix_lock::commit::Error<gix_lock::File>),
        #[error("Could not remove an empty shallow file")]
        RemoveEmpty(#[from] std::io::Error),
        #[error("Failed to write object id to shallow file")]
        Io(std::io::Error),
    }
}
#[cfg(any(feature = "blocking-network-client", feature = "async-network-client"))]
pub use write::function::write;

///
pub mod open {
    /// The error returned by [`Repository::shallow_commits()`][crate::Repository::shallow_commits()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("Could not open shallow file for reading")]
        Io(#[from] std::io::Error),
        #[error("Could not decode a line in shallow file as hex-encoded object hash")]
        DecodeHash(#[from] gix_hash::decode::Error),
    }
}
