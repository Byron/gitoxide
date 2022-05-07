use std::{ffi::OsStr, path::Path};

/// Returns true if the given `git_dir` seems to be a bare repository.
///
/// Please note that repositories without an index generally _look_ bare, even though they might also be uninitialized.
pub fn bare(git_dir_candidate: impl AsRef<Path>) -> bool {
    let git_dir = git_dir_candidate.as_ref();
    !(git_dir.join("index").exists() || (git_dir.file_name() == Some(OsStr::new(".git")) && git_dir.is_file()))
}

/// What constitutes a valid git repository, returning the guessed repository kind
/// purely based on the presence of files. Note that the git-config ultimately decides what's bare.
///
/// * [ ] git files
/// * [x] a valid head
/// * [ ] git common directory
/// * [x] an objects directory
/// * [x] a refs directory
// TODO: allow configuring common dirs at least
pub fn git(git_dir: impl AsRef<Path>) -> Result<crate::repository::Kind, crate::is_git::Error> {
    let dot_git = git_dir.as_ref();

    {
        // We expect to be able to parse any ref-hash, so we shouldn't have to know the repos hash here.
        // With ref-table, the has is probably stored as part of the ref-db itself, so we can handle it from there.
        // In other words, it's important not to fail on detached heads here because we guessed the hash kind wrongly.
        let object_hash_should_not_matter_here = git_hash::Kind::Sha1;
        let refs = git_ref::file::Store::at(
            &dot_git,
            git_ref::store::WriteReflog::Normal,
            object_hash_should_not_matter_here,
        );
        let head = refs.find_loose("HEAD")?;
        if head.name.as_bstr() != "HEAD" {
            return Err(crate::is_git::Error::MisplacedHead {
                name: head.name.into_inner(),
            });
        }
    }

    {
        let objects_path = dot_git.join("objects");
        if !objects_path.is_dir() {
            return Err(crate::is_git::Error::MissingObjectsDirectory { missing: objects_path });
        }
    }
    {
        let refs_path = dot_git.join("refs");
        if !refs_path.is_dir() {
            return Err(crate::is_git::Error::MissingRefsDirectory { missing: refs_path });
        }
    }

    Ok(if bare(git_dir) {
        crate::repository::Kind::Bare
    } else {
        crate::repository::Kind::WorkTree
    })
}
