use crate::{parser, values::path::interpolate};

/// The error returned by [`File::from_paths()`][crate::File::from_paths()] and [`File::from_env_paths()`][crate::File::from_env_paths()].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    ParserOrIoError(#[from] parser::ParserOrIoError<'static>),
    #[error(transparent)]
    Interpolate(#[from] interpolate::Error),
    #[error("The maximum allowed length {} of the file include chain built by following nested resolve_includes is exceeded", .max_depth)]
    IncludeDepthExceeded { max_depth: u8 },
    #[error("Include paths from environment variables must not be relative")]
    MissingConfigPath,
}

/// Options when loading git config using [`File::from_paths()`][crate::File::from_paths()].
#[derive(Clone, Copy)]
pub struct Options<'a> {
    /// The location where gitoxide or git is installed
    pub git_install_dir: Option<&'a std::path::Path>,
    /// The maximum allowed length of the file include chain built by following nested resolve_includes where base level is depth = 0.
    pub max_depth: u8,
    /// When max depth is exceeded while following nested included, return an error if true or silently stop following
    /// resolve_includes.
    ///
    /// Setting this value to false allows to read configuration with cycles, which otherwise always results in an error.
    pub error_on_max_depth_exceeded: bool,
    /// The location of the .git directory
    pub git_dir: Option<&'a std::path::Path>,
    /// The name of the branch that is currently checked out
    pub branch_name: Option<&'a git_ref::FullNameRef>,
}

impl<'a> Default for Options<'a> {
    fn default() -> Self {
        Options {
            git_install_dir: None,
            max_depth: 10,
            error_on_max_depth_exceeded: true,
            git_dir: None,
            branch_name: None,
        }
    }
}
