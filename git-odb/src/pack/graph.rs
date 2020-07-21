use crate::{pack, pack::index::access::PackOffset};
use git_features::progress::Progress;
use petgraph::{
    graph::{DiGraph, NodeIndex},
    Direction,
};
use quick_error::quick_error;
use std::{collections::BTreeMap, fs, io, time::SystemTime};

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

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
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
                pack_offset: self.inner.node_weight(idx).copied().unwrap(),
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
    /// The sort order is ascending. The given packfile path must match the provided offsets.
    pub fn from_sorted_offsets(
        offsets: impl Iterator<Item = PackOffset>,
        pack_path: impl AsRef<std::path::Path>,
        mut progress: impl Progress,
        resolve_in_pack_oid: impl Fn(git_object::borrowed::Id) -> Option<PackOffset>,
    ) -> Result<Self, Error> {
        use io::{BufRead, Read};

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
                RefDelta { oid } => {
                    let base_or_child = tree.add_node(pack_offset);
                    offsets_to_node.insert(pack_offset, base_or_child);
                    if let Some(base_pack_offset) = resolve_in_pack_oid(oid.to_borrowed()) {
                        let base = offsets_to_node
                            .entry(base_pack_offset)
                            .or_insert_with(|| tree.add_node(base_pack_offset));
                        tree.add_edge(*base, base_or_child, ());
                    }
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

        tree.shrink_to_fit();
        Ok(DeltaTree { inner: tree })
    }
}
