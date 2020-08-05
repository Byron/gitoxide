use crate::pack;
use git_object::owned;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Outcome {
    pub index_kind: pack::index::Kind,
    pub index_hash: owned::Id,
    pub pack_hash: owned::Id,
    pub num_objects: u32,
}

#[derive(Clone)]
pub enum ObjectKind {
    Base(git_object::Kind),
    OfsDelta,
}

impl ObjectKind {
    pub fn to_kind(&self) -> Option<git_object::Kind> {
        match self {
            ObjectKind::Base(kind) => Some(*kind),
            ObjectKind::OfsDelta => None,
        }
    }
}

pub struct TreeEntry {
    pub id: owned::Id,
    pub kind: ObjectKind,
    pub crc32: u32,
}

impl Default for TreeEntry {
    fn default() -> Self {
        TreeEntry {
            id: owned::Id::null(),
            kind: ObjectKind::OfsDelta,
            crc32: 0,
        }
    }
}

pub type EntrySlice = std::ops::Range<u64>;
