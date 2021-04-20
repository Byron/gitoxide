use bstr::BStr;

use crate::{
    immutable,
    immutable::{parse, Blob, Commit, Tag, Tree},
    Kind, Time,
};

mod error;
pub use error::Error;

/// A signature is created by an actor at a certain time.
///
/// Note that this is not a cryptographical signature.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Signature<'a> {
    /// The actor's name.
    #[cfg_attr(feature = "serde1", serde(borrow))]
    pub name: &'a BStr,
    /// The actor's email.
    pub email: &'a BStr,
    /// The time stamp at which the signature was performed.
    pub time: Time,
}

impl<'a> Signature<'a> {
    /// Deserialize a signature from the given `data`.
    pub fn from_bytes(data: &'a [u8]) -> Result<Signature<'a>, Error> {
        parse::signature(data).map(|(_, t)| t).map_err(Error::from)
    }
}

/// An immutable object representing [`Trees`][Tree], [`Blobs`][Blob], [`Commits`][Commit], or [`Tags`][Tag].
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
#[allow(missing_docs)]
pub enum Object<'a> {
    #[cfg_attr(feature = "serde1", serde(borrow))]
    Tree(Tree<'a>),
    Blob(Blob<'a>),
    Commit(Commit<'a>),
    Tag(Tag<'a>),
}

impl<'a> Object<'a> {
    /// Deserialize an object of `kind` from the given `data`.
    pub fn from_bytes(kind: Kind, data: &'a [u8]) -> Result<Object<'a>, Error> {
        Ok(match kind {
            Kind::Tree => Object::Tree(Tree::from_bytes(data)?),
            Kind::Blob => Object::Blob(Blob { data }),
            Kind::Commit => Object::Commit(Commit::from_bytes(data)?),
            Kind::Tag => Object::Tag(Tag::from_bytes(data)?),
        })
    }

    /// Convert the immutable object into a mutable version, consuming the source in the process.
    ///
    /// Note that this is an expensive operation.
    pub fn into_mutable(self) -> crate::mutable::Object {
        self.into()
    }

    /// Convert this immutable object into its mutable counterpart.
    ///
    /// Note that this is an expensive operation.
    pub fn to_mutable(&self) -> crate::mutable::Object {
        self.clone().into()
    }
}

/// Convenient access to contained objects.
impl<'a> Object<'a> {
    /// Interpret this object as blob.
    pub fn as_blob(&self) -> Option<&immutable::Blob<'_>> {
        match self {
            Object::Blob(v) => Some(v),
            _ => None,
        }
    }
    /// Interpret this object as commit.
    pub fn as_commit(&self) -> Option<&immutable::Commit<'a>> {
        match self {
            Object::Commit(v) => Some(v),
            _ => None,
        }
    }
    /// Interpret this object as tree.
    pub fn as_tree(&self) -> Option<&immutable::Tree<'_>> {
        match self {
            Object::Tree(v) => Some(v),
            _ => None,
        }
    }
    /// Interpret this object as tag.
    pub fn as_tag(&self) -> Option<&immutable::Tag<'_>> {
        match self {
            Object::Tag(v) => Some(v),
            _ => None,
        }
    }
    /// Return the kind of object.
    pub fn kind(&self) -> Kind {
        match self {
            Object::Tree(_) => Kind::Tree,
            Object::Blob(_) => Kind::Blob,
            Object::Commit(_) => Kind::Commit,
            Object::Tag(_) => Kind::Tag,
        }
    }
}

mod convert {
    use crate::immutable::{Blob, Commit, Object, Tag, Tree};
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
