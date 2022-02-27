#![forbid(unsafe_code, rust_2018_idioms)]

/// file system related utilities
pub mod fs {
    /// Common knowledge about the worktree that is needed across most interactions with the work tree
    pub struct Context {
        /// If true, the filesystem will store paths as decomposed unicode, i.e. `ä` becomes `"a\u{308}"`, which means that
        /// we have to turn these forms back from decomposed to precomposed unicode before storing it in the index or generally
        /// using it. This also applies to input received from the command-line, so callers may have to be aware of this and
        /// perform conversions accordingly.
        /// If false, no conversions will be performed.
        pub precompose_unicode: bool,
        /// If true, the filesystem ignores the case of input, which makes `A` the same file as `a`.
        /// This is also called case-folding.
        pub ignore_case: bool,
        /// If true, we assume the the executable bit is honored as part of the files mode. If false, we assume the file system
        /// ignores the executable bit, hence it will be reported as 'off' even though we just tried to set it to be on.
        pub file_mode: bool,
        /// If true, the file system supports symbolic links and we should try to create them. Otherwise symbolic links will be checked
        /// out as files which contain the link as text.
        pub symlink: bool,
    }

    #[cfg(windows)]
    impl Default for Context {
        fn default() -> Self {
            Context {
                precompose_unicode: false,
                ignore_case: true,
                file_mode: false,
                symlink: false,
            }
        }
    }

    #[cfg(target_os = "macos")]
    impl Default for Context {
        fn default() -> Self {
            Context {
                precompose_unicode: true,
                ignore_case: true,
                file_mode: true,
                symlink: true,
            }
        }
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    impl Default for Context {
        fn default() -> Self {
            Context {
                precompose_unicode: false,
                ignore_case: false,
                file_mode: true,
                symlink: true,
            }
        }
    }
}

pub mod index;
