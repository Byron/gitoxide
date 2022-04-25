use crate::fs::cache::{state, State};
use bstr::{BStr, BString};
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
                .unwrap_or(".gitignore".into()),
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
