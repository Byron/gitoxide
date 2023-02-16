use std::convert::TryFrom;

use crate::{object, Commit, Object, ObjectDetached, Tag, Tree};

impl<'repo> From<Object<'repo>> for ObjectDetached {
    fn from(mut v: Object<'repo>) -> Self {
        ObjectDetached {
            id: v.id,
            kind: v.kind,
            data: std::mem::take(&mut v.data),
        }
    }
}

impl<'repo> From<Commit<'repo>> for ObjectDetached {
    fn from(mut v: Commit<'repo>) -> Self {
        ObjectDetached {
            id: v.id,
            kind: gix_object::Kind::Commit,
            data: std::mem::take(&mut v.data),
        }
    }
}

impl<'repo> From<Tag<'repo>> for ObjectDetached {
    fn from(mut v: Tag<'repo>) -> Self {
        ObjectDetached {
            id: v.id,
            kind: gix_object::Kind::Tag,
            data: std::mem::take(&mut v.data),
        }
    }
}

impl<'repo> From<Commit<'repo>> for Object<'repo> {
    fn from(mut v: Commit<'repo>) -> Self {
        Object {
            id: v.id,
            kind: gix_object::Kind::Commit,
            data: steal_from_freelist(&mut v.data),
            repo: v.repo,
        }
    }
}

impl<'repo> AsRef<[u8]> for Object<'repo> {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

impl AsRef<[u8]> for ObjectDetached {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

impl<'repo> TryFrom<Object<'repo>> for Commit<'repo> {
    type Error = Object<'repo>;

    fn try_from(mut value: Object<'repo>) -> Result<Self, Self::Error> {
        let handle = value.repo;
        match value.kind {
            object::Kind::Commit => Ok(Commit {
                id: value.id,
                repo: handle,
                data: steal_from_freelist(&mut value.data),
            }),
            _ => Err(value),
        }
    }
}

impl<'repo> TryFrom<Object<'repo>> for Tag<'repo> {
    type Error = Object<'repo>;

    fn try_from(mut value: Object<'repo>) -> Result<Self, Self::Error> {
        let handle = value.repo;
        match value.kind {
            object::Kind::Tag => Ok(Tag {
                id: value.id,
                repo: handle,
                data: steal_from_freelist(&mut value.data),
            }),
            _ => Err(value),
        }
    }
}

impl<'repo> TryFrom<Object<'repo>> for Tree<'repo> {
    type Error = Object<'repo>;

    fn try_from(mut value: Object<'repo>) -> Result<Self, Self::Error> {
        let handle = value.repo;
        match value.kind {
            object::Kind::Tree => Ok(Tree {
                id: value.id,
                repo: handle,
                data: steal_from_freelist(&mut value.data),
            }),
            _ => Err(value),
        }
    }
}

impl<'r> std::fmt::Debug for Object<'r> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use gix_object::Kind::*;
        let type_name = match self.kind {
            Blob => "Blob",
            Commit => "Commit",
            Tree => "Tree",
            Tag => "Tag",
        };
        write!(f, "{}({})", type_name, self.id)
    }
}

/// In conjunction with the handles free list, leaving an empty Vec in place of the original causes it to not be
/// returned to the free list.
fn steal_from_freelist(data: &mut Vec<u8>) -> Vec<u8> {
    std::mem::take(data)
}
