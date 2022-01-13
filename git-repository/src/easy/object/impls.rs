use std::convert::TryFrom;

use crate::easy::{object, Commit, DetachedObject, Object, Tree};

impl<'repo> std::fmt::Debug for Object<'repo> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.id, f)
    }
}

impl<'repo> From<Object<'repo>> for DetachedObject {
    fn from(r: Object<'repo>) -> Self {
        r.into_owned()
    }
}

impl<'repo> AsRef<[u8]> for Object<'repo> {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

impl AsRef<[u8]> for DetachedObject {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

impl<'repo> TryFrom<Object<'repo>> for Commit<'repo> {
    type Error = Object<'repo>;

    fn try_from(mut value: Object<'repo>) -> Result<Self, Self::Error> {
        let handle = value.handle;
        match value.kind {
            object::Kind::Commit => Ok(Commit {
                id: value.id,
                handle,
                data: steal(&mut value.data),
            }),
            _ => Err(value),
        }
    }
}

impl<'repo> TryFrom<Object<'repo>> for Tree<'repo> {
    type Error = Object<'repo>;

    fn try_from(mut value: Object<'repo>) -> Result<Self, Self::Error> {
        let handle = value.handle;
        match value.kind {
            object::Kind::Tree => Ok(Tree {
                id: value.id,
                handle,
                data: steal(&mut value.data),
            }),
            _ => Err(value),
        }
    }
}

/// In conjunction with the handles free list, leaving an empty Vec in place of the original causes it to not be
/// returned to the free list.
fn steal(data: &mut Vec<u8>) -> Vec<u8> {
    std::mem::take(data)
}
