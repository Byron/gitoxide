use std::path::{Path, PathBuf};

use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        FindHeadRef(err: git_ref::file::find::existing::Error) {
            display("Could not find a valid HEAD reference")
            from()
            source(err)
        }
        MisplacedHead(name: crate::object::bstr::BString) {
            display("Expected HEAD at '.git/HEAD', got '.git/{}'", name)
        }
        MissingObjectsDirectory(missing: PathBuf) {
            display("Expected an objects directory at '{}'", missing.display())
        }
        MissingRefsDirectory(missing: PathBuf) {
            display("Expected a refs directory at '{}'", missing.display())
        }
    }
}

/// Returns true if the given `git_dir` seems to be a bare repository.
///
/// Please note that repositories without any file in their working tree will also appear bare.
pub fn is_bare(git_dir: impl AsRef<Path>) -> bool {
    !git_dir.as_ref().join("index").exists()
}

/// What constitutes a valid git repository, and what's yet to be implemented.
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
        let head = refs.loose_find_existing("HEAD")?;
        if head.name.as_bstr() != "HEAD" {
            return Err(Error::MisplacedHead(head.name.into_inner()));
        }
    }

    {
        let objects_path = std::env::var("GIT_OBJECT_DIRECTORY")
            .map(PathBuf::from)
            .unwrap_or_else(|_| dot_git.join("objects"));
        if !objects_path.is_dir() {
            return Err(Error::MissingObjectsDirectory(objects_path));
        }
    }
    {
        let refs_path = dot_git.join("refs");
        if !refs_path.is_dir() {
            return Err(Error::MissingRefsDirectory(refs_path));
        }
    }

    Ok(if is_bare(git_dir) {
        crate::Kind::Bare
    } else {
        crate::Kind::WorkingTree
    })
}
