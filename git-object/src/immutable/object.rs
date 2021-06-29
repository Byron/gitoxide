use crate::{
    immutable,
    immutable::{Blob, Commit, Tag, Tree},
    Kind,
};

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
    pub fn from_bytes(kind: Kind, data: &'a [u8]) -> Result<Object<'a>, decode::Error> {
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
    pub fn as_blob(&self) -> Option<&immutable::Blob<'a>> {
        match self {
            Object::Blob(v) => Some(v),
            _ => None,
        }
    }
    /// Interpret this object as blob, chainable.
    pub fn into_blob(self) -> Option<immutable::Blob<'a>> {
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
    /// Interpret this object as commit, chainable.
    pub fn into_commit(self) -> Option<immutable::Commit<'a>> {
        match self {
            Object::Commit(v) => Some(v),
            _ => None,
        }
    }
    /// Interpret this object as tree.
    pub fn as_tree(&self) -> Option<&immutable::Tree<'a>> {
        match self {
            Object::Tree(v) => Some(v),
            _ => None,
        }
    }
    /// Interpret this object as tree, chainable
    pub fn into_tree(self) -> Option<immutable::Tree<'a>> {
        match self {
            Object::Tree(v) => Some(v),
            _ => None,
        }
    }
    /// Interpret this object as tag.
    pub fn as_tag(&self) -> Option<&immutable::Tag<'a>> {
        match self {
            Object::Tag(v) => Some(v),
            _ => None,
        }
    }
    /// Interpret this object as tag, chainable.
    pub fn into_tag(self) -> Option<immutable::Tag<'a>> {
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
    use std::convert::TryFrom;

    use crate::immutable::{Blob, Commit, Object, Tag, Tree};

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

///
pub mod decode {
    use crate::bstr::{BString, ByteSlice};

    /// The type to be used for parse errors.
    pub type ParseError<'a, T = [u8]> = nom::error::Error<&'a T>;
    /// The owned type to be used for parse errors.
    pub type ParseErrorOwned = nom::error::Error<BString>;

    /// A type to indicate errors during parsing and to abstract away details related to `nom`.
    #[derive(Debug)]
    pub struct Error {
        /// The actual error
        pub inner: ParseErrorOwned,
    }

    impl Clone for Error {
        fn clone(&self) -> Self {
            use nom::error::ParseError;
            Error {
                inner: ParseErrorOwned::from_error_kind(self.inner.input.clone(), self.inner.code),
            }
        }
    }

    impl<'a> From<nom::Err<ParseError<'a, [u8]>>> for Error {
        fn from(v: nom::Err<ParseError<'a>>) -> Self {
            Error {
                inner: match v {
                    nom::Err::Error(err) | nom::Err::Failure(err) => nom::error::Error {
                        input: err.input.as_bstr().to_owned(),
                        code: err.code,
                    },
                    nom::Err::Incomplete(_) => unreachable!("we don't have streaming parsers"),
                },
            }
        }
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.inner.fmt(f)
        }
    }

    impl std::error::Error for Error {}
}
