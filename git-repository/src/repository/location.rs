impl crate::Repository {
    /// Returns the main git repository if this is a repository on a linked work-tree, or the `git_dir` itself.
    pub fn common_dir(&self) -> &std::path::Path {
        self.common_dir.as_deref().unwrap_or_else(|| self.git_dir())
    }

    /// The path to the `.git` directory itself, or equivalent if this is a bare repository.
    pub fn path(&self) -> &std::path::Path {
        self.git_dir()
    }

    /// Return the work tree containing all checked out files, if there is one.
    pub fn work_dir(&self) -> Option<&std::path::Path> {
        self.work_tree.as_deref()
    }

    // TODO: tests, respect precomposeUnicode
    /// The directory of the binary path of the current process.
    pub fn install_dir(&self) -> std::io::Result<std::path::PathBuf> {
        crate::path::install_dir()
    }

    /// Returns the relative path which is the components between the working tree and the current working dir (CWD).
    /// Note that there may be `None` if there is no work tree, even though the `PathBuf` will be empty
    /// if the CWD is at the root of the work tree.
    // TODO: tests, details - there is a lot about environment variables to change things around.
    pub fn prefix(&self) -> Option<std::io::Result<std::path::PathBuf>> {
        self.work_tree.as_ref().map(|root| {
            std::env::current_dir().and_then(|cwd| {
                git_path::realpath(root, &cwd)
                    .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))
                    .and_then(|root| {
                        cwd.strip_prefix(&root)
                            .map_err(|_| {
                                std::io::Error::new(
                                    std::io::ErrorKind::Other,
                                    format!(
                                        "CWD '{}' isn't within the work tree '{}'",
                                        cwd.display(),
                                        root.display()
                                    ),
                                )
                            })
                            .map(ToOwned::to_owned)
                    })
            })
        })
    }

    /// Return the kind of repository, either bare or one with a work tree.
    pub fn kind(&self) -> crate::Kind {
        match self.worktree() {
            Some(wt) => crate::Kind::WorkTree {
                is_linked: !wt.is_main(),
            },
            None => crate::Kind::Bare,
        }
    }

    /// Return the path to the repository itself, containing objects, references, configuration, and more.
    ///
    /// Synonymous to [`path()`][crate::Repository::path()].
    pub fn git_dir(&self) -> &std::path::Path {
        self.refs.git_dir()
    }
}
