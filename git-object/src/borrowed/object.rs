use bstr::BStr;

use crate::{
    borrowed,
    borrowed::{parse, Blob, Commit, Tag, Tree},
    Kind, Time,
};

mod error;
pub use error::Error;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Signature<'a> {
    #[cfg_attr(feature = "serde1", serde(borrow))]
    pub name: &'a BStr,
    pub email: &'a BStr,
    pub time: Time,
}

impl<'a> Signature<'a> {
    pub fn from_bytes(d: &'a [u8]) -> Result<Signature<'a>, Error> {
        parse::signature(d).map(|(_, t)| t).map_err(Error::from)
    }
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
    pub fn from_bytes(kind: Kind, bytes: &'a [u8]) -> Result<Object<'a>, Error> {
        Ok(match kind {
            Kind::Tag => Object::Tag(Tag::from_bytes(bytes)?),
            Kind::Tree => Object::Tree(Tree::from_bytes(bytes)?),
            Kind::Commit => Object::Commit(Commit::from_bytes(bytes)?),
            Kind::Blob => Object::Blob(Blob { data: bytes }),
        })
    }
}

/// Convenient access to contained objects
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
    pub fn kind(&self) -> Kind {
        match self {
            Object::Tag(_) => Kind::Tag,
            Object::Commit(_) => Kind::Commit,
            Object::Tree(_) => Kind::Tree,
            Object::Blob(_) => Kind::Blob,
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
