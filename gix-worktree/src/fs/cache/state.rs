use std::path::Path;

use bstr::{BStr, BString, ByteSlice};
use gix_glob::pattern::Case;
use gix_hash::oid;

use crate::fs::{cache::State, PathOidMapping};

type AttributeMatchGroup = gix_attributes::MatchGroup<gix_attributes::Attributes>;
type IgnoreMatchGroup = gix_attributes::MatchGroup<gix_attributes::Ignore>;

/// State related to attributes associated with files in the repository.
#[derive(Default, Clone)]
#[allow(unused)]
pub struct Attributes {
    /// Attribute patterns that match the currently set directory (in the stack).
    pub stack: AttributeMatchGroup,
    /// Attribute patterns which aren't tied to the repository root, hence are global. They are consulted last.
    pub globals: AttributeMatchGroup,
}

/// State related to the exclusion of files.
#[derive(Default, Clone)]
#[allow(unused)]
pub struct Ignore {
    /// Ignore patterns passed as overrides to everything else, typically passed on the command-line and the first patterns to
    /// be consulted.
    overrides: IgnoreMatchGroup,
    /// Ignore patterns that match the currently set director (in the stack), which is pushed and popped as needed.
    stack: IgnoreMatchGroup,
    /// Ignore patterns which aren't tied to the repository root, hence are global. They are consulted last.
    globals: IgnoreMatchGroup,
    /// A matching stack of pattern indices which is empty if we have just been initialized to indicate that the
    /// currently set directory had a pattern matched. Note that this one could be negated.
    /// (index into match groups, index into list of pattern lists, index into pattern list)
    matched_directory_patterns_stack: Vec<Option<(usize, usize, usize)>>,
    ///  The name of the file to look for in directories.
    exclude_file_name_for_directories: BString,
    /// The case to use when matching directories as they are pushed onto the stack. We run them against the exclude engine
    /// to know if an entire path can be ignored as a parent directory is ignored.
    case: Case,
}

impl Ignore {
    /// The `exclude_file_name_for_directories` is an optional override for the filename to use when checking per-directory
    /// ignore files within the repository, defaults to`.gitignore`.
    // TODO: more docs
    pub fn new(
        overrides: IgnoreMatchGroup,
        globals: IgnoreMatchGroup,
        exclude_file_name_for_directories: Option<&BStr>,
        case: Case,
    ) -> Self {
        Ignore {
            case,
            overrides,
            globals,
            stack: Default::default(),
            matched_directory_patterns_stack: Vec::with_capacity(6),
            exclude_file_name_for_directories: exclude_file_name_for_directories
                .map(ToOwned::to_owned)
                .unwrap_or_else(|| ".gitignore".into()),
        }
    }
}

impl Ignore {
    pub(crate) fn pop_directory(&mut self) {
        self.matched_directory_patterns_stack.pop().expect("something to pop");
        self.stack.patterns.pop().expect("something to pop");
    }
    /// The match groups from lowest priority to highest.
    pub(crate) fn match_groups(&self) -> [&IgnoreMatchGroup; 3] {
        [&self.globals, &self.stack, &self.overrides]
    }

    pub(crate) fn matching_exclude_pattern(
        &self,
        relative_path: &BStr,
        is_dir: Option<bool>,
        case: Case,
    ) -> Option<gix_attributes::Match<'_, ()>> {
        let groups = self.match_groups();
        let mut dir_match = None;
        if let Some((source, mapping)) = self
            .matched_directory_patterns_stack
            .iter()
            .rev()
            .filter_map(|v| *v)
            .map(|(gidx, plidx, pidx)| {
                let list = &groups[gidx].patterns[plidx];
                (list.source.as_deref(), &list.patterns[pidx])
            })
            .next()
        {
            let match_ = gix_attributes::Match {
                pattern: &mapping.pattern,
                value: &mapping.value,
                sequence_number: mapping.sequence_number,
                source,
            };
            if mapping.pattern.is_negative() {
                dir_match = Some(match_);
            } else {
                // Note that returning here is wrong if this pattern _was_ preceded by a negative pattern that
                // didn't match the directory, but would match now.
                // Git does it similarly so we do too even though it's incorrect.
                // To fix this, one would probably keep track of whether there was a preceding negative pattern, and
                // if so we check the path in full and only use the dir match if there was no match, similar to the negative
                // case above whose fix fortunately won't change the overall result.
                return match_.into();
            }
        }
        groups
            .iter()
            .rev()
            .find_map(|group| group.pattern_matching_relative_path(relative_path.as_bytes(), is_dir, case))
            .or(dir_match)
    }

    /// Like `matching_exclude_pattern()` but without checking if the current directory is excluded.
    /// It returns a triple-index into our data structure from which a match can be reconstructed.
    pub(crate) fn matching_exclude_pattern_no_dir(
        &self,
        relative_path: &BStr,
        is_dir: Option<bool>,
        case: Case,
    ) -> Option<(usize, usize, usize)> {
        let groups = self.match_groups();
        groups.iter().enumerate().rev().find_map(|(gidx, group)| {
            let basename_pos = relative_path.rfind(b"/").map(|p| p + 1);
            group
                .patterns
                .iter()
                .enumerate()
                .rev()
                .find_map(|(plidx, pl)| {
                    pl.pattern_idx_matching_relative_path(relative_path, basename_pos, is_dir, case)
                        .map(|idx| (plidx, idx))
                })
                .map(|(plidx, pidx)| (gidx, plidx, pidx))
        })
    }

    pub(crate) fn push_directory<Find, E>(
        &mut self,
        root: &Path,
        dir: &Path,
        buf: &mut Vec<u8>,
        attribute_files_in_index: &[PathOidMapping],
        mut find: Find,
    ) -> std::io::Result<()>
    where
        Find: for<'b> FnMut(&oid, &'b mut Vec<u8>) -> Result<gix_object::BlobRef<'b>, E>,
        E: std::error::Error + Send + Sync + 'static,
    {
        let rela_dir = dir.strip_prefix(root).expect("dir in root");
        self.matched_directory_patterns_stack
            .push(self.matching_exclude_pattern_no_dir(gix_path::into_bstr(rela_dir).as_ref(), Some(true), self.case));

        let ignore_path_relative = rela_dir.join(".gitignore");
        let ignore_path_relative = gix_path::to_unix_separators_on_windows(gix_path::into_bstr(ignore_path_relative));
        let ignore_file_in_index =
            attribute_files_in_index.binary_search_by(|t| t.0.as_bstr().cmp(ignore_path_relative.as_ref()));
        let follow_symlinks = ignore_file_in_index.is_err();
        if !self
            .stack
            .add_patterns_file(dir.join(".gitignore"), follow_symlinks, Some(root), buf)?
        {
            match ignore_file_in_index {
                Ok(idx) => {
                    let ignore_blob = find(&attribute_files_in_index[idx].1, buf)
                        .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
                    let ignore_path = gix_path::from_bstring(ignore_path_relative.into_owned());
                    self.stack
                        .add_patterns_buffer(ignore_blob.data, ignore_path, Some(root));
                }
                Err(_) => {
                    // Need one stack level per component so push and pop matches.
                    self.stack.patterns.push(Default::default())
                }
            }
        }
        Ok(())
    }
}

impl Attributes {
    /// Create a new instance from an attribute match group that represents `globals`.
    ///
    /// A stack of attributes will be applied on top of it later.
    pub fn new(globals: AttributeMatchGroup) -> Self {
        Attributes {
            globals,
            stack: Default::default(),
        }
    }
}

impl From<AttributeMatchGroup> for Attributes {
    fn from(group: AttributeMatchGroup) -> Self {
        Attributes::new(group)
    }
}

impl State {
    /// Configure a state to be suitable for checking out files.
    pub fn for_checkout(unlink_on_collision: bool, attributes: Attributes) -> Self {
        State::CreateDirectoryAndAttributesStack {
            unlink_on_collision,
            #[cfg(debug_assertions)]
            test_mkdir_calls: 0,
            attributes,
        }
    }

    /// Configure a state for adding files.
    pub fn for_add(attributes: Attributes, ignore: Ignore) -> Self {
        State::AttributesAndIgnoreStack { attributes, ignore }
    }

    /// Configure a state for status retrieval.
    pub fn for_status(ignore: Ignore) -> Self {
        State::IgnoreStack(ignore)
    }
}

impl State {
    /// Returns a vec of tuples of relative index paths along with the best usable OID for either ignore, attribute files or both.
    ///
    /// - ignores entries which aren't blobs
    /// - ignores ignore entries which are not skip-worktree
    /// - within merges, picks 'our' stage both for ignore and attribute files.
    pub fn build_attribute_list(
        &self,
        index: &gix_index::State,
        paths: &gix_index::PathStorageRef,
        case: Case,
    ) -> Vec<PathOidMapping> {
        let a1_backing;
        let a2_backing;
        let names = match self {
            State::IgnoreStack(v) => {
                a1_backing = [(v.exclude_file_name_for_directories.as_bytes().as_bstr(), true)];
                a1_backing.as_ref()
            }
            State::AttributesAndIgnoreStack { ignore, .. } => {
                a2_backing = [
                    (ignore.exclude_file_name_for_directories.as_bytes().as_bstr(), true),
                    (".gitattributes".into(), false),
                ];
                a2_backing.as_ref()
            }
            State::CreateDirectoryAndAttributesStack { .. } => {
                a1_backing = [(".gitattributes".into(), true)];
                a1_backing.as_ref()
            }
        };

        index
            .entries()
            .iter()
            .filter_map(move |entry| {
                let path = entry.path_in(paths);

                // Stage 0 means there is no merge going on, stage 2 means it's 'our' side of the merge, but then
                // there won't be a stage 0.
                if entry.mode == gix_index::entry::Mode::FILE && (entry.stage() == 0 || entry.stage() == 2) {
                    let basename = path
                        .rfind_byte(b'/')
                        .map(|pos| path[pos + 1..].as_bstr())
                        .unwrap_or(path);
                    let is_ignore = names.iter().find_map(|t| {
                        match case {
                            Case::Sensitive => basename == t.0,
                            Case::Fold => basename.eq_ignore_ascii_case(t.0),
                        }
                        .then_some(t.1)
                    })?;
                    // See https://github.com/git/git/blob/master/dir.c#L912:L912
                    if is_ignore && !entry.flags.contains(gix_index::entry::Flags::SKIP_WORKTREE) {
                        return None;
                    }
                    Some((path.to_owned(), entry.id))
                } else {
                    None
                }
            })
            .collect()
    }

    pub(crate) fn ignore_or_panic(&self) -> &Ignore {
        match self {
            State::IgnoreStack(v) => v,
            State::AttributesAndIgnoreStack { ignore, .. } => ignore,
            State::CreateDirectoryAndAttributesStack { .. } => {
                unreachable!("BUG: must not try to check excludes without it being setup")
            }
        }
    }
}
