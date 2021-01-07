use crate::{
    file::{self, File},
    graph,
};
use byteorder::{BigEndian, ByteOrder};
use git_hash::SIZE_OF_SHA1_DIGEST as SHA1_SIZE;
use git_object::{borrowed, owned};
use std::{
    convert::{TryFrom, TryInto},
    fmt::{Debug, Formatter},
    slice::Chunks,
};

/// The error used in the [`file::commit`][self] module.
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error("commit {0}'s extra edges overflows the commit-graph file's extra edges list")]
    ExtraEdgesListOverflow(owned::Id),
    #[error("commit {0}'s first parent is an extra edge index, which is invalid")]
    FirstParentIsExtraEdgeIndex(owned::Id),
    #[error("commit {0} has extra edges, but commit-graph file has no extra edges list")]
    MissingExtraEdgesList(owned::Id),
    #[error("commit {0} has a second parent but not a first parent")]
    SecondParentWithoutFirstParent(owned::Id),
}

// Note that git's commit-graph-format.txt as of v2.28.0 gives an incorrect value 0x0700_0000 for
// NO_PARENT. Fixed in https://github.com/git/git/commit/4d515253afcef985e94400adbfed7044959f9121 .
const NO_PARENT: u32 = 0x7000_0000;
const EXTENDED_EDGES_MASK: u32 = 0x8000_0000;

/// A commit as stored in a [`File`].
pub struct Commit<'a> {
    file: &'a File,
    pos: file::Position,
    // We can parse the below fields lazily if needed.
    commit_timestamp: u64,
    generation: u32,
    parent1: ParentEdge,
    parent2: ParentEdge,
    root_tree_id: borrowed::Id<'a>,
}

impl<'a> Commit<'a> {
    pub(crate) fn new(file: &'a File, pos: file::Position) -> Self {
        let bytes = file.commit_data_bytes(pos);
        Commit {
            file,
            pos,
            root_tree_id: borrowed::Id::try_from(&bytes[..SHA1_SIZE]).expect("20 bytes SHA1 to be alright"),
            parent1: ParentEdge::from_raw(BigEndian::read_u32(&bytes[SHA1_SIZE..SHA1_SIZE + 4])),
            parent2: ParentEdge::from_raw(BigEndian::read_u32(&bytes[SHA1_SIZE + 4..SHA1_SIZE + 8])),
            generation: BigEndian::read_u32(&bytes[SHA1_SIZE + 8..SHA1_SIZE + 12]) >> 2,
            commit_timestamp: BigEndian::read_u64(&bytes[SHA1_SIZE + 8..SHA1_SIZE + 16]) & 0x0003_ffff_ffff,
        }
    }

    /// Returns the committer timestamp of this commit.
    ///
    /// The value is the number of seconds since 1970-01-01 00:00:00 UTC.
    pub fn committer_timestamp(&self) -> u64 {
        self.commit_timestamp
    }

    /// Returns the generation number of this commit.
    ///
    /// Commits without parents have generation number 1. Commits with parents have a generation
    /// number that is the max of their parents' generation numbers + 1.
    pub fn generation(&self) -> u32 {
        self.generation
    }

    /// Returns an iterator over the parent positions for lookup in the owning [Graph][crate::Graph].
    pub fn iter_parents(&'a self) -> impl Iterator<Item = Result<graph::Position, Error>> + 'a {
        // I didn't find a combinator approach that a) was as strict as ParentIterator, b) supported
        // fuse-after-first-error behavior, and b) was significantly shorter or more understandable
        // than ParentIterator. So here we are.
        ParentIterator {
            commit_data: self,
            state: ParentIteratorState::First,
        }
    }

    /// Returns the hash of this commit.
    pub fn id(&self) -> borrowed::Id<'a> {
        self.file.id_at(self.pos)
    }

    /// Returns the first parent of this commit.
    pub fn parent1(&self) -> Result<Option<graph::Position>, Error> {
        self.iter_parents().next().transpose()
    }

    /// Returns the position at which this commit is stored in the parent [File].
    pub fn position(&self) -> file::Position {
        self.pos
    }

    /// Return the hash of the tree this commit points to.
    pub fn root_tree_id(&self) -> borrowed::Id<'a> {
        self.root_tree_id
    }
}

impl<'a> Debug for Commit<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Commit {{ id: {}, lex_pos: {}, generation: {}, root_tree_id: {}, parent1: {:?}, parent2: {:?} }}",
            self.id(),
            self.pos,
            self.generation(),
            self.root_tree_id(),
            self.parent1,
            self.parent2,
        )
    }
}

impl<'a> Eq for Commit<'a> {}

impl<'a> PartialEq for Commit<'a> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.file, other.file) && self.pos == other.pos
    }
}

/// An iterator over parents of a [`Commit`].
pub struct ParentIterator<'a> {
    commit_data: &'a Commit<'a>,
    state: ParentIteratorState<'a>,
}

impl<'a> Iterator for ParentIterator<'a> {
    type Item = Result<graph::Position, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let state = std::mem::replace(&mut self.state, ParentIteratorState::Exhausted);
        match state {
            ParentIteratorState::First => match self.commit_data.parent1 {
                ParentEdge::None => match self.commit_data.parent2 {
                    ParentEdge::None => None,
                    _ => Some(Err(Error::SecondParentWithoutFirstParent(self.commit_data.id().into()))),
                },
                ParentEdge::GraphPosition(pos) => {
                    self.state = ParentIteratorState::Second;
                    Some(Ok(pos))
                }
                ParentEdge::ExtraEdgeIndex(_) => {
                    Some(Err(Error::FirstParentIsExtraEdgeIndex(self.commit_data.id().into())))
                }
            },
            ParentIteratorState::Second => match self.commit_data.parent2 {
                ParentEdge::None => None,
                ParentEdge::GraphPosition(pos) => Some(Ok(pos)),
                ParentEdge::ExtraEdgeIndex(extra_edge_index) => {
                    if let Some(extra_edges_list) = self.commit_data.file.extra_edges_data() {
                        let start_offset: usize = extra_edge_index
                            .try_into()
                            .expect("an architecture able to hold 32 bits of integer");
                        let start_offset = start_offset
                            .checked_mul(4)
                            .expect("an extended edge index small enough to fit in usize");
                        if let Some(tail) = extra_edges_list.get(start_offset..) {
                            self.state = ParentIteratorState::Extra(tail.chunks(4));
                            // This recursive call is what blocks me from replacing ParentIterator
                            // with a std::iter::from_fn closure.
                            self.next()
                        } else {
                            Some(Err(Error::ExtraEdgesListOverflow(self.commit_data.id().into())))
                        }
                    } else {
                        Some(Err(Error::MissingExtraEdgesList(self.commit_data.id().into())))
                    }
                }
            },
            ParentIteratorState::Extra(mut chunks) => {
                if let Some(chunk) = chunks.next() {
                    let extra_edge = BigEndian::read_u32(chunk);
                    match ExtraEdge::from_raw(extra_edge) {
                        ExtraEdge::Internal(pos) => {
                            self.state = ParentIteratorState::Extra(chunks);
                            Some(Ok(pos))
                        }
                        ExtraEdge::Last(pos) => Some(Ok(pos)),
                    }
                } else {
                    Some(Err(Error::ExtraEdgesListOverflow(self.commit_data.id().into())))
                }
            }
            ParentIteratorState::Exhausted => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match (&self.state, self.commit_data.parent1, self.commit_data.parent2) {
            (ParentIteratorState::First, ParentEdge::None, ParentEdge::None) => (0, Some(0)),
            (ParentIteratorState::First, ParentEdge::None, _) => (1, Some(1)),
            (ParentIteratorState::First, ParentEdge::GraphPosition(_), ParentEdge::None) => (1, Some(1)),
            (ParentIteratorState::First, ParentEdge::GraphPosition(_), ParentEdge::GraphPosition(_)) => (2, Some(2)),
            (ParentIteratorState::First, ParentEdge::GraphPosition(_), ParentEdge::ExtraEdgeIndex(_)) => (3, None),
            (ParentIteratorState::First, ParentEdge::ExtraEdgeIndex(_), _) => (1, Some(1)),
            (ParentIteratorState::Second, _, ParentEdge::None) => (0, Some(0)),
            (ParentIteratorState::Second, _, ParentEdge::GraphPosition(_)) => (1, Some(1)),
            (ParentIteratorState::Second, _, ParentEdge::ExtraEdgeIndex(_)) => (2, None),
            (ParentIteratorState::Extra(_), _, _) => (1, None),
            (ParentIteratorState::Exhausted, _, _) => (0, Some(0)),
        }
    }
}

#[derive(Debug)]
enum ParentIteratorState<'a> {
    First,
    Second,
    Extra(Chunks<'a, u8>),
    Exhausted,
}

#[derive(Clone, Copy, Debug)]
enum ParentEdge {
    None,
    GraphPosition(graph::Position),
    ExtraEdgeIndex(u32),
}

impl ParentEdge {
    pub fn from_raw(raw: u32) -> ParentEdge {
        if raw == NO_PARENT {
            return ParentEdge::None;
        }
        if raw & EXTENDED_EDGES_MASK != 0 {
            ParentEdge::ExtraEdgeIndex(raw & !EXTENDED_EDGES_MASK)
        } else {
            ParentEdge::GraphPosition(graph::Position(raw))
        }
    }
}

const LAST_EXTENDED_EDGE_MASK: u32 = 0x8000_0000;

enum ExtraEdge {
    Internal(graph::Position),
    Last(graph::Position),
}

impl ExtraEdge {
    pub fn from_raw(raw: u32) -> Self {
        if raw & LAST_EXTENDED_EDGE_MASK != 0 {
            Self::Last(graph::Position(raw & !LAST_EXTENDED_EDGE_MASK))
        } else {
            Self::Internal(graph::Position(raw))
        }
    }
}
