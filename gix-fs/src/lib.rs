//! A crate with file-system specific utilities.
#![deny(rust_2018_idioms, missing_docs)]
#![forbid(unsafe_code)]

use std::path::PathBuf;

/// Common knowledge about the worktree that is needed across most interactions with the work tree
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    /// If true, we assume the executable bit is honored as part of the files mode. If false, we assume the file system
    /// ignores the executable bit, hence it will be reported as 'off' even though we just tried to set it to be on.
    pub executable_bit: bool,
    /// If true, the file system supports symbolic links and we should try to create them. Otherwise symbolic links will be checked
    /// out as files which contain the link as text.
    pub symlink: bool,
}
mod capabilities;

mod snapshot;
pub use snapshot::{FileSnapshot, SharedFileSnapshot, SharedFileSnapshotMut};

///
pub mod symlink;

///
pub mod dir;

/// A stack of path components with the delegation of side-effects as the currently set path changes, component by component.
#[derive(Clone)]
pub struct Stack {
    /// The prefix/root for all paths we handle.
    root: PathBuf,
    /// the most recent known cached that we know is valid.
    current: PathBuf,
    /// The relative portion of `valid` that was added previously.
    current_relative: PathBuf,
    /// The amount of path components of 'current' beyond the roots components.
    valid_components: usize,
    /// If set, we assume the `current` element is a directory to affect calls to `(push|pop)_directory()`.
    current_is_directory: bool,
}

#[cfg(unix)]
/// Returns whether a a file has the executable permission set.
pub fn is_executable(metadata: &std::fs::Metadata) -> bool {
    use std::os::unix::fs::MetadataExt;
    (metadata.mode() & 0o100) != 0
}

#[cfg(not(unix))]
/// Returns whether a a file has the executable permission set.
pub fn is_executable(_metadata: &std::fs::Metadata) -> bool {
    false
}

///
pub mod stack;
