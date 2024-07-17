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
mod locations {
    use std::ffi::{OsStr, OsString};
    use std::io::ErrorKind;
    use std::path::{Path, PathBuf};

    use known_folders::{get_known_folder_path, KnownFolder};
    use windows::core::Result as WindowsResult;
    use windows::Win32::Foundation::BOOL;
    use windows::Win32::System::Threading::{GetCurrentProcess, IsWow64Process};
    use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_QUERY_VALUE};
    use winreg::RegKey;

    macro_rules! var_os_stub {
        { $($name:expr => $value:expr),* $(,)? } => {
            |key| {
                match key {
                    $(
                        $name => Some(OsString::from($value)),
                    )*
                    _ => None,
                }
            }
        }
    }

    macro_rules! locations_from {
        ($($name:expr => $value:expr),* $(,)?) => {
            super::super::locations_under_program_files(var_os_stub! {
                $(
                    $name => $value,
                )*
            })
        }
    }

    macro_rules! pathbuf_vec {
        [$($path:expr),* $(,)?] => {
            vec![$(
                PathBuf::from($path),
            )*]
        }
    }

    #[test]
    fn locations_under_program_files_ordinary() {
        assert_eq!(
            locations_from!(
                "ProgramFiles" => r"C:\Program Files",
            ),
            if cfg!(target_pointer_width = "64") {
                pathbuf_vec![r"C:\Program Files\Git\mingw64\bin"]
            } else {
                pathbuf_vec![r"C:\Program Files\Git\mingw32\bin"]
            },
        );
        assert_eq!(
            locations_from!(
                "ProgramFiles" => {
                    if cfg!(target_pointer_width = "64") {
                        r"C:\Program Files"
                    } else {
                        r"C:\Program Files (x86)"
                    }
                },
                "ProgramFiles(x86)" => r"C:\Program Files (x86)",
                "ProgramW6432" => r"C:\Program Files",
            ),
            pathbuf_vec![
                r"C:\Program Files\Git\mingw64\bin",
                r"C:\Program Files (x86)\Git\mingw32\bin",
            ],
        );
        assert_eq!(locations_from!(), Vec::<PathBuf>::new());
    }

    #[test]
    fn locations_under_program_files_strange() {
        assert_eq!(
            locations_from!(
                "ProgramFiles" => r"X:\cur\rent",
                "ProgramFiles(x86)" => r"Y:\nar\row",
                "ProgramW6432" => r"Z:\wi\de",
            ),
            pathbuf_vec![
                r"Z:\wi\de\Git\mingw64\bin",
                r"Y:\nar\row\Git\mingw32\bin",
                if cfg!(target_pointer_width = "64") {
                    r"X:\cur\rent\Git\mingw64\bin"
                } else {
                    r"X:\cur\rent\Git\mingw32\bin"
                },
            ],
        );
        assert_eq!(
            locations_from!(
                "ProgramW6432" => r"Z:\wi\de",
            ),
            pathbuf_vec![r"Z:\wi\de\Git\mingw64\bin"],
        );
        assert_eq!(
            locations_from!(
                "ProgramFiles" => r"Z:/wi//de/",
                "ProgramFiles(x86)" => r"Y:/\nar\/row",
                "ProgramW6432" => r"Z:\wi\.\de",
            ),
            if cfg!(target_pointer_width = "64") {
                pathbuf_vec![r"Z:\wi\de\Git\mingw64\bin", r"Y:\nar\row\Git\mingw32\bin"]
            } else {
                pathbuf_vec![
                    r"Z:\wi\de\Git\mingw64\bin",
                    r"Y:\nar\row\Git\mingw32\bin",
                    r"Z:\wi\de\Git\mingw32\bin",
                ]
            },
        );
        assert_eq!(
            locations_from!(
                "ProgramFiles" => r"foo\bar",
                "ProgramFiles(x86)" => r"\\host\share\subdir",
                "ProgramW6432" => r"",
            ),
            pathbuf_vec![r"\\host\share\subdir\Git\mingw32\bin"],
        );
    }

    #[derive(Clone, Copy, Debug)]
    enum PlatformArchitecture {
        Is32on32,
        Is32on64,
        Is64on64,
    }

    impl PlatformArchitecture {
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

    fn ends_with_case_insensitive(full_text: &OsStr, literal_pattern: &str) -> Option<bool> {
        let folded_text = full_text.to_str()?.to_lowercase();
        let folded_pattern = literal_pattern.to_lowercase();
        Some(folded_text.ends_with(&folded_pattern))
    }

    /// The common global program files paths on this system, by process and system architecture.
    #[derive(Clone, Debug)]
    struct ProgramFilesPaths {
        /// The program files directory used for whatever architecture this program was built for.
        current: PathBuf,

        /// The x86 program files directory regardless of the architecture of the program.
        ///
        /// If Rust gains Windows targets like ARMv7 where this is unavailable, this could fail.
        x86: PathBuf,

        /// The 64-bit program files directory if there is one.
        ///
        /// This is present on x64 and also ARM64 systems. On an ARM64 system, ARM64 and AMD64
        /// programs use the same program files directory while 32-bit x86 and ARM programs use
        /// two others. Only a 32-bit has no 64-bit program files directory.
        maybe_64bit: Option<PathBuf>,
    }

    impl ProgramFilesPaths {
        /// Gets the three common kinds of global program files paths without environment variables.
        ///
        /// The idea here is to obtain this information, which the `alternative_locations()` unit
        /// test uses to learn the expected alternative locations, without duplicating *any* of the
        /// approach used for `ALTERNATIVE_LOCATIONS`, so it can be used to test that. The approach
        /// here is also more reliable than using environment variables, but it is a bit more
        /// complex, and it requires either additional dependencies or the use of unsafe code.
        ///
        /// This gets `pf_current` and `pf_x86` by the [known folders][known-folders] system. But
        /// it gets `maybe_pf_64bit` from the registry, as the corresponding known folder is not
        /// available to 32-bit processes. See the [`KNOWNFOLDDERID`][knownfolderid] documentation.
        ///
        /// If in the future the implementation of `ALTERNATIVE_LOCATIONS` uses these techniques,
        /// then this function can be changed to use environment variables and renamed accordingly.
        ///
        /// [known-folders]: https://learn.microsoft.com/en-us/windows/win32/shell/known-folders
        /// [knownfolderid]: https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#remarks
        fn obtain_envlessly() -> Self {
            let pf_current = get_known_folder_path(KnownFolder::ProgramFiles)
                .expect("The process architecture specific program files folder is always available");

            let pf_x86 = get_known_folder_path(KnownFolder::ProgramFilesX86)
                .expect("The x86 program files folder will in practice always be available");

            let maybe_pf_64bit = RegKey::predef(HKEY_LOCAL_MACHINE)
                .open_subkey_with_flags(r"SOFTWARE\Microsoft\Windows\CurrentVersion", KEY_QUERY_VALUE)
                .expect("The `CurrentVersion` registry key exists and allows reading")
                .get_value::<OsString, _>("ProgramW6432Dir")
                .map(PathBuf::from)
                .map_err(|error| {
                    assert_eq!(error.kind(), ErrorKind::NotFound);
                    error
                })
                .ok();

            Self {
                current: pf_current,
                x86: pf_x86,
                maybe_64bit: maybe_pf_64bit,
            }
        }

        /// Checks that the paths we got for testing are reasonable.
        ///
        /// This checks that `obtain_envlessly()` returned paths that are likely to be correct and
        /// that satisfy the most important properties based on the current system and process.
        fn validated(self) -> Self {
            match PlatformArchitecture::current().expect("Process and system 'bitness' should be available") {
                PlatformArchitecture::Is32on32 => {
                    assert_eq!(
                        self.current.as_os_str(),
                        self.x86.as_os_str(),
                        "Our program files path is exactly identical to the 32-bit one.",
                    );
                    for trailing_arch in [" (x86)", " (Arm)"] {
                        let is_adorned = ends_with_case_insensitive(self.current.as_os_str(), trailing_arch)
                            .expect("Assume the test system's important directories are valid Unicode");
                        assert!(
                            !is_adorned,
                            "The 32-bit program files directory name on a 32-bit system mentions no architecture.",
                        );
                    }
                    assert_eq!(
                        self.maybe_64bit, None,
                        "A 32-bit system has no 64-bit program files directory.",
                    );
                }
                PlatformArchitecture::Is32on64 => {
                    assert_eq!(
                        self.current.as_os_str(),
                        self.x86.as_os_str(),
                        "Our program files path is exactly identical to the 32-bit one.",
                    );
                    let pf_64bit = self
                        .maybe_64bit
                        .as_ref()
                        .expect("The 64-bit program files directory exists");
                    assert_ne!(
                        &self.x86, pf_64bit,
                        "The 32-bit and 64-bit program files directories have different locations.",
                    );
                }
                PlatformArchitecture::Is64on64 => {
                    let pf_64bit = self
                        .maybe_64bit
                        .as_ref()
                        .expect("The 64-bit program files directory exists");
                    assert_eq!(
                        self.current.as_os_str(),
                        pf_64bit.as_os_str(),
                        "Our program files path is exactly identical to the 64-bit one.",
                    );
                    assert_ne!(
                        &self.x86, pf_64bit,
                        "The 32-bit and 64-bit program files directories have different locations.",
                    );
                }
            }

            self
        }
    }

    /// Paths relative to process architecture specific program files directories.
    #[derive(Clone, Debug)]
    struct RelativeGitBinPaths<'a> {
        x86: &'a Path,
        maybe_64bit: Option<&'a Path>,
    }

    impl<'a> RelativeGitBinPaths<'a> {
        /// Assert that `locations` has the given path prefixes, and extract the suffixes.
        fn assert_from(pf: &'a ProgramFilesPaths, locations: &'static [PathBuf]) -> Self {
            match locations {
                [primary, secondary] => {
                    let prefix_64bit = pf
                        .maybe_64bit
                        .as_ref()
                        .expect("It gives two paths only if one can be 64-bit");
                    let suffix_64bit = primary
                        .strip_prefix(prefix_64bit)
                        .expect("It gives the 64-bit path and lists it first");
                    let suffix_x86 = secondary
                        .strip_prefix(pf.x86.as_path())
                        .expect("It gives the 32-bit path and lists it second");
                    Self {
                        x86: suffix_x86,
                        maybe_64bit: Some(suffix_64bit),
                    }
                }
                [only] => {
                    assert_eq!(pf.maybe_64bit, None, "It gives one path only if none can be 64-bit.");
                    let suffix_x86 = only
                        .strip_prefix(pf.x86.as_path())
                        .expect("The one path it gives is the 32-bit path");
                    Self {
                        x86: suffix_x86,
                        maybe_64bit: None,
                    }
                }
                other => panic!("{:?} has length {}, expected 1 or 2.", other, other.len()),
            }
        }

        /// Assert that the suffixes (relative subdirectories) are the common per-architecture Git install locations.
        fn assert_architectures(&self) {
            assert_eq!(self.x86, Path::new("Git/mingw32/bin"));

            if let Some(suffix_64bit) = self.maybe_64bit {
                // When Git for Windows releases ARM64 builds, there will be another 64-bit suffix,
                // likely clangarm64. In that case, this and other assertions will need updating,
                // as there will be two separate paths to check under the same 64-bit program files
                // directory. (See the definition of ProgramFilesPaths::maybe_64bit for details.)
                assert_eq!(suffix_64bit, Path::new("Git/mingw64/bin"));
            }
        }
    }

    #[test]
    fn alternative_locations() {
        // Obtain program files directory paths by other means and check that they seem correct.
        let pf = ProgramFilesPaths::obtain_envlessly().validated();

        // Check that `ALTERNATIVE_LOCATIONS` correspond to them, with the correct subdirectories.
        let locations = super::super::ALTERNATIVE_LOCATIONS.as_slice();
        RelativeGitBinPaths::assert_from(&pf, locations).assert_architectures();
    }
}

#[cfg(not(windows))]
mod locations {
    #[test]
    fn alternative_locations() {
        assert!(super::super::ALTERNATIVE_LOCATIONS.is_empty());
    }
}
