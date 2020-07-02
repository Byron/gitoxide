mod error;
pub use error::Error;

use crate::Bytes;
use crate::{
    borrowed,
    borrowed::{Blob, Commit, Tag, Tree},
    Time,
};

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub struct Signature<'data> {
    pub name: &'data Bytes,
    pub email: &'data Bytes,
    pub time: Time,
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub enum Object<'data> {
    Tag(Tag<'data>),
    Commit(Commit<'data>),
    Tree(Tree<'data>),
    Blob(Blob<'data>),
}

impl<'data> Object<'data> {
    pub fn as_blob(&self) -> Option<&borrowed::Blob> {
        match self {
            Object::Blob(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_commit(&self) -> Option<&borrowed::Commit<'data>> {
        match self {
            Object::Commit(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_tree(&self) -> Option<&borrowed::Tree> {
        match self {
            Object::Tree(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_tag(&self) -> Option<&borrowed::Tag> {
        match self {
            Object::Tag(v) => Some(v),
            _ => None,
        }
    }
    pub fn kind(&self) -> crate::Kind {
        match self {
            Object::Tag(_) => crate::Kind::Tag,
            Object::Commit(_) => crate::Kind::Commit,
            Object::Tree(_) => crate::Kind::Tree,
            Object::Blob(_) => crate::Kind::Blob,
        }
    }
}

mod convert {
    use crate::borrowed::{Blob, Commit, Object, Tag, Tree};
    use std::convert::TryFrom;

    impl<'data> From<Tag<'data>> for Object<'data> {
        fn from(v: Tag<'data>) -> Self {
            Object::Tag(v)
        }
    }

    impl<'data> From<Commit<'data>> for Object<'data> {
        fn from(v: Commit<'data>) -> Self {
            Object::Commit(v)
        }
    }

    impl<'data> From<Tree<'data>> for Object<'data> {
        fn from(v: Tree<'data>) -> Self {
            Object::Tree(v)
        }
    }

    impl<'data> From<Blob<'data>> for Object<'data> {
        fn from(v: Blob<'data>) -> Self {
            Object::Blob(v)
        }
    }

    impl<'data> TryFrom<Object<'data>> for Tag<'data> {
        type Error = Object<'data>;

        fn try_from(value: Object<'data>) -> Result<Self, Self::Error> {
            Ok(match value {
                Object::Tag(v) => v,
                _ => return Err(value),
            })
        }
    }

    impl<'data> TryFrom<Object<'data>> for Commit<'data> {
        type Error = Object<'data>;

        fn try_from(value: Object<'data>) -> Result<Self, Self::Error> {
            Ok(match value {
                Object::Commit(v) => v,
                _ => return Err(value),
            })
        }
    }

    impl<'data> TryFrom<Object<'data>> for Tree<'data> {
        type Error = Object<'data>;

        fn try_from(value: Object<'data>) -> Result<Self, Self::Error> {
            Ok(match value {
                Object::Tree(v) => v,
                _ => return Err(value),
            })
        }
    }

    impl<'data> TryFrom<Object<'data>> for Blob<'data> {
        type Error = Object<'data>;

        fn try_from(value: Object<'data>) -> Result<Self, Self::Error> {
            Ok(match value {
                Object::Blob(v) => v,
                _ => return Err(value),
            })
        }
    }
}
