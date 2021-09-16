use git_repository as git;

/// A head reference will all commits that are 'governed' by it, that is are in its exclusive ancestry.
pub struct HistorySegment<'a> {
    pub _head: git::refs::Reference,
    /// only relevant history items, that is those that change code in the respective crate.
    pub history: Vec<&'a HistoryItem>,
}

pub struct HistoryItem {
    pub id: git::hash::ObjectId,
    pub _message: git::bstr::BString,
    pub tree_data: Vec<u8>,
}

pub struct History {
    pub head: git_repository::refs::Reference,
    pub items: Vec<HistoryItem>,
}
