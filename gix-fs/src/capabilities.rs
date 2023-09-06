// TODO: tests
use std::path::Path;

use crate::Capabilities;

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

impl Capabilities {
    /// try to determine all values in this context by probing them in the given `git_dir`, which
    /// should be on the file system the git repository is located on.
    /// `git_dir` is a typical git repository, expected to be populated with the typical files like `config`.
    ///
    /// All errors are ignored and interpreted on top of the default for the platform the binary is compiled for.
    pub fn probe(git_dir: &Path) -> Self {
        let ctx = Capabilities::default();
        Capabilities {
            symlink: Self::probe_symlink(git_dir).unwrap_or(ctx.symlink),
            ignore_case: Self::probe_ignore_case(git_dir).unwrap_or(ctx.ignore_case),
            precompose_unicode: Self::probe_precompose_unicode(git_dir).unwrap_or(ctx.precompose_unicode),
            executable_bit: Self::probe_file_mode(git_dir).unwrap_or(ctx.executable_bit),
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
        if crate::symlink::create(&src_path, &link_path).is_err() {
            std::fs::remove_file(&src_path)?;
            return Ok(false);
        }

        let res = std::fs::symlink_metadata(&link_path).map(|m| m.file_type().is_symlink());

        let cleanup = crate::symlink::remove(&link_path).or_else(|_| std::fs::remove_file(&link_path));
        std::fs::remove_file(&src_path).and(cleanup)?;

        res
    }
}
