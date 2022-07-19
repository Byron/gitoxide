use crate::file::init::Options;
use crate::file::{init, Metadata};
use crate::{file, file::init::includes, parse, File};
use git_features::threading::OwnShared;
use std::collections::BTreeSet;

/// The error returned by [`File::from_paths_metadata()`] and [`File::from_path_with_buf()`].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Init(#[from] init::Error),
    #[error("Not a single path was provided to load the configuration from")]
    NoInput,
}

/// Instantiation from one or more paths
impl File<'static> {
    /// Load the single file at `path` with `source` without following include directives.
    ///
    /// Note that the path will be checked for ownership to derive trust.
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
        options: Options<'_>,
    ) -> Result<Self, Error> {
        let path = path.into();
        buf.clear();
        std::io::copy(&mut std::fs::File::open(&path)?, buf)?;

        meta.path = path.into();
        let meta = OwnShared::new(meta);
        let mut config = Self::from_parse_events_no_includes(
            parse::Events::from_bytes_owned(buf, options.to_event_filter()).map_err(init::Error::from)?,
            OwnShared::clone(&meta),
        );
        let mut buf = Vec::new();
        includes::resolve(&mut config, meta, &mut buf, options).map_err(init::Error::from)?;

        Ok(config)
    }

    /// Constructs a `git-config` file from the provided metadata, which must include a path to read from or be ignored.
    pub fn from_paths_metadata(
        path_meta: impl IntoIterator<Item = impl Into<file::Metadata>>,
        options: Options<'_>,
    ) -> Result<Self, Error> {
        let mut target = None;
        let mut buf = Vec::with_capacity(512);
        let mut seen = BTreeSet::default();
        for (path, meta) in path_meta.into_iter().filter_map(|meta| {
            let mut meta = meta.into();
            meta.path.take().map(|p| (p, meta))
        }) {
            if !seen.insert(path.clone()) {
                continue;
            }
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
