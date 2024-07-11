use std::ffi::OsString;
use std::path::{Path, PathBuf};

use bstr::{BString, ByteSlice};
use once_cell::sync::Lazy;

use crate::env::git::EXE_NAME;

mod git;

/// Return the location at which installation specific git configuration file can be found, or `None`
/// if the binary could not be executed or its results could not be parsed.
///
/// ### Performance
///
/// This invokes the git binary which is slow on windows.
pub fn installation_config() -> Option<&'static Path> {
    git::install_config_path().and_then(|p| crate::try_from_byte_slice(p).ok())
}

/// Return the location at which git installation specific configuration files are located, or `None` if the binary
/// could not be executed or its results could not be parsed.
///
/// ### Performance
///
/// This invokes the git binary which is slow on windows.
pub fn installation_config_prefix() -> Option<&'static Path> {
    installation_config().map(git::config_to_base_path)
}

/// Return the name of the Git executable to invoke it.
/// If it's in the `PATH`, it will always be a short name.
///
/// Note that on Windows, we will find the executable in the `PATH` if it exists there, or search it
/// in alternative locations which when found yields the full path to it.
pub fn exe_invocation() -> &'static Path {
    if cfg!(windows) {
        /// The path to the Git executable as located in the `PATH` or in other locations that it's known to be installed to.
        /// It's `None` if environment variables couldn't be read or if no executable could be found.
        static EXECUTABLE_PATH: Lazy<Option<PathBuf>> = Lazy::new(|| {
            std::env::split_paths(&std::env::var_os("PATH")?)
                .chain(git::ALTERNATIVE_LOCATIONS.iter().map(Into::into))
                .find_map(|prefix| {
                    let full_path = prefix.join(EXE_NAME);
                    full_path.is_file().then_some(full_path)
                })
                .map(|exe_path| {
                    let is_in_alternate_location = git::ALTERNATIVE_LOCATIONS
                        .iter()
                        .any(|prefix| exe_path.strip_prefix(prefix).is_ok());
                    if is_in_alternate_location {
                        exe_path
                    } else {
                        EXE_NAME.into()
                    }
                })
        });
        EXECUTABLE_PATH.as_deref().unwrap_or(Path::new(git::EXE_NAME))
    } else {
        Path::new("git")
    }
}

/// Returns the fully qualified path in the *xdg-home* directory (or equivalent in the home dir) to `file`,
/// accessing `env_var(<name>)` to learn where these bases are.
///
/// Note that the `HOME` directory should ultimately come from [`home_dir()`] as it handles windows correctly.
/// The same can be achieved by using [`var()`] as `env_var`.
pub fn xdg_config(file: &str, env_var: &mut dyn FnMut(&str) -> Option<OsString>) -> Option<PathBuf> {
    env_var("XDG_CONFIG_HOME")
        .map(|home| {
            let mut p = PathBuf::from(home);
            p.push("git");
            p.push(file);
            p
        })
        .or_else(|| {
            env_var("HOME").map(|home| {
                let mut p = PathBuf::from(home);
                p.push(".config");
                p.push("git");
                p.push(file);
                p
            })
        })
}

/// Returns the platform dependent system prefix or `None` if it cannot be found (right now only on windows).
///
/// ### Performance
///
/// On windows, the slowest part is the launch of the Git executable in the PATH, which only happens when launched
/// from outside of the `msys2` shell.
///
/// ### When `None` is returned
///
/// This happens only windows if the git binary can't be found at all for obtaining its executable path, or if the git binary
/// wasn't built with a well-known directory structure or environment.
pub fn system_prefix() -> Option<&'static Path> {
    if cfg!(windows) {
        static PREFIX: Lazy<Option<PathBuf>> = Lazy::new(|| {
            if let Some(root) = std::env::var_os("EXEPATH").map(PathBuf::from) {
                for candidate in ["mingw64", "mingw32"] {
                    let candidate = root.join(candidate);
                    if candidate.is_dir() {
                        return Some(candidate);
                    }
                }
            }

            let mut cmd = std::process::Command::new(exe_invocation());
            cmd.arg("--exec-path").stderr(std::process::Stdio::null());
            gix_trace::debug!(cmd = ?cmd, "invoking git to get system prefix/exec path");
            let path = cmd.output().ok()?.stdout;
            let path = BString::new(path)
                .trim_with(|b| b.is_ascii_whitespace())
                .to_path()
                .ok()?
                .to_owned();

            let one_past_prefix = path.components().enumerate().find_map(|(idx, c)| {
                matches!(c,std::path::Component::Normal(name) if name.to_str() == Some("libexec")).then_some(idx)
            })?;
            Some(path.components().take(one_past_prefix.checked_sub(1)?).collect())
        });
        PREFIX.as_deref()
    } else {
        Path::new("/").into()
    }
}

/// Returns `$HOME` or `None` if it cannot be found.
#[cfg(target_family = "wasm")]
pub fn home_dir() -> Option<PathBuf> {
    std::env::var("HOME").map(PathBuf::from).ok()
}

/// Tries to obtain the home directory from `HOME` on all platforms, but falls back to [`home::home_dir()`] for
/// more complex ways of obtaining a home directory, particularly useful on Windows.
///
/// The reason `HOME` is tried first is to allow Windows users to have a custom location for their linux-style
/// home, as otherwise they would have to accumulate dot files in a directory these are inconvenient and perceived
/// as clutter.
#[cfg(not(target_family = "wasm"))]
pub fn home_dir() -> Option<PathBuf> {
    std::env::var_os("HOME").map(Into::into).or_else(home::home_dir)
}

/// Returns the contents of an environment variable of `name` with some special handling
/// for certain environment variables (like `HOME`) for platform compatibility.
pub fn var(name: &str) -> Option<OsString> {
    if name == "HOME" {
        home_dir().map(PathBuf::into_os_string)
    } else {
        std::env::var_os(name)
    }
}
