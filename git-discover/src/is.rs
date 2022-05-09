use std::borrow::Cow;
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
/// Returns the Kind of git directory that was passed, possibly alongside the supporting private worktree git dir
///
/// Note that `.git` files are followed to a valid git directory, which then requires
///
/// * a valid head
/// * an objects directory
/// * a refs directory
pub fn git(git_dir: impl AsRef<Path>) -> Result<crate::repository::Kind, crate::is_git::Error> {
    let (dot_git, common_dir, git_dir_is_linked_worktree) = match crate::path::from_gitdir_file(git_dir.as_ref()) {
        Ok(private_git_dir) => {
            let common_dir = private_git_dir.join("commondir");
            let common_dir = crate::path::from_plain_file(&common_dir)
                .ok_or_else(|| crate::is_git::Error::MissingCommonDir {
                    missing: common_dir.clone(),
                })?
                .map_err(|_| crate::is_git::Error::MissingCommonDir { missing: common_dir })?;
            let common_dir = private_git_dir.join(common_dir);
            (Cow::Owned(private_git_dir), Cow::Owned(common_dir), true)
        }
        // TODO: use ::IsDirectory as well when stabilized, but it's permission denied on windows
        #[cfg(not(windows))]
        Err(crate::path::from_gitdir_file::Error::Io(err)) if err.raw_os_error() == Some(21) => {
            (Cow::Borrowed(git_dir.as_ref()), Cow::Borrowed(git_dir.as_ref()), false)
        }
        #[cfg(windows)]
        Err(crate::path::from_gitdir_file::Error::Io(err)) if err.kind() == std::io::ErrorKind::PermissionDenied => {
            (Cow::Borrowed(git_dir.as_ref()), Cow::Borrowed(git_dir.as_ref()), false)
        }
        Err(err) => return Err(err.into()),
    };

    {
        // We expect to be able to parse any ref-hash, so we shouldn't have to know the repos hash here.
        // With ref-table, the has is probably stored as part of the ref-db itself, so we can handle it from there.
        // In other words, it's important not to fail on detached heads here because we guessed the hash kind wrongly.
        let object_hash_should_not_matter_here = git_hash::Kind::Sha1;
        let refs = git_ref::file::Store::at(
            dot_git.as_ref(),
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
        let objects_path = common_dir.join("objects");
        if !objects_path.is_dir() {
            return Err(crate::is_git::Error::MissingObjectsDirectory { missing: objects_path });
        }
    }
    {
        let refs_path = common_dir.join("refs");
        if !refs_path.is_dir() {
            return Err(crate::is_git::Error::MissingRefsDirectory { missing: refs_path });
        }
    }

    Ok(if git_dir_is_linked_worktree {
        crate::repository::Kind::WorkTree {
            linked_git_dir: Some(dot_git.into_owned()),
        }
    } else if bare(git_dir) {
        crate::repository::Kind::Bare
    } else {
        crate::repository::Kind::WorkTree { linked_git_dir: None }
    })
}
