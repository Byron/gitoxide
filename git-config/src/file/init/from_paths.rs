use crate::file::init::Options;
use crate::file::{init, Metadata};
use crate::File;
use std::collections::BTreeSet;

/// The error returned by [`File::from_paths_metadata()`] and [`File::from_path_no_includes()`].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Init(#[from] init::Error),
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
        std::io::copy(&mut std::fs::File::open(&path)?, &mut buf)?;

        Ok(File::from_bytes_owned(
            &mut buf,
            Metadata::from(source).at(path).with(trust),
            Default::default(),
        )?)
    }

    /// Constructs a `git-config` file from the provided metadata, which must include a path to read from or be ignored.
    /// Returns `Ok(None)` if there was not a single input path provided, which is a possibility due to
    /// [`Metadata::path`] being an `Option`.
    pub fn from_paths_metadata(
        path_meta: impl IntoIterator<Item = impl Into<Metadata>>,
        options: Options<'_>,
    ) -> Result<Option<Self>, Error> {
        let mut target = None;
        let mut buf = Vec::with_capacity(512);
        let mut seen = BTreeSet::default();
        for (path, mut meta) in path_meta.into_iter().filter_map(|meta| {
            let mut meta = meta.into();
            meta.path.take().map(|p| (p, meta))
        }) {
            if !seen.insert(path.clone()) {
                continue;
            }

            buf.clear();
            std::io::copy(&mut std::fs::File::open(&path)?, &mut buf)?;
            meta.path = Some(path);

            let config = Self::from_bytes_owned(&mut buf, meta, options)?;
            match &mut target {
                None => {
                    target = Some(config);
                }
                Some(target) => {
                    target.append(config);
                }
            }
        }
        Ok(target)
    }
}
