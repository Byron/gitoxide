#![allow(missing_docs)]
use std::path::{Path, PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not find a valid HEAD reference")]
    FindHeadRef(#[from] git_ref::file::find::existing::Error),
    #[error("Expected HEAD at '.git/HEAD', got '.git/{}'", .name)]
    MisplacedHead { name: bstr::BString },
    #[error("Expected an objects directory at '{}'", .missing.display())]
    MissingObjectsDirectory { missing: PathBuf },
    #[error("Expected a refs directory at '{}'", .missing.display())]
    MissingRefsDirectory { missing: PathBuf },
}

/// Returns true if the given `git_dir` seems to be a bare repository.
///
/// Please note that repositories without any file in their work tree will also appear bare.
pub fn is_bare(git_dir: impl AsRef<Path>) -> bool {
    !git_dir.as_ref().join("index").exists()
}

/// What constitutes a valid git repository, and what's yet to be implemented, returning the guessed repository kind
/// purely based on the presence of files. Note that the git-config ultimately decides what's bare.
///
/// * [x] a valid head
/// * [ ] git common directory
///   * [ ] respect GIT_COMMON_DIR
/// * [x] an objects directory
///   * [x] respect GIT_OBJECT_DIRECTORY
/// * [x] a refs directory
pub fn is_git(git_dir: impl AsRef<Path>) -> Result<crate::Kind, Error> {
    let dot_git = git_dir.as_ref();

    {
        let refs = git_ref::file::Store::at(&dot_git, Default::default());
        let head = refs.find_loose("HEAD")?;
        if head.name.as_bstr() != "HEAD" {
            return Err(Error::MisplacedHead {
                name: head.name.into_inner(),
            });
        }
    }

    {
        let objects_path = std::env::var("GIT_OBJECT_DIRECTORY")
            .map(PathBuf::from)
            .unwrap_or_else(|_| dot_git.join("objects"));
        if !objects_path.is_dir() {
            return Err(Error::MissingObjectsDirectory { missing: objects_path });
        }
    }
    {
        let refs_path = dot_git.join("refs");
        if !refs_path.is_dir() {
            return Err(Error::MissingRefsDirectory { missing: refs_path });
        }
    }

    Ok(if is_bare(git_dir) {
        crate::Kind::Bare
    } else {
        crate::Kind::WorkTree
    })
}
