use std::path::PathBuf;

use gix_path::realpath::MAX_SYMLINKS;

impl crate::Repository {
    /// Return the path to the repository itself, containing objects, references, configuration, and more.
    ///
    /// Synonymous to [`path()`][crate::Repository::path()].
    pub fn git_dir(&self) -> &std::path::Path {
        self.refs.git_dir()
    }

    /// The trust we place in the git-dir, with lower amounts of trust causing access to configuration to be limited.
    pub fn git_dir_trust(&self) -> gix_sec::Trust {
        self.options.git_dir_trust.expect("definitely set by now")
    }

    /// Returns the main git repository if this is a repository on a linked work-tree, or the `git_dir` itself.
    pub fn common_dir(&self) -> &std::path::Path {
        self.common_dir.as_deref().unwrap_or_else(|| self.git_dir())
    }

    /// Return the path to the worktree index file, which may or may not exist.
    pub fn index_path(&self) -> PathBuf {
        self.git_dir().join("index")
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
    pub fn install_dir(&self) -> std::io::Result<PathBuf> {
        crate::path::install_dir()
    }

    /// Returns the relative path which is the components between the working tree and the current working dir (CWD).
    /// Note that there may be `None` if there is no work tree, even though the `PathBuf` will be empty
    /// if the CWD is at the root of the work tree.
    // TODO: tests, details - there is a lot about environment variables to change things around.
    pub fn prefix(&self) -> Option<std::io::Result<PathBuf>> {
        self.work_tree.as_ref().map(|root| {
            std::env::current_dir().and_then(|cwd| {
                gix_path::realpath_opts(root, &cwd, MAX_SYMLINKS)
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
            Some(wt) => {
                if gix_discover::is_submodule_git_dir(self.git_dir()) {
                    crate::Kind::Submodule
                } else {
                    crate::Kind::WorkTree {
                        is_linked: !wt.is_main(),
                    }
                }
            }
            None => crate::Kind::Bare,
        }
    }
}
