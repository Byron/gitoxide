use std::{
    ffi::OsString,
    path::{Path, PathBuf},
};

use bstr::{BString, ByteSlice};

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
/// On windows, the slowest part is the launch of the `git.exe` executable in the PATH, which only happens when launched
/// from outside of the `msys2` shell.
///
/// ### When `None` is returned
///
/// This happens only windows if the git binary can't be found at all for obtaining its executable path, or if the git binary
/// wasn't built with a well-known directory structure or environment.
pub fn system_prefix() -> Option<&'static Path> {
    if cfg!(windows) {
        static PREFIX: once_cell::sync::Lazy<Option<PathBuf>> = once_cell::sync::Lazy::new(|| {
            if let Some(root) = std::env::var_os("EXEPATH").map(PathBuf::from) {
                for candidate in ["mingw64", "mingw32"] {
                    let candidate = root.join(candidate);
                    if candidate.is_dir() {
                        return Some(candidate);
                    }
                }
            }

            let path = std::process::Command::new("git.exe")
                .arg("--exec-path")
                .stderr(std::process::Stdio::null())
                .output()
                .ok()?
                .stdout;
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

#[cfg(not(target_family = "wasm"))]
pub use home::home_dir;

/// Returns the contents of an environment variable of `name` with some special handling
/// for certain environment variables (like `HOME`) for platform compatibility.
pub fn var(name: &str) -> Option<OsString> {
    if name == "HOME" {
        home_dir().map(PathBuf::into_os_string)
    } else {
        std::env::var_os(name)
    }
}
