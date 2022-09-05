use crate::bstr::{BString, ByteVec};
use std::ffi::{OsStr, OsString};

/// Equivalent to `std::env::args_os()`, but with precomposed unicode on MacOS and other apple platforms.
#[cfg(not(target_vendor = "apple"))]
pub fn args_os() -> impl Iterator<Item = OsString> {
    std::env::args_os()
}

/// Equivalent to `std::env::args_os()`, but with precomposed unicode on MacOS and other apple platforms.
///
/// Note that this ignores `core.precomposeUnicode` as git-config isn't available yet. It's default enabled in modern git though.
#[cfg(target_vendor = "apple")]
pub fn args_os() -> impl Iterator<Item = OsString> {
    use unicode_normalization::UnicodeNormalization;
    std::env::args_os().map(|arg| match arg.to_str() {
        Some(arg) => arg.nfc().collect::<String>().into(),
        None => arg,
    })
}

/// Convert the given `input` into a `BString`, useful as `#[clap(parse(try_from_os_str = git::env::os_str_to_bstring))]` function.
pub fn os_str_to_bstring(input: &OsStr) -> Result<BString, String> {
    Vec::from_os_string(input.into())
        .map(Into::into)
        .map_err(|_| input.to_string_lossy().into_owned())
}

/// Environment information involving the `git` program itself.
pub mod git {
    use crate::bstr::{BStr, ByteSlice};

    fn first_file_from_config_with_origin(source: &BStr) -> Option<&BStr> {
        let file = source.strip_prefix(b"file:")?;
        let end_pos = file.find_byte(b'\t')?;
        file[..end_pos].as_bstr().into()
    }

    #[cfg(test)]
    mod tests {
        use crate::env::git;

        #[test]
        fn first_file_from_config_with_origin() {
            let macos = "file:/Applications/Xcode.app/Contents/Developer/usr/share/git-core/gitconfig	credential.helper=osxkeychain\nfile:/Users/byron/.gitconfig	push.default=simple\n";
            let win_msys =
                "file:C:/git-sdk-64/etc/gitconfig	core.symlinks=false\r\nfile:C:/git-sdk-64/etc/gitconfig	core.autocrlf=true";
            let win_cmd = "file:C:/Program Files/Git/etc/gitconfig	diff.astextplain.textconv=astextplain\r\nfile:C:/Program Files/Git/etc/gitconfig	filter.lfs.clean=git-lfs clean -- %f\r\n";
            let linux = "file:/home/parallels/.gitconfig	core.excludesfile=~/.gitignore\n";
            let bogus = "something unexpected";
            let empty = "";

            for (source, expected) in [
                (
                    macos,
                    Some("/Applications/Xcode.app/Contents/Developer/usr/share/git-core/gitconfig"),
                ),
                (win_msys, Some("C:/git-sdk-64/etc/gitconfig")),
                (win_cmd, Some("C:/Program Files/Git/etc/gitconfig")),
                (linux, Some("/home/parallels/.gitconfig")),
                (bogus, None),
                (empty, None),
            ] {
                assert_eq!(
                    git::first_file_from_config_with_origin(source.into()),
                    expected.map(Into::into)
                );
            }
        }
    }
}
