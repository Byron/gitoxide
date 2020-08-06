use crate::owned;
use std::io;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::large_enum_variant)]
pub enum Object {
    Tree(owned::Tree),
    Blob(owned::Blob),
    Commit(owned::Commit),
    Tag(owned::Tag),
}

/// Convenient extraction of typed object
impl Object {
    pub fn as_blob(&self) -> Option<&owned::Blob> {
        match self {
            Object::Blob(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_commit(&self) -> Option<&owned::Commit> {
        match self {
            Object::Commit(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_tree(&self) -> Option<&owned::Tree> {
        match self {
            Object::Tree(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_tag(&self) -> Option<&owned::Tag> {
        match self {
            Object::Tag(v) => Some(v),
            _ => None,
        }
    }
    pub fn kind(&self) -> crate::Kind {
        match self {
            Object::Tree(_) => crate::Kind::Tree,
            Object::Blob(_) => crate::Kind::Blob,
            Object::Commit(_) => crate::Kind::Commit,
            Object::Tag(_) => crate::Kind::Tag,
        }
    }
}

/// Serialization
impl Object {
    pub fn write_to(&self, out: impl io::Write) -> io::Result<()> {
        use Object::*;
        match self {
            Tree(v) => v.write_to(out),
            Blob(v) => v.write_to(out),
            Commit(v) => v.write_to(out),
            Tag(v) => v.write_to(out),
        }
    }
}

mod convert {
    use crate::owned::{Blob, Commit, Object, Tag, Tree};
    use std::convert::TryFrom;

    impl From<Tag> for Object {
        fn from(v: Tag) -> Self {
            Object::Tag(v)
        }
    }

    impl From<Commit> for Object {
        fn from(v: Commit) -> Self {
            Object::Commit(v)
        }
    }

    impl From<Tree> for Object {
        fn from(v: Tree) -> Self {
            Object::Tree(v)
        }
    }

    impl From<Blob> for Object {
        fn from(v: Blob) -> Self {
            Object::Blob(v)
        }
    }

    impl TryFrom<Object> for Tag {
        type Error = Object;

        fn try_from(value: Object) -> Result<Self, Self::Error> {
            Ok(match value {
                Object::Tag(v) => v,
                _ => return Err(value),
            })
        }
    }

    impl TryFrom<Object> for Commit {
        type Error = Object;

        fn try_from(value: Object) -> Result<Self, Self::Error> {
            Ok(match value {
                Object::Commit(v) => v,
                _ => return Err(value),
            })
        }
    }

    impl TryFrom<Object> for Tree {
        type Error = Object;

        fn try_from(value: Object) -> Result<Self, Self::Error> {
            Ok(match value {
                Object::Tree(v) => v,
                _ => return Err(value),
            })
        }
    }

    impl TryFrom<Object> for Blob {
        type Error = Object;

        fn try_from(value: Object) -> Result<Self, Self::Error> {
            Ok(match value {
                Object::Blob(v) => v,
                _ => return Err(value),
            })
        }
    }
}
