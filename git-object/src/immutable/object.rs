use bstr::BStr;

use crate::{
    immutable,
    immutable::{parse, Blob, Commit, Tag, Tree},
    Kind, Time,
};

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
    pub fn from_bytes(data: &'a [u8]) -> Result<Signature<'a>, decode::Error> {
        parse::signature(data).map(|(_, t)| t).map_err(decode::Error::from)
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
    use nom::error::ParseError;
    use quick_error::quick_error;

    quick_error! {
        /// An error returned by various [`Commit`][crate::immutable::Commit] and [`Signature`][crate::immutable::Signature] methods.
        #[derive(Debug)]
        #[allow(missing_docs)]
        pub enum Error {
            ParseIntegerError(msg: &'static str, number: crate::BString, err: btoi::ParseIntegerError) {
                display("{}: {:?}", msg, number)
                source(err)
            }
            Nom(err_msg: String) {
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

    impl Error {
        fn set_parse_context(mut self, ctx: &'static str) -> Self {
            if let Error::NomDetail(_, ref mut message) = self {
                *message = ctx
            }
            self
        }

        pub(crate) fn context(msg: &'static str) -> impl Fn(nom::Err<Self>) -> nom::Err<Self> {
            move |e: nom::Err<Self>| e.map(|e| e.set_parse_context(msg))
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
                nom::Err::Error(err) | nom::Err::Failure(err) => Error::Nom(err.to_string()),
                nom::Err::Incomplete(_) => unreachable!("we do not implement streaming parsers"),
            }
        }
    }
}
