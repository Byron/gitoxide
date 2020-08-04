use crate::{pack, pack::index::access::PackOffset};
use git_features::progress::Progress;
use petgraph::{
    graph::{DiGraph, NodeIndex},
    Direction,
};
use quick_error::quick_error;
use std::{
    collections::BTreeMap,
    fs,
    io::{self, BufRead, Read},
    time::Instant,
};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: io::Error, msg: &'static str) {
            display("{}", msg)
            source(err)
        }
        Header(err: pack::data::parse::Error) {
            source(err)
            from()
        }
    }
}

// TODO: replace with Tree (non-public), keep tests
pub struct DeltaTree {
    inner: DiGraph<PackOffset, (), u32>, // u32 = max amount of objects in pack
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
pub struct Node {
    pub pack_offset: PackOffset,
    index: NodeIndex<u32>,
}

impl Node {}

/// Access
impl DeltaTree {
    pub fn bases(&self) -> impl Iterator<Item = Node> + '_ {
        self.inner
            .node_indices()
            .filter(move |idx| {
                self.inner
                    .neighbors_directed(*idx, Direction::Incoming)
                    .next()
                    .is_none()
            })
            .map(move |idx| Node {
                index: idx,
                pack_offset: self
                    .inner
                    .node_weight(idx)
                    .copied()
                    .expect("index we iterate to be present in graph"),
            })
    }

    pub fn node_count(&self) -> usize {
        self.inner.node_count()
    }

    pub fn children(&self, n: Node, out: &mut Vec<Node>) {
        out.clear();
        out.extend(self.inner.neighbors_directed(n.index, Direction::Outgoing).map(|idx| {
            Node {
                index: idx,
                pack_offset: self
                    .inner
                    .node_weight(idx)
                    .copied()
                    .expect("index we iterate to be present in graph"),
            }
        }))
    }
}

const PACK_HEADER_LEN: usize = 12;

/// Initialization
impl DeltaTree {
    /// The sort order is ascending. The given packfile path must match the provided offsets.
    pub fn from_offsets_in_pack(
        offsets: impl Iterator<Item = PackOffset>,
        pack_path: impl AsRef<std::path::Path>,
        mut progress: impl Progress,
        resolve_in_pack_id: impl Fn(git_object::borrowed::Id) -> Option<PackOffset>,
    ) -> Result<Self, Error> {
        let mut r = io::BufReader::with_capacity(
            8192 * 8, // this value directly corresponds to performance, 8k (default) is about 4x slower than 64k
            fs::File::open(pack_path).map_err(|err| Error::Io(err, "open pack path"))?,
        );

        let mut tree = DiGraph::new();
        if let Some(num_objects) = offsets.size_hint().1 {
            progress.init(Some(num_objects as u32), Some("objects"));
        }

        {
            // safety check - assure ourselves it's a pack we can handle
            let mut buf = [0u8; PACK_HEADER_LEN];
            r.read_exact(&mut buf).map_err(|err| {
                Error::Io(
                    err,
                    "reading header buffer with at least 12 bytes failed - pack file truncated?",
                )
            })?;
            pack::data::parse::header(&buf)?;
        }

        let mut offsets_to_node = BTreeMap::new();
        let then = Instant::now();

        let mut previous_cursor_position = None::<u64>;

        for pack_offset in offsets {
            if let Some(previous_offset) = previous_cursor_position {
                DeltaTree::advance_cursor_to_pack_offset(&mut r, pack_offset, previous_offset)?;
            };
            let entry = pack::data::Entry::from_read(&mut r, pack_offset)
                .map_err(|err| Error::Io(err, "EOF while parsing header"))?;
            previous_cursor_position = Some(pack_offset + entry.header_size() as u64);
            use pack::data::Header::*;
            match entry.header {
                Tree | Blob | Commit | Tag => {
                    let base = tree.add_node(pack_offset);
                    offsets_to_node.insert(pack_offset, base);
                }
                RefDelta { base_id } => {
                    let base_or_child = tree.add_node(pack_offset);
                    offsets_to_node.insert(pack_offset, base_or_child);
                    if let Some(base_pack_offset) = resolve_in_pack_id(base_id.to_borrowed()) {
                        let base = offsets_to_node
                            .entry(base_pack_offset)
                            .or_insert_with(|| tree.add_node(base_pack_offset));
                        tree.add_edge(*base, base_or_child, ());
                    }
                }
                OfsDelta { base_distance } => {
                    let child = tree.add_node(pack_offset);
                    offsets_to_node.insert(pack_offset, child);
                    let base = offsets_to_node
                        .get(&(pack_offset - base_distance))
                        .expect("valid pack that puts bases before deltas that depend on it");
                    tree.add_edge(*base, child, ());
                }
            };
            progress.inc();
        }

        progress.show_throughput(then, tree.node_count() as u32, "entries");

        tree.shrink_to_fit();
        Ok(DeltaTree { inner: tree })
    }

    fn advance_cursor_to_pack_offset(
        r: &mut io::BufReader<fs::File>,
        pack_offset: u64,
        previous_offset: u64,
    ) -> Result<(), Error> {
        let mut bytes_to_skip = pack_offset
            .checked_sub(previous_offset)
            .expect("continuously ascending pack offets") as usize;
        while bytes_to_skip != 0 {
            let buf = r.fill_buf().map_err(|err| Error::Io(err, "skip bytes"))?;
            let bytes = buf.len().min(bytes_to_skip);
            r.consume(bytes);
            bytes_to_skip -= bytes;
        }
        Ok(())
    }
}
