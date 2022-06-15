use std::{borrow::Cow, env, path::PathBuf};

use bstr::{ByteSlice, ByteVec};

/// The error returned by [git_discover::discover()][function::discover()].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Failed to access a directory, or path is not a directory: '{}'", .path.display())]
    InaccessibleDirectory { path: PathBuf },
    #[error("Could find a git repository in '{}' or in any of its parents", .path.display())]
    NoGitRepository { path: PathBuf },
    #[error("Could find a git repository in '{}' or in any of its parents within ceiling height of {}", .path.display(), .ceiling_height)]
    NoGitRepositoryWithinCeiling { path: PathBuf, ceiling_height: usize },
    #[error("Could find a git repository in '{}' or in any of its parents within device limits below '{}'", .path.display(), .limit.display())]
    NoGitRepositoryWithinFs { path: PathBuf, limit: PathBuf },
    #[error("None of the passed ceiling directories prefixed the git-dir candidate, making them ineffective.")]
    NoMatchingCeilingDir,
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
    /// When discovering a repository, ignore any repositories that are located in these directories or any of their parents.
    pub ceiling_dirs: Vec<PathBuf>,
    /// If true, and `ceiling_dirs` is not empty, we expect at least one ceiling directory to match or else there will be an error.
    pub match_ceiling_dir_or_error: bool,
    /// if `true` avoid crossing filesystem boundaries.
    /// Only supported on Unix-like systems.
    // TODO: test on Linux
    // TODO: Handle WASI once https://github.com/rust-lang/rust/issues/71213 is resolved
    pub cross_fs: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            required_trust: git_sec::Trust::Reduced,
            ceiling_dirs: vec![],
            match_ceiling_dir_or_error: true,
            cross_fs: false,
        }
    }
}

impl Options {
    /// Loads discovery options overrides from the environment.
    ///
    /// The environment variables are:
    /// - `GIT_CEILING_DIRECTORIES` for `ceiling_dirs`
    ///
    /// Note that `GIT_DISCOVERY_ACROSS_FILESYSTEM` for `cross_fs` is **not** read,
    /// as it requires parsing of `git-config` style boolean values.
    // TODO: test
    pub fn apply_environment(mut self) -> Self {
        let name = "GIT_CEILING_DIRECTORIES";
        if let Some(ceiling_dirs) = env::var_os(name).and_then(|c| Vec::from_os_string(c).ok()) {
            self.ceiling_dirs = parse_ceiling_dirs(&ceiling_dirs);
        }
        self
    }
}

/// Parse a byte-string of `:`-separated paths into `Vec<PathBuf>`.
/// Non-absolute paths are discarded.
/// To match git, all paths are normalized, until an empty path is encountered.
fn parse_ceiling_dirs(ceiling_dirs: &[u8]) -> Vec<PathBuf> {
    let mut should_normalize = true;
    let mut result = Vec::new();
    for ceiling_dir in ceiling_dirs.split_str(":") {
        if ceiling_dir.is_empty() {
            should_normalize = false;
            continue;
        }

        // Paths that are invalid unicode can't be handled
        let mut dir = match ceiling_dir.to_path() {
            Ok(dir) => Cow::Borrowed(dir),
            Err(_) => continue,
        };

        // Only absolute paths are allowed
        if dir.is_relative() {
            continue;
        }

        if should_normalize {
            if let Ok(normalized) = git_path::realpath(&dir, "") {
                dir = Cow::Owned(normalized);
            }
        }
        result.push(dir.into_owned());
    }
    result
}

#[cfg(test)]
mod parse_ceiling_dirs {

    #[test]
    #[cfg(unix)]
    fn from_environment_format() -> std::io::Result<()> {
        use super::*;
        use std::{fs, os::unix::fs::symlink};

        // Setup filesystem
        let dir = tempfile::tempdir().expect("success creating temp dir");
        let direct_path = dir.path().join("direct");
        let symlink_path = dir.path().join("symlink");
        fs::create_dir(&direct_path)?;
        symlink(&direct_path, &symlink_path)?;

        // Parse & build ceiling dirs string
        let symlink_str = symlink_path.to_str().expect("symlink path is valid utf8");
        let ceiling_dir_string = format!("{}:relative::{}", symlink_str, symlink_str);
        let ceiling_dirs = parse_ceiling_dirs(ceiling_dir_string.as_bytes());

        assert_eq!(ceiling_dirs.len(), 2, "Relative path is discarded");
        assert_eq!(
            ceiling_dirs[0],
            symlink_path.canonicalize().expect("symlink path exists"),
            "Symlinks are resolved"
        );
        assert_eq!(
            ceiling_dirs[1], symlink_path,
            "Symlink are not resolved after empty item"
        );

        dir.close()
    }
}

pub(crate) mod function {
    #[cfg(unix)]
    use std::fs;
    use std::path::{Path, PathBuf};

    use git_sec::Trust;

    use super::{Error, Options};
    use crate::is_git;

    /// Find the location of the git repository directly in `directory` or in any of its parent directories and provide
    /// an associated Trust level by looking at the git directory's ownership, and control discovery using `options`.
    ///
    /// Fail if no valid-looking git repository could be found.
    // TODO: tests for trust-based discovery
    #[cfg_attr(not(unix), allow(unused_variables))]
    pub fn discover_opts(
        directory: impl AsRef<Path>,
        Options {
            required_trust,
            ceiling_dirs,
            match_ceiling_dir_or_error,
            cross_fs,
        }: Options,
    ) -> Result<(crate::repository::Path, Trust), Error> {
        // Absolutize the path so that `Path::parent()` _actually_ gives
        // us the parent directory. (`Path::parent` just strips off the last
        // path component, which means it will not do what you expect when
        // working with paths paths that contain '..'.)
        let cwd = std::env::current_dir().ok();
        let dir = git_path::absolutize(directory.as_ref(), cwd.as_deref());
        let dir_metadata = dir.metadata().map_err(|_| Error::InaccessibleDirectory {
            path: dir.to_path_buf(),
        })?;

        if !dir_metadata.is_dir() {
            return Err(Error::InaccessibleDirectory { path: dir.into_owned() });
        }
        let mut dir_made_absolute = !directory.as_ref().is_absolute()
            && cwd.as_deref().map_or(false, |cwd| {
                cwd.strip_prefix(dir.as_ref())
                    .or_else(|_| dir.as_ref().strip_prefix(cwd))
                    .is_ok()
            });

        let filter_by_trust = |x: &Path| -> Result<Option<Trust>, Error> {
            let trust = Trust::from_path_ownership(x).map_err(|err| Error::CheckTrust { path: x.into(), err })?;
            Ok((trust >= required_trust).then(|| (trust)))
        };

        let max_height = if !ceiling_dirs.is_empty() {
            let max_height = find_ceiling_height(&dir, &ceiling_dirs, cwd.as_deref());
            if max_height.is_none() && match_ceiling_dir_or_error {
                return Err(Error::NoMatchingCeilingDir);
            }
            max_height
        } else {
            None
        };

        #[cfg(unix)]
        let initial_device = device_id(&dir_metadata);

        let mut cursor = dir.clone().into_owned();
        let mut current_height = 0;
        'outer: loop {
            if max_height.map_or(false, |x| current_height > x) {
                return Err(Error::NoGitRepositoryWithinCeiling {
                    path: dir.into_owned(),
                    ceiling_height: current_height,
                });
            }
            current_height += 1;

            #[cfg(unix)]
            if current_height != 0 && !cross_fs {
                let metadata = if cursor.as_os_str().is_empty() {
                    Path::new(".")
                } else {
                    cursor.as_ref()
                }
                .metadata()
                .map_err(|_| Error::InaccessibleDirectory { path: cursor.clone() })?;

                if device_id(&metadata) != initial_device {
                    return Err(Error::NoGitRepositoryWithinFs {
                        path: dir.into_owned(),
                        limit: cursor.clone(),
                    });
                }
            }

            for append_dot_git in &[false, true] {
                if *append_dot_git {
                    cursor.push(".git");
                }
                if let Ok(kind) = is_git(&cursor) {
                    match filter_by_trust(&cursor)? {
                        Some(trust) => {
                            // TODO: test this more, it definitely doesn't find the shortest path to a directory
                            let path = if dir_made_absolute {
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
                if dir_made_absolute
                    || matches!(
                        cursor.components().next(),
                        Some(std::path::Component::RootDir) | Some(std::path::Component::Prefix(_))
                    )
                {
                    break Err(Error::NoGitRepository { path: dir.into_owned() });
                } else {
                    dir_made_absolute = true;
                    cursor = if cursor.as_os_str().is_empty() {
                        cwd.clone()
                    } else {
                        cursor.canonicalize().ok()
                    }
                    .ok_or(Error::InaccessibleDirectory { path: cursor })?;
                }
            }
        }
    }

    fn shorten_path_with_cwd(cursor: PathBuf, cwd: Option<PathBuf>) -> PathBuf {
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

        if let Some(cwd) = cwd {
            debug_assert_eq!(cursor.file_name().and_then(|f| f.to_str()), Some(".git"));
            let parent = cursor.parent().expect(".git appended");
            cwd.strip_prefix(parent)
                .ok()
                .and_then(|path_relative_to_cwd| {
                    let relative_path_components = path_relative_to_cwd.components().count();
                    let current_component_len = cursor.components().map(comp_len).sum::<usize>();
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

    /// Find the number of components parenting the `search_dir` before the first directory in `ceiling_dirs`.
    /// `search_dir` needs to be absolutized, and we absolutize every ceiling as well.
    fn find_ceiling_height(search_dir: &Path, ceiling_dirs: &[PathBuf], cwd: Option<&Path>) -> Option<usize> {
        ceiling_dirs
            .iter()
            .filter_map(|ceiling_dir| {
                let mut ceiling_dir = git_path::absolutize(ceiling_dir, cwd);
                match (search_dir.is_absolute(), ceiling_dir.is_absolute()) {
                    (true, false) => ceiling_dir = cwd?.join(ceiling_dir.as_ref()).into(),
                    (false, true) => {
                        let stripped = ceiling_dir.as_ref().strip_prefix(cwd?).ok()?.to_owned();
                        ceiling_dir = stripped.into();
                    }
                    (false, false) | (true, true) => {}
                };
                search_dir
                    .strip_prefix(ceiling_dir.as_ref())
                    .ok()
                    .map(|path_relative_to_ceiling| path_relative_to_ceiling.components().count())
            })
            .min()
    }

    #[cfg(target_os = "linux")]
    /// Returns the device ID of the directory.
    fn device_id(m: &fs::Metadata) -> u64 {
        use std::os::linux::fs::MetadataExt;
        m.st_dev()
    }

    #[cfg(all(unix, not(target_os = "linux")))]
    /// Returns the device ID of the directory.
    fn device_id(m: &fs::Metadata) -> u64 {
        use std::os::unix::fs::MetadataExt;
        m.dev()
    }

    /// Find the location of the git repository directly in `directory` or in any of its parent directories, and provide
    /// the trust level derived from Path ownership.
    ///
    /// Fail if no valid-looking git repository could be found.
    pub fn discover(directory: impl AsRef<Path>) -> Result<(crate::repository::Path, Trust), Error> {
        discover_opts(directory, Default::default())
    }
}
