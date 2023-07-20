//! The implementation of creating an archive from a worktree stream, similar to `git archive`.
//!
//! ## Deviation
//!
//! This implementation is early and just does the basics. Git does more to support more context when filtering and to keep
//! more information about entries in the various archive formats.
//! `tar` is implemented in a very basic fashion only.
//!
//! ## Feature Flags
//! All features are related to which container formats are available.
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![deny(rust_2018_idioms, missing_docs)]
#![forbid(unsafe_code)]

use bstr::BString;

/// The error returned by [`write_stream()`].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    NextStreamEntry(#[from] gix_worktree_stream::entry::Error),
    #[error("The internal format cannot be used as an archive, it's merely a debugging tool")]
    InternalFormatMustNotPersist,
}

/// The supported container formats for use in [`write_stream()`].
#[derive(Default, PartialEq, Eq, Copy, Clone, Debug)]
pub enum Format {
    /// An internal format that is suitable only for intra-process communication.
    ///
    /// All transformations in the options are ignored. Calling [`write_stream`] is disallowed
    /// as it's more efficient to call [gix_worktree_stream::Stream::into_read()] right away.
    /// It is provided here as a basis available without extra dependencies, and as a debugging tool.
    #[default]
    InternalTransientNonPersistable,
    /// A standard `tar` archive.
    ///
    /// Use it as well if a custom container format is desired. The idea is to decode it on a separate thread
    /// to rewrite the data to the desired format.
    #[cfg(feature = "tar")]
    Tar,
}

/// Options for configuring [`write_stream()`].
#[derive(Clone, Debug)]
pub struct Options {
    /// The archive's format.
    pub format: Format,
    /// Given a `path`, originating in the git tree, to place into the archive, put `<prefix>/path` in front of it.
    ///
    /// Note that that `/` should be used as separator, and that a prefix directory has to end with `/`.
    pub tree_prefix: Option<BString>,
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
pub use write::write_stream;
