use crate::fs::cache::{state, State};
use crate::fs::PathOidMapping;
use bstr::{BStr, BString, ByteSlice};
use git_glob::pattern::Case;
use git_hash::oid;
use std::path::Path;

type AttributeMatchGroup = git_attributes::MatchGroup<git_attributes::Attributes>;
type IgnoreMatchGroup = git_attributes::MatchGroup<git_attributes::Ignore>;

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
    pub overrides: IgnoreMatchGroup,
    /// Ignore patterns that match the currently set director (in the stack).
    pub stack: IgnoreMatchGroup,
    /// Ignore patterns which aren't tied to the repository root, hence are global. They are consulted last.
    pub globals: IgnoreMatchGroup,
    ///  The name of the file to look for in directories.
    pub exclude_file_name_for_directories: BString,
}

impl Ignore {
    /// The `exclude_file_name_for_directories` is an optional override for the filename to use when checking per-directory
    /// ignore files within the repository, defaults to`.gitignore`.
    // TODO: more docs
    pub fn new(
        overrides: IgnoreMatchGroup,
        globals: IgnoreMatchGroup,
        exclude_file_name_for_directories: Option<&BStr>,
    ) -> Self {
        Ignore {
            overrides,
            globals,
            stack: Default::default(),
            exclude_file_name_for_directories: exclude_file_name_for_directories
                .map(ToOwned::to_owned)
                .unwrap_or_else(|| ".gitignore".into()),
        }
    }

    pub fn matching_exclude_pattern(
        &self,
        relative_path: &BStr,
        is_dir: Option<bool>,
        case: git_glob::pattern::Case,
    ) -> Option<git_attributes::Match<'_, ()>> {
        [&self.overrides, &self.stack, &self.globals]
            .iter()
            .find_map(|group| group.pattern_matching_relative_path(relative_path.as_ref(), is_dir, case))
    }

    pub fn push_directory<Find, E>(
        &mut self,
        root: &Path,
        dir: &Path,
        buf: &mut Vec<u8>,
        attribute_files_in_index: &[PathOidMapping<'_>],
        mut find: Find,
    ) -> std::io::Result<()>
    where
        Find: for<'b> FnMut(&oid, &'b mut Vec<u8>) -> Result<git_object::BlobRef<'b>, E>,
        E: std::error::Error + Send + Sync + 'static,
    {
        let ignore_path_relative =
            git_features::path::convert::to_unix_separators(git_features::path::into_bytes_or_panic_on_windows(
                dir.strip_prefix(root).expect("dir in root").join(".gitignore"),
            ));
        let ignore_file_in_index =
            attribute_files_in_index.binary_search_by(|t| t.0.cmp(ignore_path_relative.as_bstr()));
        let follow_symlinks = ignore_file_in_index.is_err();
        if !self
            .stack
            .add_patterns_file(dir.join(".gitignore"), follow_symlinks, Some(root), buf)?
        {
            match ignore_file_in_index {
                Ok(idx) => {
                    let ignore_blob = find(&attribute_files_in_index[idx].1, buf)
                        .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
                    let ignore_path =
                        git_features::path::from_byte_vec_or_panic_on_windows(ignore_path_relative.into_owned());
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
    pub fn for_checkout(unlink_on_collision: bool, attributes: state::Attributes) -> Self {
        State::CreateDirectoryAndAttributesStack {
            unlink_on_collision,
            #[cfg(debug_assertions)]
            test_mkdir_calls: 0,
            attributes,
        }
    }

    /// Configure a state for adding files.
    pub fn for_add(attributes: state::Attributes, ignore: state::Ignore) -> Self {
        State::AttributesAndIgnoreStack { attributes, ignore }
    }

    /// Configure a state for status retrieval.
    pub fn for_status(ignore: state::Ignore) -> Self {
        State::IgnoreStack(ignore)
    }
}

impl State {
    /// Returns a vec of tuples of relative index paths along with the best usable OID for either ignore, attribute files or both.
    ///
    /// - ignores entries which aren't blobs
    /// - ignores ignore entries which are not skip-worktree
    /// - within merges, picks 'our' stage both for ignore and attribute files.
    pub fn build_attribute_list<'paths>(
        &self,
        index: &git_index::State,
        paths: &'paths git_index::PathStorage,
        case: git_glob::pattern::Case,
    ) -> Vec<PathOidMapping<'paths>> {
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
                if entry.mode == git_index::entry::Mode::FILE && (entry.stage() == 0 || entry.stage() == 2) {
                    let basename = path
                        .rfind_byte(b'/')
                        .map(|pos| path[pos + 1..].as_bstr())
                        .unwrap_or(path);
                    let is_ignore = names.iter().find_map(|t| {
                        match case {
                            Case::Sensitive => basename == t.0,
                            Case::Fold => basename.eq_ignore_ascii_case(t.0),
                        }
                        .then(|| t.1)
                    })?;
                    // See https://github.com/git/git/blob/master/dir.c#L912:L912
                    if is_ignore && !entry.flags.contains(git_index::entry::Flags::SKIP_WORKTREE) {
                        return None;
                    }
                    Some((path, entry.id))
                } else {
                    None
                }
            })
            .collect()
    }

    pub(crate) fn ignore_or_panic(&self) -> &state::Ignore {
        match self {
            State::IgnoreStack(v) => v,
            State::AttributesAndIgnoreStack { ignore, .. } => ignore,
            State::CreateDirectoryAndAttributesStack { .. } => {
                unreachable!("BUG: must not try to check excludes without it being setup")
            }
        }
    }
}
