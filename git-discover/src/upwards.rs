use std::path::PathBuf;

/// The error returned by [git_discover::discover()][function::discover()].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Failed to access a directory, or path is not a directory: '{}'", .path.display())]
    InaccessibleDirectory { path: PathBuf },
    #[error("Could find a git repository in '{}' or in any of its parents", .path.display())]
    NoGitRepository { path: PathBuf },
    #[error("Could find a trusted git repository in '{}' or in any of its parents, candidate at '{}' discarded", .path.display(), .candidate.display())]
    NoTrustedGitRepository {
        path: PathBuf,
        candidate: PathBuf,
        required: git_sec::Trust,
    },
    #[error("Could not determine trust level for path '{}'.", .path.display())]
    CheckTrust {
        path: PathBuf,
        #[source]
        err: std::io::Error,
    },
}

/// Options to help guide the [discovery][function::discover()] of repositories, along with their options
/// when instantiated.
pub struct Options {
    /// When discovering a repository, assure it has at least this trust level or ignore it otherwise.
    ///
    /// This defaults to [`Reduced`][git_sec::Trust::Reduced] as our default settings are geared towards avoiding abuse.
    /// Set it to `Full` to only see repositories that [are owned by the current user][git_sec::Trust::from_path_ownership()].
    pub required_trust: git_sec::Trust,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            required_trust: git_sec::Trust::Reduced,
        }
    }
}

pub(crate) mod function {
    use std::path::{Path, PathBuf};

    use crate::is_git;
    use git_sec::Trust;

    use super::{Error, Options};

    /// Find the location of the git repository directly in `directory` or in any of its parent directories and provide
    /// an associated Trust level by looking at the git directory's ownership, and control discovery using `options`.
    ///
    /// Fail if no valid-looking git repository could be found.
    // TODO: tests for trust-based discovery
    pub fn discover_opts(
        directory: impl AsRef<Path>,
        Options { required_trust }: Options,
    ) -> Result<(crate::repository::Path, git_sec::Trust), Error> {
        // Canonicalize the path so that `git_discover::parent` _actually_ gives
        // us the parent directory. (`git_discover::parent` just strips off the last
        // path component, which means it will not do what you expect when
        // working with paths paths that contain '..'.)
        let cwd = std::env::current_dir().ok();
        let dir = git_path::absolutize(directory.as_ref(), cwd.as_deref());
        if !dir.is_dir() {
            return Err(Error::InaccessibleDirectory { path: dir.into_owned() });
        }
        let mut is_canonicalized = false;

        let filter_by_trust = |x: &std::path::Path| -> Result<Option<git_sec::Trust>, Error> {
            let trust =
                git_sec::Trust::from_path_ownership(x).map_err(|err| Error::CheckTrust { path: x.into(), err })?;
            Ok((trust >= required_trust).then(|| (trust)))
        };

        let mut cursor = dir.clone().into_owned();
        'outer: loop {
            for append_dot_git in &[false, true] {
                if *append_dot_git {
                    cursor.push(".git");
                }
                if let Ok(kind) = is_git(&cursor) {
                    match filter_by_trust(&cursor)? {
                        Some(trust) => {
                            // TODO: test this more, it definitely doesn't find the shortest path to a directory
                            let path = if is_canonicalized {
                                shorten_path_with_cwd(cursor, cwd)
                            } else {
                                cursor
                            };
                            break 'outer Ok((crate::repository::Path::from_dot_git_dir(path, kind), trust));
                        }
                        None => {
                            break 'outer Err(Error::NoTrustedGitRepository {
                                path: dir.into_owned(),
                                candidate: cursor,
                                required: required_trust,
                            })
                        }
                    }
                }
                if *append_dot_git {
                    cursor.pop();
                }
            }
            if !cursor.pop() {
                if is_canonicalized
                    || matches!(
                        cursor.components().next(),
                        Some(std::path::Component::RootDir) | Some(std::path::Component::Prefix(_))
                    )
                {
                    break Err(Error::NoGitRepository { path: dir.into_owned() });
                } else {
                    is_canonicalized = true;
                    cursor = if cursor.as_os_str().is_empty() {
                        cwd.clone()
                    } else {
                        cursor.canonicalize().ok()
                    }
                    .ok_or_else(|| Error::InaccessibleDirectory { path: cursor })?;
                }
            }
        }
    }

    fn shorten_path_with_cwd(cursor: PathBuf, cwd: Option<PathBuf>) -> PathBuf {
        if let Some(cwd) = cwd {
            debug_assert_eq!(cursor.file_name().and_then(|f| f.to_str()), Some(".git"));
            let parent = cursor.parent().expect(".git appended");
            cwd.strip_prefix(parent)
                .ok()
                .and_then(|path_relative_to_cwd| {
                    let relative_path_components = path_relative_to_cwd.components().count();
                    let current_component_len = cursor.components().map(|c| comp_len(c)).sum::<usize>();
                    (relative_path_components * "..".len() < current_component_len).then(|| {
                        std::iter::repeat("..")
                            .take(relative_path_components)
                            .chain(Some(".git"))
                            .collect()
                    })
                })
                .unwrap_or(cursor)
        } else {
            cursor
        }
    }

    fn comp_len(c: std::path::Component<'_>) -> usize {
        use std::path::Component::*;
        match c {
            Prefix(p) => p.as_os_str().len(),
            CurDir => 1,
            ParentDir => 2,
            Normal(p) => p.len(),
            RootDir => 1,
        }
    }

    /// Find the location of the git repository directly in `directory` or in any of its parent directories, and provide
    /// the trust level derived from Path ownership.
    ///
    /// Fail if no valid-looking git repository could be found.
    pub fn discover(directory: impl AsRef<Path>) -> Result<(crate::repository::Path, Trust), Error> {
        discover_opts(directory, Default::default())
    }
}
