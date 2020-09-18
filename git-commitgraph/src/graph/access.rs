use crate::graph::GraphPosition;
use crate::graph_file::{GraphFile, LexPosition};
use crate::{CommitData, Graph};
use git_object::borrowed;

impl Graph {
    pub fn commit_at(&self, pos: GraphPosition) -> CommitData<'_> {
        let r = self.lookup_by_pos(pos);
        r.file.commit_at(r.lex_pos)
    }

    pub fn commit_by_id(&self, id: borrowed::Id<'_>) -> Option<CommitData<'_>> {
        let r = self.lookup_by_id(id)?;
        Some(r.file.commit_at(r.lex_pos))
    }

    pub fn id_at(&self, pos: GraphPosition) -> borrowed::Id<'_> {
        let r = self.lookup_by_pos(pos);
        r.file.id_at(r.lex_pos)
    }

    /// Iterate over commits in unsorted order.
    pub fn iter_commits(&self) -> impl Iterator<Item = CommitData<'_>> {
        self.files.iter().flat_map(|file| file.iter_commits())
    }

    /// Iterate over commit IDs in unsorted order.
    pub fn iter_ids(&self) -> impl Iterator<Item = borrowed::Id<'_>> {
        self.files.iter().flat_map(|file| file.iter_ids())
    }

    pub fn lookup(&self, id: borrowed::Id<'_>) -> Option<GraphPosition> {
        Some(self.lookup_by_id(id)?.graph_pos)
    }

    pub fn num_commits(&self) -> u32 {
        self.files.iter().map(|f| f.num_commits()).sum()
    }
}

impl Graph {
    fn lookup_by_id(&self, id: borrowed::Id<'_>) -> Option<LookupByIdResult<'_>> {
        let mut current_file_start = 0;
        for file in self.files.iter() {
            if let Some(lex_pos) = file.lookup(id) {
                return Some(LookupByIdResult {
                    file,
                    lex_pos,
                    graph_pos: GraphPosition(current_file_start + lex_pos.0),
                });
            }
            current_file_start += file.num_commits();
        }
        None
    }

    fn lookup_by_pos(&self, pos: GraphPosition) -> LookupByPositionResult<'_> {
        let mut remaining = pos.0;
        for file in self.files.iter() {
            match remaining.checked_sub(file.num_commits()) {
                Some(v) => remaining = v,
                None => {
                    return LookupByPositionResult {
                        file,
                        lex_pos: LexPosition(remaining),
                    }
                }
            }
        }
        panic!("graph position too large: {}", pos.0);
    }
}

struct LookupByIdResult<'a> {
    pub file: &'a GraphFile,
    pub graph_pos: GraphPosition,
    pub lex_pos: LexPosition,
}

struct LookupByPositionResult<'a> {
    pub file: &'a GraphFile,
    pub lex_pos: LexPosition,
}
