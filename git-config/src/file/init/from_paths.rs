use crate::file::Metadata;
use crate::parse::Event;
use crate::{file, file::init::includes, parse, path::interpolate, File};
use git_features::threading::OwnShared;

/// The error returned by [`File::from_paths_metadata()`] and [`File::from_env_paths()`].
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
    #[error("Not a single path was provided to load the configuration from")]
    NoInput,
}

/// Options when loading git config using [`File::from_paths_metadata()`].
#[derive(Clone, Copy, Default)]
pub struct Options<'a> {
    /// Configure how to follow includes while handling paths.
    pub includes: file::includes::Options<'a>,
    /// If true, only value-bearing parse events will be kept to reduce memory usage and increase performance.
    ///
    /// Note that doing so will prevent [`write_to()`][File::write_to()] to serialize itself meaningfully and correctly,
    /// as newlines will be missing. Use this only if it's clear that serialization will not be attempted.
    pub lossy: bool,
}

/// Instantiation from one or more paths
impl File<'static> {
    /// Load the file at `path` from `source` without following include directives. Note that the path will be checked for
    /// ownership to derive trust.
    pub fn from_path_no_includes(path: impl Into<std::path::PathBuf>, source: crate::Source) -> Result<Self, Error> {
        let path = path.into();
        let trust = git_sec::Trust::from_path_ownership(&path)?;
        let mut buf = Vec::new();
        File::from_path_with_buf(path, &mut buf, Metadata::from(source).with(trust), Default::default())
    }

    /// Open a single configuration file by reading all data at `path` into `buf` and
    /// copying all contents from there, without resolving includes. Note that the `path` in `meta`
    /// will be set to the one provided here.
    pub fn from_path_with_buf(
        path: impl Into<std::path::PathBuf>,
        buf: &mut Vec<u8>,
        mut meta: file::Metadata,
        Options {
            includes: include_options,
            lossy,
        }: Options<'_>,
    ) -> Result<Self, Error> {
        let path = path.into();
        buf.clear();
        std::io::copy(&mut std::fs::File::open(&path)?, buf)?;

        meta.path = path.into();
        let meta = OwnShared::new(meta);
        let mut config = Self::from_parse_events(
            parse::Events::from_bytes_owned(buf, if lossy { Some(discard_nonessential_events) } else { None })?,
            OwnShared::clone(&meta),
        );
        let mut buf = Vec::new();
        includes::resolve(&mut config, meta, &mut buf, include_options)?;

        Ok(config)
    }

    /// Constructs a `git-config` file from the provided metadata, which must include a path to read from or be ignored.
    pub fn from_paths_metadata(
        path_meta: impl IntoIterator<Item = impl Into<file::Metadata>>,
        options: Options<'_>,
    ) -> Result<Self, Error> {
        let mut target = None;
        let mut buf = Vec::with_capacity(512);
        for (path, meta) in path_meta.into_iter().filter_map(|meta| {
            let mut meta = meta.into();
            meta.path.take().map(|p| (p, meta))
        }) {
            let config = Self::from_path_with_buf(path, &mut buf, meta, options)?;
            match &mut target {
                None => {
                    target = Some(config);
                }
                Some(target) => {
                    target.append(config);
                }
            }
        }
        target.ok_or(Error::NoInput)
    }
}

fn discard_nonessential_events(e: &Event<'_>) -> bool {
    match e {
        Event::Whitespace(_) | Event::Comment(_) | Event::Newline(_) => false,
        Event::SectionHeader(_)
        | Event::SectionKey(_)
        | Event::KeyValueSeparator
        | Event::Value(_)
        | Event::ValueNotDone(_)
        | Event::ValueDone(_) => true,
    }
}
