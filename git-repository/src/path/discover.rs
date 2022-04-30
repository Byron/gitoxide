use std::path::PathBuf;

/// The error returned by [path::discover()][function::discover()].
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
    use super::{Error, Options};
    use git_sec::Trust;
    use std::path::PathBuf;
    use std::{
        borrow::Cow,
        path::{Component, Path},
    };

    use crate::path;

    /// Find the location of the git repository directly in `directory` or in any of its parent directories and provide
    /// an associated Trust level by looking at the git directory's ownership, and control discovery using `options`.
    ///
    /// Fail if no valid-looking git repository could be found.
    // TODO: tests for trust-based discovery
    pub fn discover_opts(
        directory: impl AsRef<Path>,
        Options { required_trust }: Options,
    ) -> Result<(crate::Path, git_sec::Trust), Error> {
        // Canonicalize the path so that `Path::parent` _actually_ gives
        // us the parent directory. (`Path::parent` just strips off the last
        // path component, which means it will not do what you expect when
        // working with paths paths that contain '..'.)
        let directory = directory.as_ref();
        let dir = maybe_canonicalize(directory).map_err(|_| Error::InaccessibleDirectory { path: directory.into() })?;
        let is_canonicalized = dir.as_ref() != directory;
        if !dir.is_dir() {
            return Err(Error::InaccessibleDirectory { path: dir.into_owned() });
        }

        let filter_by_trust = |x: &std::path::Path| -> Result<Option<git_sec::Trust>, Error> {
            let trust =
                git_sec::Trust::from_path_ownership(x).map_err(|err| Error::CheckTrust { path: x.into(), err })?;
            Ok((trust >= required_trust).then(|| (trust)))
        };

        let mut cursor = dir.clone();
        'outer: loop {
            for append_dot_git in &[false, true] {
                if *append_dot_git {
                    cursor = cursor.into_owned().into();
                    if let Cow::Owned(p) = &mut cursor {
                        p.push(".git");
                    }
                }
                if let Ok(kind) = path::is::git(&cursor) {
                    match filter_by_trust(&cursor)? {
                        Some(trust) => {
                            // TODO: test this more
                            let path = if is_canonicalized {
                                match std::env::current_dir() {
                                    Ok(cwd) => {
                                        dbg!(&cwd, &cursor);
                                        let short_path_components = cwd
                                            .strip_prefix(&cursor.parent().expect(".git appended"))
                                            .expect("cwd is always within canonicalized candidate")
                                            .components()
                                            .count();
                                        if short_path_components < cursor.components().count() {
                                            let mut p = PathBuf::new();
                                            p.extend(std::iter::repeat("..").take(short_path_components));
                                            p.push(".git");
                                            p
                                        } else {
                                            cursor.into_owned()
                                        }
                                    }
                                    Err(_) => cursor.into_owned(),
                                }
                            } else {
                                cursor.into_owned()
                            };
                            break 'outer Ok((crate::Path::from_dot_git_dir(path, kind), trust));
                        }
                        None => {
                            break 'outer Err(Error::NoTrustedGitRepository {
                                path: dir.into_owned(),
                                candidate: cursor.into_owned(),
                                required: required_trust,
                            })
                        }
                    }
                }
                if *append_dot_git {
                    if let Cow::Owned(p) = &mut cursor {
                        p.pop();
                    }
                }
            }
            match cursor.parent() {
                Some(parent) => cursor = parent.to_owned().into(),
                None => break Err(Error::NoGitRepository { path: dir.into_owned() }),
            }
        }
    }

    /// Find the location of the git repository directly in `directory` or in any of its parent directories, and provide
    /// the trust level derived from Path ownership.
    ///
    /// Fail if no valid-looking git repository could be found.
    pub fn discover(directory: impl AsRef<Path>) -> Result<(crate::Path, Trust), Error> {
        discover_opts(directory, Default::default())
    }

    fn maybe_canonicalize(path: &Path) -> std::io::Result<Cow<'_, Path>> {
        let ends_with_relative_component = path
            .components()
            .last()
            .map_or(true, |c| matches!(c, Component::CurDir | Component::ParentDir));
        if ends_with_relative_component {
            path.canonicalize().map(Into::into)
        } else {
            Ok(path.into())
        }
    }

    #[cfg(test)]
    mod maybe_canonicalize {
        use super::*;

        fn relative_component_count(path: impl AsRef<Path>) -> usize {
            path.as_ref()
                .components()
                .filter(|c| matches!(c, Component::CurDir | Component::ParentDir))
                .count()
        }

        #[test]
        fn empty_paths_are_invalid() {
            assert!(
                maybe_canonicalize(Path::new("")).is_err(),
                "empty paths are not equivalent to '.' but are non-existing"
            );
        }

        #[test]
        fn paths_starting_with_dot_but_end_with_normal_path_are_not_canonicalized() {
            assert_eq!(
                relative_component_count(maybe_canonicalize(&Path::new(".").join("hello")).unwrap()),
                1,
            );
        }

        #[test]
        fn paths_ending_with_non_normal_component_are_canonicalized() {
            assert_eq!(
                relative_component_count(maybe_canonicalize(&Path::new(".").join(".")).unwrap()),
                0,
            );
        }
    }
}
