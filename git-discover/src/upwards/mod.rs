mod types;
pub use types::{Error, Options};

mod util;

pub(crate) mod function {
    use std::{borrow::Cow, path::Path};

    use git_sec::Trust;

    use super::{Error, Options};
    #[cfg(unix)]
    use crate::upwards::util::device_id;
    use crate::{
        is_git,
        upwards::util::{find_ceiling_height, shorten_path_with_cwd},
        DOT_GIT_DIR,
    };

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
            current_dir,
        }: Options<'_>,
    ) -> Result<(crate::repository::Path, Trust), Error> {
        // Normalize the path so that `Path::parent()` _actually_ gives
        // us the parent directory. (`Path::parent` just strips off the last
        // path component, which means it will not do what you expect when
        // working with paths paths that contain '..'.)
        let cwd = current_dir
            .map(|cwd| Ok(Cow::Borrowed(cwd)))
            .unwrap_or_else(|| std::env::current_dir().map(Cow::Owned))?;
        let directory = directory.as_ref();
        let dir = git_path::normalize(directory, cwd.as_ref()).ok_or_else(|| Error::InvalidInput {
            directory: directory.into(),
        })?;
        let dir_metadata = dir.metadata().map_err(|_| Error::InaccessibleDirectory {
            path: dir.to_path_buf(),
        })?;

        if !dir_metadata.is_dir() {
            return Err(Error::InaccessibleDirectory { path: dir.into_owned() });
        }
        let mut dir_made_absolute = !directory.is_absolute()
            && cwd
                .as_ref()
                .strip_prefix(dir.as_ref())
                .or_else(|_| dir.as_ref().strip_prefix(cwd.as_ref()))
                .is_ok();

        let filter_by_trust = |x: &Path| -> Result<Option<Trust>, Error> {
            let trust = Trust::from_path_ownership(x).map_err(|err| Error::CheckTrust { path: x.into(), err })?;
            Ok((trust >= required_trust).then(|| (trust)))
        };

        let max_height = if !ceiling_dirs.is_empty() {
            let max_height = find_ceiling_height(&dir, &ceiling_dirs, cwd.as_ref());
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
                    cursor.push(DOT_GIT_DIR);
                }
                if let Ok(kind) = is_git(&cursor) {
                    match filter_by_trust(&cursor)? {
                        Some(trust) => {
                            // TODO: test this more, it definitely doesn't always find the shortest path to a directory
                            let path = if dir_made_absolute {
                                shorten_path_with_cwd(cursor, cwd.as_ref())
                            } else {
                                cursor
                            };
                            break 'outer Ok((
                                crate::repository::Path::from_dot_git_dir(path, kind, cwd).ok_or_else(|| {
                                    Error::InvalidInput {
                                        directory: directory.into(),
                                    }
                                })?,
                                trust,
                            ));
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
            if cursor.parent().map_or(false, |p| p.as_os_str().is_empty()) {
                cursor = cwd.to_path_buf();
                dir_made_absolute = true;
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
                    debug_assert!(!cursor.as_os_str().is_empty());
                    // TODO: realpath or normalize? No test runs into this.
                    cursor = git_path::normalize(&cursor, cwd.as_ref())
                        .ok_or_else(|| Error::InvalidInput {
                            directory: cursor.clone(),
                        })?
                        .into_owned();
                }
            }
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
