use std::path::PathBuf;

use gix_glob::pattern::Case;

use crate::cache::state::{AttributeMatchGroup, Attributes};

/// Decide where to read `.gitattributes` files from.
#[derive(Default, Debug, Clone, Copy)]
pub enum Source {
    /// Retrieve attribute files from an attribute list, see
    /// [State::attribute_list_from_index()][crate::cache::State::attribute_list_from_index()].
    ///
    /// The attribute list is typically produced from an index. If a tree should be the source, build an attribute list
    /// from a tree instead.
    #[default]
    AttributeList,
    /// Read from an attribute list and if not present, read from the worktree.
    AttributeListThenWorktree,
    /// Read from the worktree and if not present, read from the attribute list.
    WorktreeThenAttributeList,
}

/// Initialization
impl Attributes {
    /// Create a new instance from an attribute match group that represents `globals`.
    /// `globals` contribute first and consist of all globally available, static files.
    pub fn new(
        globals: AttributeMatchGroup,
        info_attributes: Option<PathBuf>,
        case: Case,
        source: Source,
        collection: gix_attributes::search::MetadataCollection,
    ) -> Self {
        Attributes {
            globals,
            stack: Default::default(),
            info_attributes,
            case,
            source,
            collection,
        }
    }
}

/// Builder
impl Attributes {
    /// Set the case to use when matching attributes to paths.
    pub fn with_case(mut self, case: gix_glob::pattern::Case) -> Self {
        self.case = case;
        self
    }
}
