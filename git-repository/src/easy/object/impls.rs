use std::convert::TryFrom;

use crate::easy::{object, Object, ObjectRef, TreeRef};

impl<'repo, A> std::fmt::Debug for ObjectRef<'repo, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.id, f)
    }
}

impl<'repo, A> From<ObjectRef<'repo, A>> for Object {
    fn from(r: ObjectRef<'repo, A>) -> Self {
        r.into_owned()
    }
}

impl<'repo, A> AsRef<[u8]> for ObjectRef<'repo, A> {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

impl AsRef<[u8]> for Object {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

impl<'repo, A> TryFrom<ObjectRef<'repo, A>> for TreeRef<'repo, A> {
    type Error = ObjectRef<'repo, A>;

    fn try_from(value: ObjectRef<'repo, A>) -> Result<Self, Self::Error> {
        match value.kind {
            object::Kind::Tree => Ok(TreeRef {
                id: value.id,
                data: value.data,
                access: value.access,
            }),
            _ => Err(value),
        }
    }
}
