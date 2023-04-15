use bstr::{BString, ByteSlice};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

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

/// Returns a platform independent home directory.
///
/// On unix this simply returns $HOME on windows this uses %HOMEDRIVE%\%HOMEPATH% or %USERPROFILE%
pub fn home_dir() -> Option<PathBuf> {
    if let Some(home) = std::env::var_os("HOME") {
        return Some(home.into());
    }

    // NOTE: technically we should also check HOMESHARE in case HOME is a UNC path
    //       but git doesn't do this either so probably best to wait for an upstream fix.
    #[cfg(windows)]
    {
        if let Some(homedrive) = std::env::var_os("HOMEDRIVE") {
            if let Some(home_path) = std::env::var_os("HOMEPATH") {
                let home = PathBuf::from(homedrive).join(home_path);
                if home.metadata().map_or(false, |home| home.is_dir()) {
                    return Some(home);
                }
            }
        }
        if let Some(userprofile) = std::env::var_os("USERPROFILE") {
            return Some(userprofile.into());
        }
    }
    None
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
