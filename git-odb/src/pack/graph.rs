use crate::{pack, pack::index::access::PackOffset};
use git_features::progress::Progress;
use petgraph::{
    graph::{DiGraph, NodeIndex},
    Direction,
};
use quick_error::quick_error;
use std::{collections::BTreeMap, convert::TryInto, io, time::SystemTime};

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

pub struct DeltaTree {
    inner: DiGraph<PackOffset, (), u32>, // u32 = max amount of objects in pack
}

pub struct Node {
    pub pack_offset: PackOffset,
    index: NodeIndex<u32>,
}

impl Node {}

/// Access
impl DeltaTree {
    pub fn bases(&self) -> impl Iterator<Item = Node> + '_ {
        self.inner.node_indices().filter_map(move |idx| {
            self.inner
                .neighbors_directed(idx, Direction::Incoming)
                .next()
                .map(|_| Node {
                    index: idx,
                    pack_offset: self.inner.node_weight(idx).copied().unwrap(),
                })
        })
    }

    pub fn node_count(&self) -> usize {
        self.inner.node_count()
    }

    pub fn children(&self, n: Node, out: &mut Vec<Node>) {
        out.clear();
        out.extend(
            self.inner
                .neighbors_directed(n.index, Direction::Outgoing)
                .map(|idx| Node {
                    index: idx,
                    pack_offset: self.inner.node_weight(idx).copied().unwrap(),
                }),
        )
    }
}

const PACK_HEADER_LEN: usize = 12;

/// Initialization
impl DeltaTree {
    /// The sort order is ascending.
    pub fn from_sorted_offsets(
        offsets: impl Iterator<Item = PackOffset>,
        mut r: impl io::BufRead + io::Read,
        mut progress: impl Progress,
    ) -> Result<Self, Error> {
        let mut tree = DiGraph::new();
        if let Some(num_objects) = offsets.size_hint().1 {
            progress.init(Some(num_objects as u32), Some("objects"));
        }

        {
            // safety check - assure ourselves it's a pack we can handle
            let buf = r.fill_buf().map_err(|err| Error::Io(err, "read header"))?;
            pack::data::parse::header(
                &buf[..PACK_HEADER_LEN]
                    .try_into()
                    .expect("buffer with at least 12 bytes - pack file truncated?"),
            )?;
            r.consume(PACK_HEADER_LEN);
        }

        let mut offsets_to_node = BTreeMap::new();
        let then = SystemTime::now();

        let mut count = 0;
        let mut previous_offset = None::<u64>;

        for pack_offset in offsets {
            count += 1;
            if let Some(previous_offset) = previous_offset {
                let mut bytes_to_skip = pack_offset
                    .checked_sub(previous_offset)
                    .expect("continuously ascending pack offets") as usize;
                while bytes_to_skip != 0 {
                    let buf = r.fill_buf().map_err(|err| Error::Io(err, "skip bytes"))?;
                    let bytes = buf.len().min(bytes_to_skip);
                    r.consume(bytes);
                    bytes_to_skip -= bytes;
                }
            };
            let (header, _decompressed_size, consumed) = pack::data::Header::from_read(&mut r, pack_offset)
                .map_err(|err| Error::Io(err, "EOF while parsing header"))?;
            previous_offset = Some(pack_offset + consumed as u64);
            use pack::data::Header::*;
            match header {
                Tree | Blob | Commit | Tag => {
                    let base = tree.add_node(pack_offset);
                    offsets_to_node.insert(pack_offset, base);
                }
                RefDelta { oid: _ } => {
                    let base = tree.add_node(pack_offset);
                    offsets_to_node.insert(pack_offset, base);
                }
                OfsDelta {
                    pack_offset: base_pack_offset,
                } => {
                    let child = tree.add_node(pack_offset);
                    offsets_to_node.insert(pack_offset, child);
                    let base = offsets_to_node
                        .get(&base_pack_offset)
                        .expect("valid pack that puts bases before deltas that depend on it");
                    tree.add_edge(*base, child, ());
                }
            };
            progress.set(count);
        }

        let elapsed = then.elapsed().expect("system time to work").as_secs_f32();
        progress.info(format!(
            "tree from {} entries in {:.02}s ({} entries /s)",
            tree.node_count(),
            elapsed,
            tree.node_count() as f32 / elapsed
        ));

        Ok(DeltaTree { inner: tree })
    }
}
