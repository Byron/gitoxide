mod error;
pub use error::Error;

use crate::BStr;
use crate::{
    borrowed,
    borrowed::{Blob, Commit, Tag, Tree},
    Time,
};

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Signature<'a> {
    #[cfg_attr(feature = "serde1", serde(borrow))]
    pub name: &'a BStr,
    pub email: &'a BStr,
    pub time: Time,
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Object<'a> {
    #[cfg_attr(feature = "serde1", serde(borrow))]
    Tag(Tag<'a>),
    Commit(Commit<'a>),
    Tree(Tree<'a>),
    Blob(Blob<'a>),
}

impl<'a> Object<'a> {
    pub fn as_blob(&self) -> Option<&borrowed::Blob> {
        match self {
            Object::Blob(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_commit(&self) -> Option<&borrowed::Commit<'a>> {
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

    impl<'a> From<Tag<'a>> for Object<'a> {
        fn from(v: Tag<'a>) -> Self {
            Object::Tag(v)
        }
    }

    impl<'a> From<Commit<'a>> for Object<'a> {
        fn from(v: Commit<'a>) -> Self {
            Object::Commit(v)
        }
    }

    impl<'a> From<Tree<'a>> for Object<'a> {
        fn from(v: Tree<'a>) -> Self {
            Object::Tree(v)
        }
    }

    impl<'a> From<Blob<'a>> for Object<'a> {
        fn from(v: Blob<'a>) -> Self {
            Object::Blob(v)
        }
    }

    impl<'a> TryFrom<Object<'a>> for Tag<'a> {
        type Error = Object<'a>;

        fn try_from(value: Object<'a>) -> Result<Self, Self::Error> {
            Ok(match value {
                Object::Tag(v) => v,
                _ => return Err(value),
            })
        }
    }

    impl<'a> TryFrom<Object<'a>> for Commit<'a> {
        type Error = Object<'a>;

        fn try_from(value: Object<'a>) -> Result<Self, Self::Error> {
            Ok(match value {
                Object::Commit(v) => v,
                _ => return Err(value),
            })
        }
    }

    impl<'a> TryFrom<Object<'a>> for Tree<'a> {
        type Error = Object<'a>;

        fn try_from(value: Object<'a>) -> Result<Self, Self::Error> {
            Ok(match value {
                Object::Tree(v) => v,
                _ => return Err(value),
            })
        }
    }

    impl<'a> TryFrom<Object<'a>> for Blob<'a> {
        type Error = Object<'a>;

        fn try_from(value: Object<'a>) -> Result<Self, Self::Error> {
            Ok(match value {
                Object::Blob(v) => v,
                _ => return Err(value),
            })
        }
    }
}
