//! The implementation of creating an archive from a git tree, similar to `git archive`.
#![deny(rust_2018_idioms, missing_docs)]
#![forbid(unsafe_code)]

/// The error returned by [`write_to()`].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error<E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Could not find a blob or tree for archival")]
    Find(#[source] E),
}

/// The supported container formats for use in [`write_to()`].
#[derive(Default, PartialEq, Eq, Copy, Clone, Debug)]
pub enum Format {
    /// A standard `tar` archive.
    ///
    /// Use it as well if a custom container format is desired. The idea is to decode it on a separate thread
    /// to rewrite the data to the desired format.
    #[default]
    Tar,
    /// A convenience format that will `zip` deflate the `tar` stream.
    TarGz {
        /// The compression level to use for the `zlib` compression, ranging from 0 (no compression) to 9 (best compression).
        compression_level: u8,
    },
    /// Use the zip` container format, instead of `tar`, provided for convenience.
    Zip {
        /// The compression level to use for the `zlib` compression, ranging from 0 (no compression) to 9 (best compression).
        compression_level: u8,
    },
}

/// Options for configuring [`write_to()`].
#[derive(Clone, Debug)]
pub struct Options {
    /// The archive's format.
    pub format: Format,
    /// Given a `path`, originating in the git tree, to place into the archive, put `<prefix>/path` in front of it.
    pub tree_prefix: Option<String>,
    /// The modification time for all entries in the archive.
    ///
    /// Defaults to the current time. The caller may set this to the commit time if available.
    pub modification_time: std::time::SystemTime,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            format: Default::default(),
            tree_prefix: None,
            modification_time: std::time::SystemTime::now(),
        }
    }
}

mod write;
pub use write::write_to;
