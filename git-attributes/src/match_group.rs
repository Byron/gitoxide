use crate::{MatchGroup, PatternList, PatternMapping};
use bstr::{BStr, BString, ByteSlice, ByteVec};
use std::ffi::OsString;
use std::io::Read;
use std::path::{Path, PathBuf};

/// A marker trait to identify the type of a description.
pub trait Tag: Clone + PartialEq + Eq + std::fmt::Debug + std::hash::Hash + Ord + PartialOrd + Default {
    /// The value associated with a pattern.
    type Value: PartialEq + Eq + std::fmt::Debug + std::hash::Hash + Ord + PartialOrd + Clone;
}

/// Identify ignore patterns.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Default)]
pub struct Ignore;

impl Tag for Ignore {
    type Value = ();
}

/// Identify patterns with attributes.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Default)]
pub struct Attributes;

impl Tag for Attributes {
    /// TODO: identify the actual value, should be name/State pairs, but there is the question of storage.
    type Value = ();
}

/// Describes a matching value within a [`MatchGroup`].
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub struct Match<'a, T> {
    pub pattern: &'a git_glob::Pattern,
    /// The value associated with the pattern.
    pub value: &'a T,
    /// The path to the source from which the pattern was loaded, or `None` if it was specified by other means.
    pub source: Option<&'a Path>,
    /// The line at which the pattern was found in its `source` file, or the occurrence in which it was provided.
    pub sequence_number: usize,
}

impl<T> MatchGroup<T>
where
    T: Tag,
{
    /// Match `relative_path`, a path relative to the repository containing all patterns.
    // TODO: better docs
    pub fn pattern_matching_relative_path<'a>(
        &self,
        relative_path: impl Into<&'a BStr>,
        is_dir: bool,
        case: git_glob::pattern::Case,
    ) -> Option<Match<'_, T::Value>> {
        let relative_path = relative_path.into();
        let basename_pos = relative_path.rfind(b"/").map(|p| p + 1);
        self.patterns
            .iter()
            .rev()
            .find_map(|pl| pl.pattern_matching_relative_path(relative_path, basename_pos, is_dir, case))
    }
}

impl MatchGroup<Ignore> {
    /// Given `git_dir`, a `.git` repository, load ignore patterns from `info/exclude` and from `excludes_file` if it
    /// is provided.
    /// Note that it's not considered an error if the provided `excludes_file` does not exist.
    pub fn from_git_dir(
        git_dir: impl AsRef<Path>,
        excludes_file: Option<PathBuf>,
        buf: &mut Vec<u8>,
    ) -> std::io::Result<Self> {
        let mut group = Self::default();

        // order matters! More important ones first.
        group.patterns.extend(
            excludes_file
                .map(|file| PatternList::<Ignore>::from_file(file, None, buf))
                .transpose()?
                .flatten(),
        );
        group.patterns.extend(PatternList::<Ignore>::from_file(
            git_dir.as_ref().join("info").join("exclude"),
            None,
            buf,
        )?);
        Ok(group)
    }

    /// See [PatternList::<Ignore>::from_overrides()] for details.
    pub fn from_overrides(patterns: impl IntoIterator<Item = impl Into<OsString>>) -> Self {
        MatchGroup {
            patterns: vec![PatternList::<Ignore>::from_overrides(patterns)],
        }
    }

    /// Add the given file at `source` if it exists, otherwise do nothing. If a `root` is provided, it's not considered a global file anymore.
    /// Returns true if the file was added, or false if it didn't exist.
    pub fn add_patterns_file(&mut self, source: impl Into<PathBuf>, root: Option<&Path>) -> std::io::Result<bool> {
        let mut buf = Vec::with_capacity(1024);
        let previous_len = self.patterns.len();
        self.patterns
            .extend(PatternList::<Ignore>::from_file(source.into(), root, &mut buf)?);
        Ok(self.patterns.len() != previous_len)
    }

    pub fn add_patterns_buffer(&mut self, bytes: &[u8], source: impl Into<PathBuf>, root: Option<&Path>) {
        self.patterns
            .push(PatternList::<Ignore>::from_bytes(bytes, source.into(), root));
    }
}

fn read_in_full_ignore_missing(path: &Path, buf: &mut Vec<u8>) -> std::io::Result<bool> {
    buf.clear();
    Ok(match std::fs::File::open(path) {
        Ok(mut file) => {
            file.read_to_end(buf)?;
            true
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => false,
        Err(err) => return Err(err),
    })
}
impl PatternList<Ignore> {
    /// `source` is the location of the `bytes` which represent a list of patterns line by line.
    pub fn from_bytes(bytes: &[u8], source: impl Into<PathBuf>, root: Option<&Path>) -> Self {
        let source = source.into();
        let patterns = crate::parse::ignore(bytes)
            .map(|(pattern, line_number)| PatternMapping {
                pattern,
                value: (),
                sequence_number: line_number,
            })
            .collect();

        let base = root
            .and_then(|root| source.parent().expect("file").strip_prefix(root).ok())
            .and_then(|base| {
                (!base.as_os_str().is_empty()).then(|| {
                    let mut base: BString = git_features::path::convert::to_unix_separators(
                        git_features::path::into_bytes_or_panic_on_windows(base),
                    )
                    .into_owned()
                    .into();
                    base.push_byte(b'/');
                    base
                })
            });
        PatternList {
            patterns,
            source: Some(source),
            base,
        }
    }
    pub fn from_file(
        source: impl Into<PathBuf>,
        root: Option<&Path>,
        buf: &mut Vec<u8>,
    ) -> std::io::Result<Option<Self>> {
        let source = source.into();
        Ok(read_in_full_ignore_missing(&source, buf)?.then(|| Self::from_bytes(buf, source, root)))
    }
}

impl<T> PatternList<T>
where
    T: Tag,
{
    fn pattern_matching_relative_path(
        &self,
        relative_path: &BStr,
        basename_pos: Option<usize>,
        is_dir: bool,
        case: git_glob::pattern::Case,
    ) -> Option<Match<'_, T::Value>> {
        let (relative_path, basename_start_pos) = match self.base.as_deref() {
            Some(base) => (
                relative_path.strip_prefix(base.as_slice())?.as_bstr(),
                basename_pos.and_then(|pos| {
                    let pos = pos - base.len();
                    (pos != 0).then(|| pos)
                }),
            ),
            None => (relative_path, basename_pos),
        };
        self.patterns.iter().rev().find_map(
            |PatternMapping {
                 pattern,
                 value,
                 sequence_number,
             }| {
                pattern
                    .matches_repo_relative_path(relative_path, basename_start_pos, is_dir, case)
                    .then(|| Match {
                        pattern,
                        value,
                        source: self.source.as_deref(),
                        sequence_number: *sequence_number,
                    })
            },
        )
    }
}

impl PatternList<Ignore> {
    /// Parse a list of patterns, using slashes as path separators
    pub fn from_overrides(patterns: impl IntoIterator<Item = impl Into<OsString>>) -> Self {
        PatternList {
            patterns: patterns
                .into_iter()
                .map(Into::into)
                .enumerate()
                .filter_map(|(seq_id, pattern)| {
                    let pattern = git_features::path::into_bytes(PathBuf::from(pattern)).ok()?;
                    git_glob::parse(pattern.as_ref()).map(|p| PatternMapping {
                        pattern: p,
                        value: (),
                        sequence_number: seq_id,
                    })
                })
                .collect(),
            source: None,
            base: None,
        }
    }
}
