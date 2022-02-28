use std::path::Path;

/// Common knowledge about the worktree that is needed across most interactions with the work tree
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
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

impl Context {
    /// try to determine all values in this context by probing them in the given `directory`, which
    /// should be on the file system the git repository is located on.
    ///
    /// All errors are ignored and interpreted on top of the default for the platform the binary is compiled for.
    pub fn probe(directory: impl AsRef<std::path::Path>) -> Self {
        let root = directory.as_ref();
        let ctx = Context::default();
        Context {
            symlink: Self::probe_symlink(root).unwrap_or(ctx.symlink),
            ..ctx
        }
    }

    fn probe_symlink(root: &Path) -> std::io::Result<bool> {
        let src_path = root.join("__link_src_file");
        std::fs::File::options().create_new(true).write(true).open(&src_path)?;
        let link_path = root.join("__file_link");
        if symlink::symlink_file(&src_path, &link_path).is_err() {
            std::fs::remove_file(&src_path)?;
            return Ok(false);
        }
        let cleanup_all = || {
            let res = std::fs::remove_file(&src_path);
            symlink::remove_symlink_file(&link_path)
                .or_else(|_| std::fs::remove_file(&link_path))
                .and(res)
        };

        let res = std::fs::symlink_metadata(&link_path)
            .or_else(|err| cleanup_all().and(Err(err)))?
            .is_symlink();

        cleanup_all()?;
        Ok(res)
    }
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
