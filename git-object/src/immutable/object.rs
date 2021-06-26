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
    use git_actor::immutable::signature;
    use nom::error::{ContextError, ParseError};
    use quick_error::quick_error;

    quick_error! {
        /// An error returned by the [`Commit`][crate::immutable::Commit] method.
        #[derive(Debug, Clone)]
        #[allow(missing_docs)]
        pub enum Error {
            Parse(err_msg: String) {
                display("{}", err_msg)
            }
            NomDetail(input: crate::BString, msg: &'static str) {
                display("{}: '{}' could not be parsed", msg, input)
            }
            ParseKindError(err: crate::types::Error) {
                display("{}", err)
                source(err)
            }
            ObjectKind(err: crate::Error) {
                from()
                source(err)
            }
        }
    }

    impl ContextError<&[u8]> for Error {
        fn add_context(_input: &[u8], ctx: &'static str, _other_usually_internal_ignored: Self) -> Self {
            Error::Parse(ctx.into())
        }
    }

    impl ParseError<&[u8]> for Error {
        fn from_error_kind(input: &[u8], _kind: nom::error::ErrorKind) -> Self {
            Error::NomDetail(input.into(), "parse error")
        }

        fn append(_: &[u8], _: nom::error::ErrorKind, other: Self) -> Self {
            other
        }
    }

    impl From<nom::Err<Error>> for Error {
        fn from(e: nom::Err<Error>) -> Self {
            match e {
                nom::Err::Error(err) | nom::Err::Failure(err) => Error::Parse(err.to_string()),
                nom::Err::Incomplete(_) => unreachable!("we do not implement streaming parsers"),
            }
        }
    }

    impl From<signature::decode::Error> for Error {
        fn from(e: signature::decode::Error) -> Self {
            Error::Parse(e.to_string())
        }
    }
}
