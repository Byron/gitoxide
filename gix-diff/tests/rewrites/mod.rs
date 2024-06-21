use gix_diff::rewrites::tracker::ChangeKind;
use gix_hash::{oid, ObjectId};
use gix_object::tree::{EntryKind, EntryMode};

mod tracker;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Change {
    id: ObjectId,
    kind: ChangeKind,
    mode: EntryMode,
}

impl gix_diff::rewrites::tracker::Change for Change {
    fn id(&self) -> &oid {
        &self.id
    }

    fn kind(&self) -> ChangeKind {
        self.kind
    }

    fn entry_mode(&self) -> EntryMode {
        self.mode
    }

    fn id_and_entry_mode(&self) -> (&oid, EntryMode) {
        (&self.id, self.mode)
    }
}

const NULL_ID: gix_hash::ObjectId = gix_hash::Kind::Sha1.null();

impl Change {
    fn modification() -> Self {
        Change {
            id: NULL_ID,
            kind: ChangeKind::Modification,
            mode: EntryKind::Blob.into(),
        }
    }
    fn deletion() -> Self {
        Change {
            id: NULL_ID,
            kind: ChangeKind::Deletion,
            mode: EntryKind::Blob.into(),
        }
    }
    fn addition() -> Self {
        Change {
            id: NULL_ID,
            kind: ChangeKind::Addition,
            mode: EntryKind::Blob.into(),
        }
    }
}
