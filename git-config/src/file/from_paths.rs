use crate::{file::resolve_includes, parse, path::interpolate, File};

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
#[derive(Clone, Copy)]
pub struct Options<'a> {
    /// Used during path interpolation.
    pub interpolate: interpolate::Options<'a>,
    /// The maximum allowed length of the file include chain built by following nested resolve_includes where base level is depth = 0.
    pub max_depth: u8,
    /// When max depth is exceeded while following nested included, return an error if true or silently stop following
    /// resolve_includes.
    ///
    /// Setting this value to false allows to read configuration with cycles, which otherwise always results in an error.
    pub error_on_max_depth_exceeded: bool,
    /// The location of the .git directory
    ///
    /// Used for conditional includes, e.g. `gitdir:` or `gitdir/i`.
    pub git_dir: Option<&'a std::path::Path>,
    /// The name of the branch that is currently checked out
    ///
    /// Used for conditional includes, e.g. `onbranch:`
    pub branch_name: Option<&'a git_ref::FullNameRef>,
}

impl Default for Options<'_> {
    fn default() -> Self {
        Options {
            interpolate: Default::default(),
            max_depth: 10,
            error_on_max_depth_exceeded: true,
            git_dir: None,
            branch_name: None,
        }
    }
}

impl File<'static> {
    /// Open a single configuration file by reading `path` into `buf` and copying all contents from there, without resolving includes.
    pub fn from_path_with_buf(path: &std::path::Path, buf: &mut Vec<u8>) -> Result<Self, Error> {
        buf.clear();
        std::io::copy(&mut std::fs::File::open(path)?, buf)?;
        Self::from_bytes(buf)
    }

    /// Constructs a `git-config` file from the provided paths in the order provided.
    /// This is neither zero-copy nor zero-alloc.
    pub fn from_paths(
        paths: impl IntoIterator<Item = impl AsRef<std::path::Path>>,
        options: Options<'_>,
    ) -> Result<Self, Error> {
        let mut target = Self::default();
        let mut buf = Vec::with_capacity(512);
        for path in paths {
            let path = path.as_ref();
            let mut config = Self::from_path_with_buf(path, &mut buf)?;
            resolve_includes(&mut config, Some(path), &mut buf, options)?;
            target.append(config);
        }
        Ok(target)
    }

    pub(crate) fn from_bytes(input: &[u8]) -> Result<Self, Error> {
        Ok(parse::Events::from_bytes_owned(input, None)?.into())
    }
}
