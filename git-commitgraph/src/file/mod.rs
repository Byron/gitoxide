use filebuffer::FileBuffer;
use git_object::SHA1_SIZE;
use std::ops::Range;
use std::path::PathBuf;

mod access;
pub mod commit;
mod init;

pub use init::Error;
use std::fmt::{Display, Formatter};

const COMMIT_DATA_ENTRY_SIZE: usize = SHA1_SIZE + 16;
const FAN_LEN: usize = 256;
const SIGNATURE: &[u8] = b"CGPH";

/// A single commit-graph file.
///
/// All operations on a `File` are local to that graph file. Since a commit graph can span multiple
/// files, all interesting graph operations belong on `Graph`.
pub struct File {
    base_graph_count: u8,
    base_graphs_list_offset: Option<usize>,
    commit_data_offset: usize,
    data: FileBuffer,
    extra_edges_list_range: Option<Range<usize>>,
    fan: [u32; FAN_LEN],
    oid_lookup_offset: usize,
    path: PathBuf,
}

/// The position of a given commit within a graph file, starting at 0.
///
/// Commits within a graph file are sorted in lexicographical order by OID; a commit's lex position
/// is its position in this ordering. If a commit graph spans multiple files, each file's commits
/// start at lex position 0, so lex position is unique across a single file but is not unique across
/// the whole commit graph. Each commit also has a graph position (`GraphPosition`), which is unique
/// across the whole commit graph. In order to avoid accidentally mixing lex positions with graph
/// positions, distinct types are used for each.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LexPosition(pub u32);

impl Display for LexPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
