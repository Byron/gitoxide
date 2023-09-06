//! Utilities for searching matches of paths to patterns.
//!
//! Please note that these are specific to how both excludes and attributes are searched, and this is
//! merely a way to share code among them.
use std::path::{Path, PathBuf};

///
pub mod pattern;

/// A trait to convert bytes into patterns and their associated value.
///
/// This is used for `gitattributes` which have a value, and `gitignore` which don't.
pub trait Pattern: Clone + PartialEq + Eq + std::fmt::Debug + std::hash::Hash + Ord + PartialOrd + Default {
    /// The value associated with a pattern.
    type Value: PartialEq + Eq + std::fmt::Debug + std::hash::Hash + Ord + PartialOrd + Clone;

    /// Parse all patterns in `bytes` line by line, ignoring lines with errors, and collect them.
    fn bytes_to_patterns(bytes: &[u8], source: &Path) -> Vec<pattern::Mapping<Self::Value>>;
}

/// Add the given file at `source` if it exists, otherwise do nothing.
/// If a `root` is provided, it's not considered a global file anymore.
/// Returns `true` if the file was added, or `false` if it didn't exist.
pub fn add_patterns_file<T: Pattern>(
    patterns: &mut Vec<pattern::List<T>>,
    source: PathBuf,
    follow_symlinks: bool,
    root: Option<&Path>,
    buf: &mut Vec<u8>,
) -> std::io::Result<bool> {
    let previous_len = patterns.len();
    patterns.extend(pattern::List::<T>::from_file(source, root, follow_symlinks, buf)?);
    Ok(patterns.len() != previous_len)
}
