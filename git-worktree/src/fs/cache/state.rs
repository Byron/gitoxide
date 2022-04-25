use crate::fs::cache::{state, State};
use bstr::{BStr, BString, ByteSlice};
use git_glob::pattern::Case;
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

    pub fn push(&mut self, root: &Path, dir: &Path, buf: &mut Vec<u8>) -> std::io::Result<()> {
        let follow_symlinks = true;
        if !self
            .stack
            .add_patterns_file(dir.join(".gitignore"), follow_symlinks, Some(root), buf)?
        {
            // Need one stack level per component so push and pop matches.
            self.stack.patterns.push(Default::default());
        }
        // TODO: from index
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
    pub(crate) fn build_attribute_list<'a>(
        &self,
        index: &'a git_index::State,
        case: git_glob::pattern::Case,
    ) -> Vec<(&'a BStr, git_hash::ObjectId)> {
        let a1_backing;
        let a2_backing;
        let names = match self {
            State::IgnoreStack(v) => {
                a1_backing = [(v.exclude_file_name_for_directories.as_bytes().as_bstr(), true)];
                a1_backing.as_slice()
            }
            State::AttributesAndIgnoreStack { ignore, .. } => {
                a2_backing = [
                    (ignore.exclude_file_name_for_directories.as_bytes().as_bstr(), true),
                    (".gitattributes".into(), false),
                ];
                a2_backing.as_slice()
            }
            State::CreateDirectoryAndAttributesStack { .. } => {
                a1_backing = [(".gitattributes".into(), true)];
                a1_backing.as_slice()
            }
        };

        index
            .entries()
            .iter()
            .filter_map(move |entry| {
                let path = entry.path(index);

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
