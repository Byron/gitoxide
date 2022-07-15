use crate::{file, file::init::resolve_includes, parse, path::interpolate, File};

/// The error returned by [`File::from_paths()`][crate::File::from_paths()] and [`File::from_env_paths()`][crate::File::from_env_paths()].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Parse(#[from] parse::Error),
    #[error(transparent)]
    Interpolate(#[from] interpolate::Error),
    #[error("The maximum allowed length {} of the file include chain built by following nested resolve_includes is exceeded", .max_depth)]
    IncludeDepthExceeded { max_depth: u8 },
    #[error("Include paths from environment variables must not be relative as no config file paths exists as root")]
    MissingConfigPath,
    #[error("The git directory must be provided to support `gitdir:` conditional includes")]
    MissingGitDir,
    #[error(transparent)]
    Realpath(#[from] git_path::realpath::Error),
}

/// Options when loading git config using [`File::from_paths()`][crate::File::from_paths()].
#[derive(Clone, Copy, Default)]
pub struct Options<'a> {
    /// Configure how to follow includes while handling paths.
    pub resolve_includes: file::resolve_includes::Options<'a>,
}

/// Instantiation from one or more paths
impl File<'static> {
    /// Open a single configuration file by reading all data at `path` into `buf` and
    /// copying all contents from there, without resolving includes.
    pub fn from_path_with_buf(path: &std::path::Path, buf: &mut Vec<u8>) -> Result<Self, Error> {
        buf.clear();
        std::io::copy(&mut std::fs::File::open(path)?, buf)?;
        Self::from_bytes(buf)
    }

    /// Constructs a `git-config` file from the provided paths in the order provided.
    pub fn from_paths(
        paths: impl IntoIterator<Item = impl AsRef<std::path::Path>>,
        options: Options<'_>,
    ) -> Result<Self, Error> {
        let mut target = Self::default();
        let mut buf = Vec::with_capacity(512);
        for path in paths {
            let path = path.as_ref();
            let mut config = Self::from_path_with_buf(path, &mut buf)?;
            resolve_includes(&mut config, Some(path), &mut buf, options.resolve_includes)?;
            target.append(config);
        }
        Ok(target)
    }

    pub(crate) fn from_bytes(input: &[u8]) -> Result<Self, Error> {
        Ok(parse::Events::from_bytes_owned(input, None)?.into())
    }
}
