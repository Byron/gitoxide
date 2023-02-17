use std::{borrow::Cow, ffi::OsStr, path::Path};

use crate::{DOT_GIT_DIR, MODULES};

/// Returns true if the given `git_dir` seems to be a bare repository.
///
/// Please note that repositories without an index generally _look_ bare, even though they might also be uninitialized.
pub fn bare(git_dir_candidate: impl AsRef<Path>) -> bool {
    let git_dir = git_dir_candidate.as_ref();
    !(git_dir.join("index").exists() || (git_dir.file_name() == Some(OsStr::new(DOT_GIT_DIR))))
}

/// Returns true if `git_dir` is is located within a `.git/modules` directory, indicating it's a submodule clone.
pub fn submodule_git_dir(git_dir: impl AsRef<Path>) -> bool {
    let git_dir = git_dir.as_ref();

    let mut last_comp = None;
    git_dir.file_name() != Some(OsStr::new(DOT_GIT_DIR))
        && git_dir.components().rev().any(|c| {
            if c.as_os_str() == OsStr::new(DOT_GIT_DIR) {
                true
            } else {
                last_comp = Some(c.as_os_str());
                false
            }
        })
        && last_comp == Some(OsStr::new(MODULES))
}

/// What constitutes a valid git repository, returning the guessed repository kind
/// purely based on the presence of files. Note that the gix-config ultimately decides what's bare.
///
/// Returns the `Kind` of git directory that was passed, possibly alongside the supporting private worktree git dir.
///
/// Note that `.git` files are followed to a valid git directory, which then requires…
///
///   * …a valid head
///   * …an objects directory
///   * …a refs directory
///
pub fn git(git_dir: impl AsRef<Path>) -> Result<crate::repository::Kind, crate::is_git::Error> {
    #[derive(Eq, PartialEq)]
    enum Kind {
        MaybeRepo,
        Submodule,
        LinkedWorkTreeDir,
        WorkTreeGitDir { work_dir: std::path::PathBuf },
    }
    let git_dir = git_dir.as_ref();
    let (dot_git, common_dir, kind) = if git_dir
        .metadata()
        .map_err(|err| crate::is_git::Error::Metadata {
            source: err,
            path: git_dir.into(),
        })?
        .is_file()
    {
        let private_git_dir = crate::path::from_gitdir_file(git_dir)?;
        let common_dir = private_git_dir.join("commondir");
        match crate::path::from_plain_file(&common_dir) {
            Some(Err(err)) => {
                return Err(crate::is_git::Error::MissingCommonDir {
                    missing: common_dir,
                    source: err,
                })
            }
            Some(Ok(common_dir)) => {
                let common_dir = private_git_dir.join(common_dir);
                (
                    Cow::Owned(private_git_dir),
                    Cow::Owned(common_dir),
                    Kind::LinkedWorkTreeDir,
                )
            }
            None => (
                Cow::Owned(private_git_dir.clone()),
                Cow::Owned(private_git_dir),
                Kind::Submodule,
            ),
        }
    } else {
        let common_dir = git_dir.join("commondir");
        let worktree_and_common_dir = crate::path::from_plain_file(common_dir)
            .and_then(Result::ok)
            .and_then(|cd| {
                crate::path::from_plain_file(git_dir.join("gitdir"))
                    .and_then(Result::ok)
                    .map(|worktree_gitfile| (crate::path::without_dot_git_dir(worktree_gitfile), cd))
            });
        match worktree_and_common_dir {
            Some((work_dir, common_dir)) => {
                let common_dir = git_dir.join(common_dir);
                (
                    Cow::Borrowed(git_dir),
                    Cow::Owned(common_dir),
                    Kind::WorkTreeGitDir { work_dir },
                )
            }
            None => (Cow::Borrowed(git_dir), Cow::Borrowed(git_dir), Kind::MaybeRepo),
        }
    };

    {
        // We expect to be able to parse any ref-hash, so we shouldn't have to know the repos hash here.
        // With ref-table, the has is probably stored as part of the ref-db itself, so we can handle it from there.
        // In other words, it's important not to fail on detached heads here because we guessed the hash kind wrongly.
        let object_hash_should_not_matter_here = gix_hash::Kind::Sha1;
        let refs = gix_ref::file::Store::at(
            dot_git.as_ref(),
            gix_ref::store::WriteReflog::Normal,
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
    Ok(match kind {
        Kind::LinkedWorkTreeDir => crate::repository::Kind::WorkTree {
            linked_git_dir: Some(dot_git.into_owned()),
        },
        Kind::WorkTreeGitDir { work_dir } => crate::repository::Kind::WorkTreeGitDir { work_dir },
        Kind::Submodule => crate::repository::Kind::Submodule {
            git_dir: dot_git.into_owned(),
        },
        Kind::MaybeRepo => {
            if bare(git_dir) {
                crate::repository::Kind::Bare
            } else if submodule_git_dir(git_dir) {
                crate::repository::Kind::SubmoduleGitDir
            } else {
                crate::repository::Kind::WorkTree { linked_git_dir: None }
            }
        }
    })
}
