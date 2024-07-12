use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use bstr::{BStr, BString, ByteSlice};
use once_cell::sync::Lazy;

/// Other places to find Git in.
#[cfg(windows)]
pub(super) static ALTERNATIVE_LOCATIONS: Lazy<Vec<OsString>> = Lazy::new(|| {
    vec![
        "C:/Program Files/Git/mingw64/bin".into(),
        "C:/Program Files (x86)/Git/mingw32/bin".into(),
    ]
});
#[cfg(not(windows))]
pub(super) static ALTERNATIVE_LOCATIONS: Lazy<Vec<OsString>> = Lazy::new(|| vec![]);

#[cfg(windows)]
pub(super) static EXE_NAME: &str = "git.exe";
#[cfg(not(windows))]
pub(super) static EXE_NAME: &str = "git";

/// Invoke the git executable in PATH to obtain the origin configuration, which is cached and returned.
pub(super) static EXE_INFO: Lazy<Option<BString>> = Lazy::new(|| {
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
            let executable = ALTERNATIVE_LOCATIONS.iter().find_map(|prefix| {
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
    static PATH: Lazy<Option<BString>> = Lazy::new(|| {
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

    #[cfg(windows)]
    use {
        known_folders::{get_known_folder_path, KnownFolder},
        std::ffi::{OsStr, OsString},
        std::path::PathBuf,
        windows::core::Result as WindowsResult,
        windows::Win32::Foundation::BOOL,
        windows::Win32::System::Threading::{GetCurrentProcess, IsWow64Process},
        winreg::enums::{HKEY_LOCAL_MACHINE, KEY_QUERY_VALUE},
        winreg::RegKey,
    };

    #[cfg(windows)]
    trait Current: Sized {
        fn current() -> WindowsResult<Self>;
    }

    #[cfg(windows)]
    enum PlatformArchitecture {
        Is32on32,
        Is32on64,
        Is64on64,
    }

    #[cfg(windows)]
    impl Current for PlatformArchitecture {
        fn current() -> WindowsResult<Self> {
            // Ordinarily, we would check the target pointer width first to avoid doing extra work,
            // because if this is a 64-bit executable then the operating system is 64-bit. But this
            // is for the test suite, and doing it this way allows problems to be caught earlier if
            // a change made on a 64-bit development machine breaks the IsWow64Process() call.
            let mut wow64process = BOOL::default();
            unsafe { IsWow64Process(GetCurrentProcess(), &mut wow64process)? };

            let platform_architecture = if wow64process.as_bool() {
                Self::Is32on64
            } else if cfg!(target_pointer_width = "32") {
                Self::Is32on32
            } else {
                assert!(cfg!(target_pointer_width = "64"));
                Self::Is64on64
            };
            Ok(platform_architecture)
        }
    }

    #[cfg(windows)]
    fn ends_with_case_insensitive(text: &OsStr, suffix: &str) -> Option<bool> {
        Some(text.to_str()?.to_lowercase().ends_with(&suffix.to_lowercase()))
    }

    #[cfg(windows)]
    struct PathsByRole {
        /// The program files directory relative to what architecture this program was built for.
        pf_current: PathBuf,

        /// The x86 program files directory regardless of the architecture of the program.
        ///
        /// If Rust gains Windows targets like ARMv7 where this is unavailable, this could fail.
        pf_x86: PathBuf,

        /// The 64-bit program files directory if there is one.
        ///
        /// This is present on x64 and also ARM64 systems. On an ARM64 system, ARM64 and AMD64
        /// programs use the same program files directory while 32-bit x86 and ARM programs use two
        /// others. Only a 32-bit has no 64-bit program files directory.
        maybe_pf_64bit: Result<PathBuf, std::io::Error>,
        // While
        // KnownFolder::ProgramFilesX64 exists, it's unfortunately unavailable to 32-bit
        // processes; see
        // https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#remarks.
    }

    impl PathsByRole {
        /// Gets the three common kinds of global program files paths without environment variables.
        ///
        /// The idea here is to obtain this information, which the `alternative_locations()` unit
        /// test uses to learn the expected alternative locations, without duplicating *any* of the
        /// approach used for `ALTERNATIVE_LOCATIONS`, so it can be used to test that. The approach
        /// here is also be more reliable than using environment variables, but it is a bit more
        /// complex, and it requires either additional dependencies or the use of unsafe code.
        ///
        /// This obtains `pf_current` and `pf_x86` by the [known folders][known-folders] system,
        /// and `maybe_pf_64bit` from the registry since the corresponding known folder is not
        /// available to 32-bit processes, per the [KNOWNFOLDDERID][knownfolderid] documentation.
        ///
        /// If in the future the implementation of `ALTERNATIVE_LOCATIONS` uses these techniques,
        /// then this function can be changed to use environment variables and renamed accordingly.
        ///
        /// [known-folders]: https://learn.microsoft.com/en-us/windows/win32/shell/known-folders
        /// [knownfolderid]: https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#remarks
        fn obtain_envlessly() -> Self {
            let pf_current = get_known_folder_path(KnownFolder::ProgramFiles)
                .expect("The process architecture specific program files folder is always available.");

            let pf_x86 = get_known_folder_path(KnownFolder::ProgramFilesX86)
                .expect("The x86 program files folder will in practice always be available.");

            let maybe_pf_64bit = RegKey::predef(HKEY_LOCAL_MACHINE)
                .open_subkey_with_flags(r#"SOFTWARE\Microsoft\Windows\CurrentVersion"#, KEY_QUERY_VALUE)
                .expect("The `CurrentVersion` key exists and allows reading.")
                .get_value::<OsString, _>("ProgramW6432Dir")
                .map(PathBuf::from);

            Self {
                pf_current,
                pf_x86,
                maybe_pf_64bit,
            }
        }

        /// Checks that the paths we got for testing are reasonable.
        ///
        /// This checks that `obtain_envlessly()` returned paths that are likely to be correct and
        /// that satisfy the most important properties based on the current system and process.
        fn validate(self) -> Self {
            let PathsByRole {
                pf_current,
                pf_x86,
                maybe_pf_64bit,
            } = self;

            match PlatformArchitecture::current().expect("Process and system 'bitness' should be available.") {
                PlatformArchitecture::Is32on32 => {
                    assert_eq!(
                        pf_current.as_os_str(),
                        pf_x86.as_os_str(),
                        "Ours is identical to 32-bit path."
                    );
                    for arch_suffix in [" (x86)", " (Arm)"] {
                        let has_arch_suffix = ends_with_case_insensitive(pf_current.as_os_str(), arch_suffix)
                            .expect("Assume the test systems key dirs are valid Unicode.");
                        assert!(
                            !has_arch_suffix,
                            "32-bit program files on 32-bit system has unadorned name."
                        );
                    }
                    maybe_pf_64bit
                        .as_ref()
                        .expect_err("A 32-bit system has no 64-bit program files directory.");
                }
                PlatformArchitecture::Is32on64 => {
                    assert_eq!(
                        pf_current.as_os_str(),
                        pf_x86.as_os_str(),
                        "Ours is identical to 32-bit path."
                    );
                    let pf_64bit = maybe_pf_64bit.as_ref().expect("64-bit program files exists.");
                    assert_ne!(&pf_x86, pf_64bit, "32-bit and 64-bit program files directories differ.");
                }
                PlatformArchitecture::Is64on64 => {
                    let pf_64bit = maybe_pf_64bit.as_ref().expect("64-bit program files exists.");
                    assert_eq!(
                        pf_current.as_os_str(),
                        pf_64bit.as_os_str(),
                        "Ours is identical to 64-bit path."
                    );
                    assert_ne!(&pf_x86, pf_64bit, "32-bit and 64-bit program files directories differ.");
                }
            }

            Self {
                pf_current,
                pf_x86,
                maybe_pf_64bit,
            }
        }
    }

    #[test]
    #[cfg(windows)]
    fn alternative_locations() {
        let PathsByRole {
            pf_current,
            pf_x86,
            maybe_pf_64bit,
        } = PathsByRole::obtain_envlessly().validate();

        // FIXME: Assert the relationships between the above values and ALTERNATIVE_LOCATIONS contents!
    }

    #[test]
    #[cfg(not(windows))]
    fn alternative_locations() {
        assert!(super::ALTERNATIVE_LOCATIONS.is_empty());
    }
}
