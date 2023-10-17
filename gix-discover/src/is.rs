use std::{borrow::Cow, ffi::OsStr, path::Path};

use crate::{DOT_GIT_DIR, MODULES};

/// Returns true if the given `git_dir` seems to be a bare repository.
///
/// Please note that repositories without an index generally _look_ bare, even though they might also be uninitialized.
pub fn bare(git_dir_candidate: &Path) -> bool {
    !(git_dir_candidate.join("index").exists() || (git_dir_candidate.file_name() == Some(OsStr::new(DOT_GIT_DIR))))
}

/// Parse `<git_dir_candidate>/config` quickly to evaluate the value of the `bare` line, or return `true` if the file doesn't exist
/// similar to what`guess_repository_type` seems to be doing.
/// Return `None` if the `bare` line can't be found or the value of `bare` can't be determined.
fn bare_by_config(git_dir_candidate: &Path) -> std::io::Result<Option<bool>> {
    match std::fs::read(git_dir_candidate.join("config")) {
        Ok(buf) => Ok(config::parse_bare(&buf)),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(Some(true)),
        Err(err) => Err(err),
    }
}

// Copied and adapted from `gix-config-value::boolean`.
mod config {
    use bstr::{BStr, ByteSlice};

    /// Note that we intentionally turn repositories that have a worktree configuration into bare repos,
    /// as we don't actually parse the worktree from the config file and expect the caller to do the right
    /// think when seemingly seeing bare repository.
    /// The reason we do this is to not incorrectly pretend this is a worktree.
    pub(crate) fn parse_bare(buf: &[u8]) -> Option<bool> {
        let mut is_bare = None;
        let mut has_worktree_configuration = false;
        for line in buf.lines() {
            if is_bare.is_none() {
                if let Some(line) = line.trim().strip_prefix(b"bare") {
                    is_bare = match line.first() {
                        None => Some(true),
                        Some(c) if *c == b'=' => parse_bool(line.get(1..)?.trim_start().as_bstr()),
                        Some(c) if c.is_ascii_whitespace() => match line.split_once_str(b"=") {
                            Some((_left, right)) => parse_bool(right.trim_start().as_bstr()),
                            None => Some(true),
                        },
                        Some(_other_char_) => None,
                    };
                    continue;
                }
            }
            if line.trim().strip_prefix(b"worktree").is_some() {
                has_worktree_configuration = true;
                break;
            }
        }
        is_bare.map(|bare| bare || has_worktree_configuration)
    }

    fn parse_bool(value: &BStr) -> Option<bool> {
        Some(if parse_true(value) {
            true
        } else if parse_false(value) {
            false
        } else {
            use std::str::FromStr;
            if let Some(integer) = value.to_str().ok().and_then(|s| i64::from_str(s).ok()) {
                integer != 0
            } else {
                return None;
            }
        })
    }

    fn parse_true(value: &BStr) -> bool {
        value.eq_ignore_ascii_case(b"yes") || value.eq_ignore_ascii_case(b"on") || value.eq_ignore_ascii_case(b"true")
    }

    fn parse_false(value: &BStr) -> bool {
        value.eq_ignore_ascii_case(b"no")
            || value.eq_ignore_ascii_case(b"off")
            || value.eq_ignore_ascii_case(b"false")
            || value.is_empty()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn various() {
            for (input, expected) in [
                ("bare=true", Some(true)),
                ("bare=1", Some(true)),
                ("bare =1", Some(true)),
                ("bare= yes", Some(true)),
                ("bare=false", Some(false)),
                ("bare=0", Some(false)),
                ("bare=blah", None),
                ("bare=", Some(false)),
                ("bare=  \n", Some(false)),
                ("bare = true \n", Some(true)),
                ("\t bare = false \n", Some(false)),
                ("\n\tbare=true", Some(true)),
                ("\n\tbare=true\n\tfoo", Some(true)),
                ("\n\tbare ", Some(true)),
                ("\n\tbare", Some(true)),
                ("not found\nreally", None),
            ] {
                assert_eq!(parse_bare(input.as_bytes()), expected, "{input:?}");
            }
        }
    }
}

/// Returns true if `git_dir` is located within a `.git/modules` directory, indicating it's a submodule clone.
pub fn submodule_git_dir(git_dir: &Path) -> bool {
    let mut last_comp = None;
    git_dir.file_name() != Some(OsStr::new(DOT_GIT_DIR))
        && git_dir.components().rev().skip(1).any(|c| {
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
/// purely based on the presence of files. Note that the git-config ultimately decides what's bare.
///
/// Returns the `Kind` of git directory that was passed, possibly alongside the supporting private worktree git dir.
///
/// Note that `.git` files are followed to a valid git directory, which then requires…
///
///   * …a valid head
///   * …an objects directory
///   * …a refs directory
///
pub fn git(git_dir: &Path) -> Result<crate::repository::Kind, crate::is_git::Error> {
    let git_dir_metadata = git_dir.metadata().map_err(|err| crate::is_git::Error::Metadata {
        source: err,
        path: git_dir.into(),
    })?;
    git_with_metadata(git_dir, git_dir_metadata)
}

pub(crate) fn git_with_metadata(
    git_dir: &Path,
    git_dir_metadata: std::fs::Metadata,
) -> Result<crate::repository::Kind, crate::is_git::Error> {
    #[derive(Eq, PartialEq)]
    enum Kind {
        MaybeRepo,
        Submodule,
        LinkedWorkTreeDir,
        WorkTreeGitDir { work_dir: std::path::PathBuf },
    }

    let dot_git = if git_dir_metadata.is_file() {
        let private_git_dir = crate::path::from_gitdir_file(git_dir)?;
        Cow::Owned(private_git_dir)
    } else {
        Cow::Borrowed(git_dir)
    };

    {
        // Fast-path: avoid doing the complete search if HEAD is already not there.
        // TODO(reftable): use a ref-store to lookup HEAD if ref-tables should be supported, or detect ref-tables beforehand.
        if !dot_git.join("HEAD").exists() {
            return Err(crate::is_git::Error::MissingHead);
        }
        // We expect to be able to parse any ref-hash, so we shouldn't have to know the repos hash here.
        // With ref-table, the has is probably stored as part of the ref-db itself, so we can handle it from there.
        // In other words, it's important not to fail on detached heads here because we guessed the hash kind wrongly.
        let object_hash_should_not_matter_here = gix_hash::Kind::Sha1;
        let refs = gix_ref::file::Store::at(
            dot_git.as_ref().into(),
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

    let (common_dir, kind) = if git_dir_metadata.is_file() {
        let common_dir = dot_git.join("commondir");
        match crate::path::from_plain_file(&common_dir) {
            Some(Err(err)) => {
                return Err(crate::is_git::Error::MissingCommonDir {
                    missing: common_dir,
                    source: err,
                })
            }
            Some(Ok(common_dir)) => {
                let common_dir = dot_git.join(common_dir);
                (Cow::Owned(common_dir), Kind::LinkedWorkTreeDir)
            }
            None => (dot_git.clone(), Kind::Submodule),
        }
    } else {
        let common_dir = dot_git.join("commondir");
        let worktree_and_common_dir = crate::path::from_plain_file(&common_dir)
            .and_then(Result::ok)
            .and_then(|cd| {
                crate::path::from_plain_file(&dot_git.join("gitdir"))
                    .and_then(Result::ok)
                    .map(|worktree_gitfile| (crate::path::without_dot_git_dir(worktree_gitfile), cd))
            });
        match worktree_and_common_dir {
            Some((work_dir, common_dir)) => {
                let common_dir = dot_git.join(common_dir);
                (Cow::Owned(common_dir), Kind::WorkTreeGitDir { work_dir })
            }
            None => (dot_git.clone(), Kind::MaybeRepo),
        }
    };

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
            let conformed_git_dir = if git_dir == Path::new(".") {
                gix_path::realpath(git_dir)
                    .map(Cow::Owned)
                    .unwrap_or(Cow::Borrowed(git_dir))
            } else {
                Cow::Borrowed(git_dir)
            };
            if bare(conformed_git_dir.as_ref()) || conformed_git_dir.extension() == Some(OsStr::new("git")) {
                crate::repository::Kind::PossiblyBare
            } else if submodule_git_dir(conformed_git_dir.as_ref()) {
                crate::repository::Kind::SubmoduleGitDir
            } else if conformed_git_dir.file_name() == Some(OsStr::new(DOT_GIT_DIR))
                || !bare_by_config(conformed_git_dir.as_ref())
                    .map_err(|err| crate::is_git::Error::Metadata {
                        source: err,
                        path: conformed_git_dir.join("config"),
                    })?
                    .ok_or(crate::is_git::Error::Inconclusive)?
            {
                crate::repository::Kind::WorkTree { linked_git_dir: None }
            } else {
                crate::repository::Kind::PossiblyBare
            }
        }
    })
}
