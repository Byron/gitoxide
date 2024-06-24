use std::path::PathBuf;
use std::{
    path::Path,
    process::{Command, Stdio},
};

use bstr::{BStr, BString, ByteSlice};

/// Other places to find Git in.
#[cfg(windows)]
pub(super) static ALTERNATIVE_LOCATIONS: &[&str] = &[
    "C:/Program Files/Git/mingw64/bin",
    "C:/Program Files (x86)/Git/mingw32/bin",
];
#[cfg(not(windows))]
pub(super) static ALTERNATIVE_LOCATIONS: &[&str] = &[];

#[cfg(windows)]
pub(super) static EXE_NAME: &str = "git.exe";
#[cfg(not(windows))]
pub(super) static EXE_NAME: &str = "git";

/// Invoke the git executable in PATH to obtain the origin configuration, which is cached and returned.
pub(super) static EXE_INFO: once_cell::sync::Lazy<Option<BString>> = once_cell::sync::Lazy::new(|| {
    let git_cmd = |executable: PathBuf| {
        let mut cmd = Command::new(executable);
        cmd.args(["config", "-l", "--show-origin"])
            .stdin(Stdio::null())
            .stderr(Stdio::null());
        cmd
    };
    let mut cmd = git_cmd(EXE_NAME.into());
    gix_trace::debug!(cmd = ?cmd, "invoking git for installation config path");
    let cmd_output = match cmd.output() {
        Ok(out) => out.stdout,
        #[cfg(windows)]
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            let executable = ALTERNATIVE_LOCATIONS.into_iter().find_map(|prefix| {
                let candidate = Path::new(prefix).join(EXE_NAME);
                candidate.is_file().then_some(candidate)
            })?;
            gix_trace::debug!(cmd = ?cmd, "invoking git for installation config path in alternate location");
            git_cmd(executable).output().ok()?.stdout
        }
        Err(_) => return None,
    };

    first_file_from_config_with_origin(cmd_output.as_slice().into()).map(ToOwned::to_owned)
});

/// Returns the file that contains git configuration coming with the installation of the `git` file in the current `PATH`, or `None`
/// if no `git` executable was found or there were other errors during execution.
pub(super) fn install_config_path() -> Option<&'static BStr> {
    let _span = gix_trace::detail!("gix_path::git::install_config_path()");
    static PATH: once_cell::sync::Lazy<Option<BString>> = once_cell::sync::Lazy::new(|| {
        // Shortcut: in Msys shells this variable is set which allows to deduce the installation directory,
        // so we can save the `git` invocation.
        #[cfg(windows)]
        if let Some(mut exec_path) = std::env::var_os("EXEPATH").map(std::path::PathBuf::from) {
            exec_path.push("etc");
            exec_path.push("gitconfig");
            return crate::os_string_into_bstring(exec_path.into()).ok();
        }
        EXE_INFO.clone()
    });
    PATH.as_ref().map(AsRef::as_ref)
}

fn first_file_from_config_with_origin(source: &BStr) -> Option<&BStr> {
    let file = source.strip_prefix(b"file:")?;
    let end_pos = file.find_byte(b'\t')?;
    file[..end_pos].trim_with(|c| c == '"').as_bstr().into()
}

/// Given `config_path` as obtained from `install_config_path()`, return the path of the git installation base.
pub(super) fn config_to_base_path(config_path: &Path) -> &Path {
    config_path
        .parent()
        .expect("config file paths always have a file name to pop")
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    #[test]
    fn config_to_base_path() {
        for (input, expected) in [
            (
                "/Applications/Xcode.app/Contents/Developer/usr/share/git-core/gitconfig",
                "/Applications/Xcode.app/Contents/Developer/usr/share/git-core",
            ),
            ("C:/git-sdk-64/etc/gitconfig", "C:/git-sdk-64/etc"),
            ("C:\\ProgramData/Git/config", "C:\\ProgramData/Git"),
            ("C:/Program Files/Git/etc/gitconfig", "C:/Program Files/Git/etc"),
        ] {
            assert_eq!(super::config_to_base_path(Path::new(input)), Path::new(expected));
        }
    }
    #[test]
    fn first_file_from_config_with_origin() {
        let macos = "file:/Applications/Xcode.app/Contents/Developer/usr/share/git-core/gitconfig	credential.helper=osxkeychain\nfile:/Users/byron/.gitconfig	push.default=simple\n";
        let win_msys =
            "file:C:/git-sdk-64/etc/gitconfig	core.symlinks=false\r\nfile:C:/git-sdk-64/etc/gitconfig	core.autocrlf=true";
        let win_cmd = "file:C:/Program Files/Git/etc/gitconfig	diff.astextplain.textconv=astextplain\r\nfile:C:/Program Files/Git/etc/gitconfig	filter.lfs.clean=gix-lfs clean -- %f\r\n";
        let win_msys_old = "file:\"C:\\ProgramData/Git/config\"	diff.astextplain.textconv=astextplain\r\nfile:\"C:\\ProgramData/Git/config\"	filter.lfs.clean=git-lfs clean -- %f\r\n";
        let linux = "file:/home/parallels/.gitconfig	core.excludesfile=~/.gitignore\n";
        let bogus = "something unexpected";
        let empty = "";

        for (source, expected) in [
            (
                macos,
                Some("/Applications/Xcode.app/Contents/Developer/usr/share/git-core/gitconfig"),
            ),
            (win_msys, Some("C:/git-sdk-64/etc/gitconfig")),
            (win_msys_old, Some("C:\\ProgramData/Git/config")),
            (win_cmd, Some("C:/Program Files/Git/etc/gitconfig")),
            (linux, Some("/home/parallels/.gitconfig")),
            (bogus, None),
            (empty, None),
        ] {
            assert_eq!(
                super::first_file_from_config_with_origin(source.into()),
                expected.map(Into::into)
            );
        }
    }
}
