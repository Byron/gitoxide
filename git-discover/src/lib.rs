//! Find git repositories or search them upwards from a starting point, or determine if a directory looks like a git repository.
//!
//! Note that detection methods are educated guesses using the presence of files, without looking too much into the details.
#![forbid(unsafe_code, rust_2018_idioms)]
#![deny(missing_docs)]

/// The name of the `.git` directory.
pub const DOT_GIT_DIR: &str = ".git";

///
pub mod repository;

///
pub mod is_git {
    use std::path::PathBuf;

    /// The error returned by [`crate::is_git()`].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("Could not find a valid HEAD reference")]
        FindHeadRef(#[from] git_ref::file::find::existing::Error),
        #[error("Expected HEAD at '.git/HEAD', got '.git/{}'", .name)]
        MisplacedHead { name: bstr::BString },
        #[error("Expected an objects directory at '{}'", .missing.display())]
        MissingObjectsDirectory { missing: PathBuf },
        #[error("The worktree's private repo's commondir file at '{}' or it could not be read", .missing.display())]
        MissingCommonDir { missing: PathBuf, source: std::io::Error },
        #[error("Expected a refs directory at '{}'", .missing.display())]
        MissingRefsDirectory { missing: PathBuf },
        #[error(transparent)]
        GitFile(#[from] crate::path::from_gitdir_file::Error),
    }
}

mod is;
pub use is::{bare as is_bare, git as is_git};

///
pub mod upwards;
pub use upwards::function::{discover as upwards, discover_opts as upwards_opts};

///
pub mod path;

///
pub mod parse;
