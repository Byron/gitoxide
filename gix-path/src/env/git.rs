use std::{
    path::Path,
    process::{Command, Stdio},
};

use bstr::{BStr, BString, ByteSlice};

/// Returns the file that contains git configuration coming with the installation of the `git` file in the current `PATH`, or `None`
/// if no `git` executable was found or there were other errors during execution.
pub(crate) fn install_config_path() -> Option<&'static BStr> {
    let _span = gix_trace::detail!("gix_path::git::install_config_path()");
    static PATH: once_cell::sync::Lazy<Option<BString>> = once_cell::sync::Lazy::new(|| {
        // Shortcut: in Msys shells this variable is set which allows to deduce the installation directory
        // so we can save the `git` invocation.
        #[cfg(windows)]
        if let Some(mut exec_path) = std::env::var_os("EXEPATH").map(std::path::PathBuf::from) {
            exec_path.push("etc");
            exec_path.push("gitconfig");
            return crate::os_string_into_bstring(exec_path.into()).ok();
        }
        let mut cmd = Command::new(if cfg!(windows) { "git.exe" } else { "git" });
        cmd.args(["config", "-l", "--show-origin"])
            .stdin(Stdio::null())
            .stderr(Stdio::null());
        first_file_from_config_with_origin(cmd.output().ok()?.stdout.as_slice().into()).map(ToOwned::to_owned)
    });
    PATH.as_ref().map(AsRef::as_ref)
}

fn first_file_from_config_with_origin(source: &BStr) -> Option<&BStr> {
    let file = source.strip_prefix(b"file:")?;
    let end_pos = file.find_byte(b'\t')?;
    file[..end_pos].trim_with(|c| c == '"').as_bstr().into()
}

/// Given `config_path` as obtained from `install_config_path()`, return the path of the git installation base.
pub(crate) fn config_to_base_path(config_path: &Path) -> &Path {
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
