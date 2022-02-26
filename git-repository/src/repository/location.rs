impl crate::Repository {
    /// The path to the `.git` directory itself, or equivalent if this is a bare repository.
    pub fn path(&self) -> &std::path::Path {
        self.git_dir()
    }

    /// Return the work tree containing all checked out files, if there is one.
    pub fn work_tree(&self) -> Option<&std::path::Path> {
        self.work_tree.as_deref()
    }

    /// Return the kind of repository, either bare or one with a work tree.
    pub fn kind(&self) -> crate::Kind {
        match self.work_tree {
            Some(_) => crate::Kind::WorkTree,
            None => crate::Kind::Bare,
        }
    }

    /// Return the path to the repository itself, containing objects, references, configuration, and more.
    ///
    /// Synonymous to [`path()`][crate::Repository::path()].
    pub fn git_dir(&self) -> &std::path::Path {
        self.refs.base()
    }

    // TODO: tests
    /// Load the index file of this repository's workspace, if present.
    ///
    /// Note that it is loaded into memory each time this method is called, but also is independent of the workspace.
    #[cfg(feature = "git-index")]
    pub fn load_index(&self) -> Option<Result<git_index::File, git_index::file::init::Error>> {
        // TODO: choose better/correct options
        let opts = git_index::decode::Options {
            object_hash: self.object_hash,
            thread_limit: None,
            min_extension_block_in_bytes_for_threading: 1024 * 256,
        };
        match git_index::File::at(self.git_dir().join("index"), opts) {
            Ok(index) => Some(Ok(index)),
            Err(git_index::file::init::Error::Io(err)) if err.kind() == std::io::ErrorKind::NotFound => None,
            Err(err) => Some(Err(err)),
        }
    }
}
