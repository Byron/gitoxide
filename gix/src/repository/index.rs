use gix_odb::FindExt;

use crate::{config::cache::util::ApplyLeniencyDefault, repository::IndexPersistedOrInMemory, worktree};

/// Index access
impl crate::Repository {
    /// Open a new copy of the index file and decode it entirely.
    ///
    /// It will use the `index.threads` configuration key to learn how many threads to use.
    /// Note that it may fail if there is no index.
    pub fn open_index(&self) -> Result<gix_index::File, worktree::open_index::Error> {
        let thread_limit = self
            .config
            .resolved
            .string("index", None, "threads")
            .map(|value| crate::config::tree::Index::THREADS.try_into_index_threads(value))
            .transpose()
            .with_lenient_default(self.config.lenient_config)?;
        gix_index::File::at(
            self.index_path(),
            self.object_hash(),
            gix_index::decode::Options {
                thread_limit,
                min_extension_block_in_bytes_for_threading: 0,
                expected_checksum: None,
            },
        )
        .map_err(Into::into)
    }

    /// Return a shared worktree index which is updated automatically if the in-memory snapshot has become stale as the underlying file
    /// on disk has changed.
    ///
    /// The index file is shared across all clones of this repository.
    pub fn index(&self) -> Result<worktree::Index, worktree::open_index::Error> {
        self.index
            .recent_snapshot(
                || self.index_path().metadata().and_then(|m| m.modified()).ok(),
                || {
                    self.open_index().map(Some).or_else(|err| match err {
                        worktree::open_index::Error::IndexFile(gix_index::file::init::Error::Io(err))
                            if err.kind() == std::io::ErrorKind::NotFound =>
                        {
                            Ok(None)
                        }
                        err => Err(err),
                    })
                },
            )
            .and_then(|opt| match opt {
                Some(index) => Ok(index),
                None => Err(worktree::open_index::Error::IndexFile(
                    gix_index::file::init::Error::Io(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        format!("Could not find index file at {:?} for opening.", self.index_path()),
                    )),
                )),
            })
    }

    /// Open the persisted worktree index or generate it from the current `HEAD^{tree}` to live in-memory only.
    ///
    /// Use this method to get an index in any repository, even bare ones that don't have one naturally.
    pub fn index_or_load_from_head(
        &self,
    ) -> Result<IndexPersistedOrInMemory, crate::repository::index_or_load_from_head::Error> {
        Ok(match self.index() {
            Ok(index) => IndexPersistedOrInMemory::Persisted(index),
            Err(worktree::open_index::Error::IndexFile(_)) => {
                let tree = self.head_commit()?.tree_id()?;
                IndexPersistedOrInMemory::InMemory(gix_index::File::from_state(
                    gix_index::State::from_tree(&tree, |oid, buf| self.objects.find_tree_iter(oid, buf).ok())?,
                    self.git_dir().join("index"),
                ))
            }
            Err(err) => return Err(err.into()),
        })
    }
}

impl std::ops::Deref for IndexPersistedOrInMemory {
    type Target = gix_index::File;

    fn deref(&self) -> &Self::Target {
        match self {
            IndexPersistedOrInMemory::Persisted(i) => i,
            IndexPersistedOrInMemory::InMemory(i) => i,
        }
    }
}

impl IndexPersistedOrInMemory {
    /// Consume this instance and turn it into an owned index file.
    ///
    /// Note that this will cause the persisted index to be cloned, which would happen whenever the repository has a worktree.
    pub fn into_owned(self) -> gix_index::File {
        match self {
            IndexPersistedOrInMemory::Persisted(i) => gix_index::File::clone(&i),
            IndexPersistedOrInMemory::InMemory(i) => i,
        }
    }
}
