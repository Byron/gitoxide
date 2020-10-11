use crate::{
    file::{self, Commit, File},
    graph::{self, Graph},
};
use git_object::borrowed;

/// Access
impl Graph {
    pub fn commit_at(&self, pos: graph::Position) -> Commit<'_> {
        let r = self.lookup_by_pos(pos);
        r.file.commit_at(r.pos)
    }

    pub fn commit_by_id(&self, id: borrowed::Id<'_>) -> Option<Commit<'_>> {
        let r = self.lookup_by_id(id)?;
        Some(r.file.commit_at(r.file_pos))
    }

    pub fn id_at(&self, pos: graph::Position) -> borrowed::Id<'_> {
        let r = self.lookup_by_pos(pos);
        r.file.id_at(r.pos)
    }

    /// Iterate over commits in unsorted order.
    pub fn iter_commits(&self) -> impl Iterator<Item = Commit<'_>> {
        self.files.iter().flat_map(|file| file.iter_commits())
    }

    /// Iterate over commit IDs in unsorted order.
    pub fn iter_ids(&self) -> impl Iterator<Item = borrowed::Id<'_>> {
        self.files.iter().flat_map(|file| file.iter_ids())
    }

    pub fn lookup(&self, id: borrowed::Id<'_>) -> Option<graph::Position> {
        Some(self.lookup_by_id(id)?.graph_pos)
    }

    pub fn num_commits(&self) -> u32 {
        self.files.iter().map(|f| f.num_commits()).sum()
    }
}

/// Access fundamentals
impl Graph {
    pub(crate) fn lookup_by_id(&self, id: borrowed::Id<'_>) -> Option<LookupByIdResult<'_>> {
        let mut current_file_start = 0;
        for file in &self.files {
            if let Some(lex_pos) = file.lookup(id) {
                return Some(LookupByIdResult {
                    file,
                    file_pos: lex_pos,
                    graph_pos: graph::Position(current_file_start + lex_pos.0),
                });
            }
            current_file_start += file.num_commits();
        }
        None
    }

    pub(crate) fn lookup_by_pos(&self, pos: graph::Position) -> LookupByPositionResult<'_> {
        let mut remaining = pos.0;
        for (file_index, file) in self.files.iter().enumerate() {
            match remaining.checked_sub(file.num_commits()) {
                Some(v) => remaining = v,
                None => {
                    return LookupByPositionResult {
                        file,
                        file_index,
                        pos: file::Position(remaining),
                    }
                }
            }
        }
        panic!("graph position too large: {}", pos.0);
    }
}

#[derive(Clone)]
pub(crate) struct LookupByIdResult<'a> {
    pub file: &'a File,
    pub graph_pos: graph::Position,
    pub file_pos: file::Position,
}

#[derive(Clone)]
pub(crate) struct LookupByPositionResult<'a> {
    pub file: &'a File,
    pub file_index: usize,
    pub pos: file::Position,
}
