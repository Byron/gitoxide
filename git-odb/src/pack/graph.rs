use crate::{pack, pack::index::access::PackOffset};
use git_features::progress::Progress;
use petgraph::graph::DiGraph;
use quick_error::quick_error;
use std::{collections::BTreeMap, io};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: io::Error, msg: &'static str) {
            display("{}", msg)
            source(err)
        }
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub enum PackEntryKind {
    RefBase(PackOffset),
    Base(PackOffset),
    Delta(PackOffset),
}

pub struct DeltaTree {
    inner: DiGraph<PackEntryKind, (), u32>, // u32 = max amount of objects in pack
}

impl DeltaTree {
    /// The sort order is ascending.
    pub fn from_sorted_offsets(
        offsets: impl Iterator<Item = PackOffset>,
        mut r: impl io::BufRead + io::Read + io::Seek,
        mut progress: impl Progress,
    ) -> Result<Self, Error> {
        let mut tree = DiGraph::new();
        if let Some(num_objects) = offsets.size_hint().1 {
            progress.init(Some(num_objects as u32), Some("objects"));
        }

        let mut offsets_to_node = BTreeMap::new();

        let mut count = 0;
        for pack_offset in offsets {
            count += 1;
            r.seek(io::SeekFrom::Start(pack_offset))
                .map_err(|err| Error::Io(err, "Seek to next offset failed"))?;
            let (header, _decompressed_size) = pack::data::Header::from_read(&mut r, pack_offset)
                .map_err(|err| Error::Io(err, "EOF while parsing header"))?;
            use pack::data::Header::*;
            match header {
                Tree | Blob | Commit | Tag => {
                    let base = tree.add_node(PackEntryKind::Base(pack_offset));
                    offsets_to_node.insert(pack_offset, base);
                }
                RefDelta { oid: _ } => {
                    let base = tree.add_node(PackEntryKind::RefBase(pack_offset));
                    offsets_to_node.insert(pack_offset, base);
                }
                OfsDelta {
                    pack_offset: base_pack_offset,
                } => {
                    let child = tree.add_node(PackEntryKind::Delta(pack_offset));
                    offsets_to_node.insert(pack_offset, child);
                    let base = offsets_to_node
                        .get(&base_pack_offset)
                        .expect("valid pack that puts bases before deltas that depend on it");
                    tree.add_edge(*base, child, ());
                }
            };
            progress.set(count);
        }

        Ok(DeltaTree { inner: tree })
    }
}
