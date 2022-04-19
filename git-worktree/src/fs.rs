use std::path::{Path, PathBuf};

/// Common knowledge about the worktree that is needed across most interactions with the work tree
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
pub struct Capabilities {
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
    pub executable_bit: bool,
    /// If true, the file system supports symbolic links and we should try to create them. Otherwise symbolic links will be checked
    /// out as files which contain the link as text.
    pub symlink: bool,
}

impl Capabilities {
    /// try to determine all values in this context by probing them in the given `git_dir`, which
    /// should be on the file system the git repository is located on.
    /// `git_dir` is a typical git repository, expected to be populated with the typical files like `config`.
    ///
    /// All errors are ignored and interpreted on top of the default for the platform the binary is compiled for.
    pub fn probe(git_dir: impl AsRef<Path>) -> Self {
        let root = git_dir.as_ref();
        let ctx = Capabilities::default();
        Capabilities {
            symlink: Self::probe_symlink(root).unwrap_or(ctx.symlink),
            ignore_case: Self::probe_ignore_case(root).unwrap_or(ctx.ignore_case),
            precompose_unicode: Self::probe_precompose_unicode(root).unwrap_or(ctx.precompose_unicode),
            executable_bit: Self::probe_file_mode(root).unwrap_or(ctx.executable_bit),
        }
    }

    #[cfg(unix)]
    fn probe_file_mode(root: &Path) -> std::io::Result<bool> {
        use std::os::unix::fs::{MetadataExt, OpenOptionsExt};

        // test it exactly as we typically create executable files, not using chmod.
        let test_path = root.join("_test_executable_bit");
        let res = std::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .mode(0o777)
            .open(&test_path)
            .and_then(|f| f.metadata().map(|m| m.mode() & 0o100 == 0o100));
        std::fs::remove_file(test_path)?;
        res
    }

    #[cfg(not(unix))]
    fn probe_file_mode(_root: &Path) -> std::io::Result<bool> {
        Ok(false)
    }

    fn probe_ignore_case(git_dir: &Path) -> std::io::Result<bool> {
        std::fs::metadata(git_dir.join("cOnFiG")).map(|_| true).or_else(|err| {
            if err.kind() == std::io::ErrorKind::NotFound {
                Ok(false)
            } else {
                Err(err)
            }
        })
    }

    fn probe_precompose_unicode(root: &Path) -> std::io::Result<bool> {
        let precomposed = "ä";
        let decomposed = "a\u{308}";

        let precomposed = root.join(precomposed);
        std::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&precomposed)?;
        let res = root.join(decomposed).symlink_metadata().map(|_| true);
        std::fs::remove_file(precomposed)?;
        res
    }

    fn probe_symlink(root: &Path) -> std::io::Result<bool> {
        let src_path = root.join("__link_src_file");
        std::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&src_path)?;
        let link_path = root.join("__file_link");
        if crate::os::create_symlink(&src_path, &link_path).is_err() {
            std::fs::remove_file(&src_path)?;
            return Ok(false);
        }

        let res = std::fs::symlink_metadata(&link_path).map(|m| m.is_symlink());

        let cleanup = crate::os::remove_symlink(&link_path).or_else(|_| std::fs::remove_file(&link_path));
        std::fs::remove_file(&src_path).and(cleanup)?;

        res
    }
}

#[cfg(windows)]
impl Default for Capabilities {
    fn default() -> Self {
        Capabilities {
            precompose_unicode: false,
            ignore_case: true,
            executable_bit: false,
            symlink: false,
        }
    }
}

#[cfg(target_os = "macos")]
impl Default for Capabilities {
    fn default() -> Self {
        Capabilities {
            precompose_unicode: true,
            ignore_case: true,
            executable_bit: true,
            symlink: true,
        }
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
impl Default for Capabilities {
    fn default() -> Self {
        Capabilities {
            precompose_unicode: false,
            ignore_case: false,
            executable_bit: true,
            symlink: true,
        }
    }
}

pub struct Stack {
    /// The prefix/root for all paths we handle.
    root: PathBuf,
    /// the most recent known cached that we know is valid.
    current: PathBuf,
    /// The relative portion of `valid` that was added previously.
    current_relative: PathBuf,
    /// The amount of path components of 'current' beyond the roots components.
    valid_components: usize,
}

pub mod stack {
    use crate::fs::Stack;
    use std::path::{Path, PathBuf};

    impl Stack {
        pub fn root(&self) -> &Path {
            &self.root
        }

        pub fn current(&self) -> &Path {
            &self.current
        }

        pub fn current_relative(&self) -> &Path {
            &self.current_relative
        }
    }

    impl Stack {
        /// Create a new instance with `root` being the base for all future paths we handle, assuming it to be valid which includes
        /// symbolic links to be included in it as well.
        pub fn new(root: impl Into<PathBuf>) -> Self {
            let root = root.into();
            Stack {
                current: root.clone(),
                current_relative: PathBuf::with_capacity(128),
                valid_components: 0,
                root,
            }
        }

        /// Set the current stack to point to the `relative` path and call `push_comp()` each time a new path component is popped
        /// along with the stacks state for inspection to perform an operation that produces some data.
        ///
        /// The full path to `relative` will be returned along with the data returned by push_comp.
        pub fn make_relative_path_current(
            &mut self,
            relative: impl AsRef<Path>,
            mut push_comp: impl FnMut(&mut std::iter::Peekable<std::path::Components<'_>>, &Self) -> std::io::Result<()>,
            mut pop_comp: impl FnMut(&Self),
        ) -> std::io::Result<()> {
            let relative = relative.as_ref();
            debug_assert!(
                relative.is_relative(),
                "only index paths are handled correctly here, must be relative"
            );

            let mut components = relative.components().peekable();
            let mut existing_components = self.current_relative.components();
            let mut matching_components = 0;
            while let (Some(existing_comp), Some(new_comp)) = (existing_components.next(), components.peek()) {
                if existing_comp == *new_comp {
                    components.next();
                    matching_components += 1;
                } else {
                    break;
                }
            }

            for _ in 0..self.valid_components - matching_components {
                self.current.pop();
                self.current_relative.pop();
                pop_comp(&*self);
            }
            self.valid_components = matching_components;

            while let Some(comp) = components.next() {
                self.current.push(comp);
                self.current_relative.push(comp);
                self.valid_components += 1;
                let res = push_comp(&mut components, &*self);

                if let Err(err) = res {
                    self.current.pop();
                    self.current_relative.pop();
                    self.valid_components -= 1;
                    pop_comp(&*self);
                    return Err(err);
                }
            }
            Ok(())
        }
    }
}
