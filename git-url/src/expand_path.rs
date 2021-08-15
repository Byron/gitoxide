//! Functions for expanding repository paths.
use std::path::{Path, PathBuf};

use bstr::{BStr, BString, ByteSlice};
use quick_error::quick_error;

/// Whether a repository is resolving for the current user, or the given one.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum ForUser {
    /// The currently logged in user.
    Current,
    /// The user with the given name.
    Name(BString),
}

impl From<ForUser> for Option<BString> {
    fn from(v: ForUser) -> Self {
        match v {
            ForUser::Name(user) => Some(user),
            ForUser::Current => None,
        }
    }
}

quick_error! {
    /// The error used by [`parse()`], [`with()`] and [`expand_path()`].
    #[derive(Debug)]
    pub enum Error {
        Utf8(err: bstr::Utf8Error) {
            display("UTF8 conversion on non-unix system failed")
            from()
            source(err)
        }
        MissingHome(user: Option<BString>) {
            display("Home directory could not be obtained for {}", match user {Some(user) => format!("user '{}'", user), None => "current user".into()})
        }
    }
}

fn path_segments(path: &BStr) -> Option<impl Iterator<Item = &[u8]>> {
    if path.starts_with(b"/") {
        Some(path[1..].split(|c| *c == b'/'))
    } else {
        None
    }
}

/// Parse user information from the given `path`, returning `(possible user information, adjusted input path)`.
///
/// Supported formats for user extraction are…
/// * `~/repopath` - the currently logged in user's home.
/// * `~user/repopath` - the repository in the given user's home.
pub fn parse(path: &BStr) -> Result<(Option<ForUser>, BString), Error> {
    Ok(path_segments(path)
        .and_then(|mut iter| {
            iter.next().map(|segment| {
                if segment.starts_with(b"~") {
                    let eu = if segment.len() == 1 {
                        Some(ForUser::Current)
                    } else {
                        Some(ForUser::Name(segment[1..].into()))
                    };
                    (
                        eu,
                        format!(
                            "/{}",
                            iter.map(|s| s.as_bstr().to_str_lossy()).collect::<Vec<_>>().join("/")
                        )
                        .into(),
                    )
                } else {
                    (None, path.into())
                }
            })
        })
        .unwrap_or_else(|| (None, path.into())))
}

/// Expand `path` for use in a shell and return the expanded path.
pub fn for_shell(path: BString) -> BString {
    use bstr::ByteVec;
    match parse(path.as_slice().as_bstr()) {
        Ok((user, mut path)) => match user {
            Some(ForUser::Current) => {
                path.insert(0, b'~');
                path
            }
            Some(ForUser::Name(mut user)) => {
                user.insert(0, b'~');
                user.append(path.as_vec_mut());
                user
            }
            None => path,
        },
        Err(_) => path,
    }
}

/// Expand `path` for the given `user`, which can be obtained by [`parse()`], resolving them with `home_for_user(&user)`.
///
/// For the common case consider using [`expand_path()]` instead.
pub fn with(
    user: Option<&ForUser>,
    path: &BStr,
    home_for_user: impl FnOnce(&ForUser) -> Option<PathBuf>,
) -> Result<PathBuf, Error> {
    fn make_relative(path: &Path) -> PathBuf {
        path.components().skip(1).collect()
    }
    let path = path.to_path()?;
    Ok(match user {
        Some(user) => home_for_user(user)
            .ok_or_else(|| Error::MissingHome(user.to_owned().into()))?
            .join(make_relative(path)),
        None => path.into(),
    })
}

/// Expand `path` for the given `user`, which can be obtained by [`parse()`], resolving the home directories
/// of `user` automatically.
///
/// If more precise control of the resolution mechanism is needed, then use the [`with()`] function.
pub fn expand_path(user: Option<&ForUser>, path: &BStr) -> Result<PathBuf, Error> {
    with(user, path, |user| match user {
        ForUser::Current => home::home_dir(),
        ForUser::Name(user) => {
            home::home_dir().and_then(|home| home.parent().map(|home_dirs| home_dirs.join(user.to_string())))
        }
    })
}
