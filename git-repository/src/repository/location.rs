impl crate::Repository {
    /// The path to the `.git` directory itself, or equivalent if this is a bare repository.
    pub fn path(&self) -> &std::path::Path {
        self.git_dir()
    }

    /// Return the work tree containing all checked out files, if there is one.
    pub fn work_tree(&self) -> Option<&std::path::Path> {
        self.work_tree.as_deref()
    }

    /// The directory of the binary path of the current process.
    pub fn install_directory(&self) -> std::io::Result<std::path::PathBuf> {
        std::env::current_exe()
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
}
