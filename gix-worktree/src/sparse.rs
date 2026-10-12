//! Sparse-checkout pattern loading and selection for initial checkout with a full index.

use std::{
    collections::{HashMap, HashSet},
    io,
    path::Path,
};

use bstr::{BStr, ByteSlice};
use gix_error::{Result, ResultExt, message};
use gix_ignore::glob::pattern::{Case, Mode};
use gix_index::entry::Flags;

/// Read `info/sparse-checkout` from a worktree's private `git_dir`, not its common directory.
///
/// A missing file returns `None`; an empty file returns `Some` with an empty buffer. Other read errors
/// are propagated. The caller must check whether sparse checkout is enabled before calling this function.
pub fn read_patterns(git_dir: &Path) -> io::Result<Option<Vec<u8>>> {
    match std::fs::read(git_dir.join("info/sparse-checkout")) {
        Ok(patterns) => Ok(Some(patterns)),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(err) => Err(err),
    }
}

/// Mark entries excluded by `patterns` with `SKIP_WORKTREE` and `EXTENDED` for initial checkout.
///
/// `index` must be a full index with no pre-existing `SKIP_WORKTREE` flags. Included entries and their
/// flags are left unchanged; this does not expand or produce a sparse index or update an existing checkout.
/// Gitlinks are treated as absent paths, not directories, when matching directory-only patterns.
/// `source` identifies the patterns for diagnostics and is not read from disk.
///
/// If `cone` is enabled, valid cone patterns select directories and their parent files. Invalid cone
/// patterns fall back to ordinary sparse-pattern matching. `case` controls matching and cone validation.
/// Ancestor decisions are reused between entries with shared directory prefixes, using memory proportional
/// to path depth. Sorted entries benefit most, but sorting is not required.
pub fn apply_patterns(
    index: &mut gix_index::State,
    patterns: &[u8],
    source: &Path,
    cone: bool,
    case: Case,
) -> Result<()> {
    let mut search = gix_ignore::Search::default();
    search
        .add_patterns_buffer(patterns, source, None, Default::default())
        .or_raise(|| message("Could not parse sparse-checkout patterns"))?;
    let cone = cone.then(|| Cone::from_search(&search, case)).flatten();
    if cone.as_ref().is_some_and(|cone| cone.full) {
        return Ok(());
    }
    let mut cone_directories = Directories::default();
    let mut pattern_directories = Directories::default();
    let mut folded_path = Vec::new();
    for (entry, path) in index.entries_mut_with_paths() {
        let included = match &cone {
            Some(cone) => {
                let path = if case == Case::Fold {
                    folded_path.clear();
                    folded_path.extend_from_slice(path);
                    folded_path.make_ascii_lowercase();
                    folded_path.as_bstr()
                } else {
                    path
                };
                let parent = cone_directories.decision(parent_path(path), |path, parent| cone.decide(path, parent));
                cone.decide(path, parent) != ConeDecision::Excluded
            }
            None => search
                .pattern_matching_relative_path(path, Some(false), case)
                .map_or_else(
                    || {
                        pattern_directories.decision(parent_path(path), |path, parent| {
                            search
                                .pattern_matching_relative_path(path, Some(true), case)
                                .map_or(parent, |matched| !matched.pattern.is_negative())
                        })
                    },
                    |matched| !matched.pattern.is_negative(),
                ),
        };
        if !included {
            entry.flags.insert(Flags::SKIP_WORKTREE | Flags::EXTENDED);
        }
    }
    Ok(())
}

fn parent_path(path: &BStr) -> &BStr {
    path[..path.rfind_byte(b'/').unwrap_or(0)].as_bstr()
}

/// A repository-relative directory path stored as slash-separated Git path bytes without leading/trailing slashes.
/// An empty path denotes the repository root; bytes use the representation required by the matching mode.
type DirectoryPath = Vec<u8>;

/// Cache cumulative matching decisions along a single directory path, with `T::default()` as the root decision.
/// Reuse decisions for complete shared ancestors of the next path and recompute the remaining components.
/// Paths need not be sorted; storage is bounded by the longest path and deepest ancestor chain seen.
#[derive(Default)]
struct Directories<T> {
    /// Most recently evaluated directory path, in the byte representation used for matching.
    path: DirectoryPath,
    /// Exclusive byte offsets into `path` and their decisions, ordered from shallowest to deepest directory.
    /// Each decision incorporates its parent's decision; the final offset is `path.len()` for nonempty paths.
    decisions: Vec<(usize, T)>,
}

impl<T: Copy + Default> Directories<T> {
    /// `path` is a repository-relative directory path in Git's byte-oriented, slash-separated form,
    /// without leading or trailing slashes or `.`/`..` components; an empty path denotes the repository root.
    /// Pass an entry's parent directory, not its filename, and apply any case folding consistently before calling.
    /// Retain only complete shared ancestors. This also works when entries aren't sorted or case folding
    /// changes their order, and uses memory proportional to path depth rather than the number of directories.
    fn decision(&mut self, path: &BStr, mut decide: impl FnMut(&BStr, T) -> T) -> T {
        if self.path.as_bstr() == path {
            return self.decisions.last().map_or_else(T::default, |&(_, decision)| decision);
        }
        let common = self.path.iter().zip(path.iter()).take_while(|(a, b)| a == b).count();
        while self
            .decisions
            .last()
            .is_some_and(|&(end, _)| end > common || path.get(end).is_some_and(|&byte| byte != b'/'))
        {
            self.decisions.pop();
        }
        let (start, mut parent) = self
            .decisions
            .last()
            .map_or((0, T::default()), |&(end, decision)| (end + 1, decision));
        if start < path.len() {
            for end in path[start..].find_iter(b"/").map(|pos| start + pos).chain([path.len()]) {
                parent = decide(path[..end].as_bstr(), parent);
                self.decisions.push((end, parent));
            }
        }
        self.path.clear();
        self.path.extend_from_slice(path);
        parent
    }
}

/// Cumulative cone-matching state passed from a directory to its children.
/// Only `Excluded` skips an index entry; `Recursive` and `Excluded` propagate unchanged to all descendants.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum ConeDecision {
    /// A recursive selection includes this path and its entire subtree.
    Recursive,
    /// A parent selection includes immediate entries while requiring deeper paths to be selected separately.
    /// Also represents the repository root, whose immediate entries are always included in cone mode.
    #[default]
    Parent,
    /// This path is included as an immediate entry of a parent, but its directory contents are unselected.
    /// An immediately selected recursive child can still match; otherwise children become `Excluded`.
    Unselected,
    /// An unselected ancestor has pruned this path, making even explicitly selected descendants unreachable.
    Excluded,
}

/// Validated cone selections with literal directory paths extracted from patterns, such as `selected/nested`.
/// Pattern escapes are removed, and paths are ASCII-lowercased when matching is case-insensitive.
#[derive(Default)]
struct Cone {
    /// Directory paths selected recursively by `/dir/`, mapped to their remaining selection counts.
    /// Git permits duplicates: each `!/dir/*/` removes one selection, and zero-count paths are removed.
    recursive: HashMap<DirectoryPath, usize>,
    /// Directory paths made parent-only by `!/dir/*/`: immediate entries remain included, but deeper
    /// paths need separate selections. A path can also remain in `recursive` if it had duplicate selections.
    parents: HashSet<DirectoryPath>,
    /// Include every entry regardless of directory selections when the last root-wide pattern is `/*`, not `!/*/`.
    /// This permits skipping entry traversal, but only after all patterns have passed cone validation.
    full: bool,
}

impl Cone {
    /// Validate all parsed patterns in `search`, in order, and build their cone selections.
    /// `case` controls both matching parent exclusions to earlier selections and folding stored paths.
    ///
    /// Return `Some(cone)` only if every pattern and selection/exclusion sequence is valid, including
    /// an empty pattern list. Return `None` on any violation so the caller falls back to ordinary
    /// matching of the entire pattern list, rather than using partially built selections.
    fn from_search(search: &gix_ignore::Search, case: Case) -> Option<Self> {
        let mut cone = Self::default();
        for mapping in search.patterns.iter().flat_map(|list| &list.patterns) {
            let pattern = &mapping.pattern;
            let text = pattern.text.as_bytes();
            let negative = pattern.is_negative();
            let directory = pattern.mode.contains(Mode::MUST_BE_DIR);
            if pattern.mode.contains(Mode::ABSOLUTE) && text == b"*" && negative == directory {
                cone.full = !negative;
                continue;
            }
            if !pattern.mode.contains(Mode::ABSOLUTE)
                || text.is_empty()
                || !directory
                || text.windows(2).any(|pair| pair == b"**")
            {
                return None;
            }
            for (pos, &byte) in text.iter().enumerate() {
                if !matches!(byte, b'*' | b'?' | b'[' | b'\\')
                    || pos > 0 && text[pos - 1] == b'\\'
                    || byte == b'\\' && text.get(pos + 1).is_some_and(|next| b"*?[\\".contains(next))
                    || byte == b'*' && (pos == 0 || text[pos - 1] == b'/') && pos + 1 == text.len()
                {
                    continue;
                }
                return None;
            }
            if negative != text.ends_with(b"/*") {
                return None;
            }
            let mut path = DirectoryPath::with_capacity(text.len());
            let mut bytes = text.iter().copied();
            while let Some(byte) = bytes.next() {
                path.push(if byte == b'\\' { bytes.next()? } else { byte });
            }
            if path.len() > 1 && path.ends_with(b"/*") {
                path.truncate(path.len() - 2);
            }
            if case == Case::Fold {
                path.make_ascii_lowercase();
            }
            if negative {
                let count = cone.recursive.get_mut(&path)?;
                *count -= 1;
                if *count == 0 {
                    cone.recursive.remove(&path);
                }
                cone.parents.insert(path);
            } else {
                if cone.parents.contains(&path) {
                    return None;
                }
                *cone.recursive.entry(path).or_default() += 1;
            }
        }
        Some(cone)
    }

    fn decide(&self, path: &BStr, parent: ConeDecision) -> ConeDecision {
        match parent {
            ConeDecision::Recursive | ConeDecision::Excluded => parent,
            _ if self.recursive.contains_key(path.as_bytes()) => ConeDecision::Recursive,
            // Checkout prunes unmatched directories before reaching matching descendants.
            ConeDecision::Unselected => ConeDecision::Excluded,
            _ if self.parents.contains(path.as_bytes()) => ConeDecision::Parent,
            _ => ConeDecision::Unselected,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directory_decisions_reuse_only_complete_shared_ancestors() {
        let mut directories = Directories::<usize>::default();
        let mut visited = Vec::new();
        for path in ["a/b", "a/b", "a/b/c", "a/bb", "a", "ab/c", "", "a/b"] {
            directories.decision(path.as_bytes().as_bstr(), |path, parent| {
                visited.push(path.to_owned());
                parent + 1
            });
        }
        assert_eq!(
            visited,
            ["a", "a/b", "a/b/c", "a/bb", "ab", "ab/c", "a", "a/b"],
            "siblings reuse their ancestors, ascents retain parents, and similarly named directories don't share decisions"
        );
    }
}
