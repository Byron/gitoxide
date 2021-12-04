use std::convert::TryFrom;

use crate::easy::{object, Object, ObjectRef, TreeRef};

impl<'repo> std::fmt::Debug for ObjectRef<'repo> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.id, f)
    }
}

impl<'repo> From<ObjectRef<'repo>> for Object {
    fn from(r: ObjectRef<'repo>) -> Self {
        r.into_owned()
    }
}

impl<'repo> AsRef<[u8]> for ObjectRef<'repo> {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

impl AsRef<[u8]> for Object {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

impl<'repo> TryFrom<ObjectRef<'repo>> for TreeRef<'repo> {
    type Error = ObjectRef<'repo>;

    fn try_from(value: ObjectRef<'repo>) -> Result<Self, Self::Error> {
        match value.kind {
            object::Kind::Tree => Ok(TreeRef {
                id: value.id,
                data: value.data,
                handle: value.handle,
            }),
            _ => Err(value),
        }
    }
}
