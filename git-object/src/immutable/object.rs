use crate::{BlobRef, CommitRef, Kind, Object, ObjectRef, TagRef, TreeRef};

impl<'a> ObjectRef<'a> {
    /// Deserialize an object of `kind` from the given `data`.
    pub fn from_bytes(kind: Kind, data: &'a [u8]) -> Result<ObjectRef<'a>, decode::Error> {
        Ok(match kind {
            Kind::Tree => ObjectRef::Tree(TreeRef::from_bytes(data)?),
            Kind::Blob => ObjectRef::Blob(BlobRef { data }),
            Kind::Commit => ObjectRef::Commit(CommitRef::from_bytes(data)?),
            Kind::Tag => ObjectRef::Tag(TagRef::from_bytes(data)?),
        })
    }

    /// Convert the immutable object into a mutable version, consuming the source in the process.
    ///
    /// Note that this is an expensive operation.
    pub fn into_owned(self) -> Object {
        self.into()
    }

    /// Convert this immutable object into its mutable counterpart.
    ///
    /// Note that this is an expensive operation.
    pub fn to_owned(&self) -> Object {
        self.clone().into()
    }
}

/// Convenient access to contained objects.
impl<'a> ObjectRef<'a> {
    /// Interpret this object as blob.
    pub fn as_blob(&self) -> Option<&BlobRef<'a>> {
        match self {
            ObjectRef::Blob(v) => Some(v),
            _ => None,
        }
    }
    /// Interpret this object as blob, chainable.
    pub fn into_blob(self) -> Option<BlobRef<'a>> {
        match self {
            ObjectRef::Blob(v) => Some(v),
            _ => None,
        }
    }
    /// Interpret this object as commit.
    pub fn as_commit(&self) -> Option<&CommitRef<'a>> {
        match self {
            ObjectRef::Commit(v) => Some(v),
            _ => None,
        }
    }
    /// Interpret this object as commit, chainable.
    pub fn into_commit(self) -> Option<CommitRef<'a>> {
        match self {
            ObjectRef::Commit(v) => Some(v),
            _ => None,
        }
    }
    /// Interpret this object as tree.
    pub fn as_tree(&self) -> Option<&TreeRef<'a>> {
        match self {
            ObjectRef::Tree(v) => Some(v),
            _ => None,
        }
    }
    /// Interpret this object as tree, chainable
    pub fn into_tree(self) -> Option<TreeRef<'a>> {
        match self {
            ObjectRef::Tree(v) => Some(v),
            _ => None,
        }
    }
    /// Interpret this object as tag.
    pub fn as_tag(&self) -> Option<&TagRef<'a>> {
        match self {
            ObjectRef::Tag(v) => Some(v),
            _ => None,
        }
    }
    /// Interpret this object as tag, chainable.
    pub fn into_tag(self) -> Option<TagRef<'a>> {
        match self {
            ObjectRef::Tag(v) => Some(v),
            _ => None,
        }
    }
    /// Return the kind of object.
    pub fn kind(&self) -> Kind {
        match self {
            ObjectRef::Tree(_) => Kind::Tree,
            ObjectRef::Blob(_) => Kind::Blob,
            ObjectRef::Commit(_) => Kind::Commit,
            ObjectRef::Tag(_) => Kind::Tag,
        }
    }
}

mod convert {
    use std::convert::TryFrom;

    use crate::{BlobRef, CommitRef, ObjectRef, TagRef, TreeRef};

    impl<'a> From<TagRef<'a>> for ObjectRef<'a> {
        fn from(v: TagRef<'a>) -> Self {
            ObjectRef::Tag(v)
        }
    }

    impl<'a> From<CommitRef<'a>> for ObjectRef<'a> {
        fn from(v: CommitRef<'a>) -> Self {
            ObjectRef::Commit(v)
        }
    }

    impl<'a> From<TreeRef<'a>> for ObjectRef<'a> {
        fn from(v: TreeRef<'a>) -> Self {
            ObjectRef::Tree(v)
        }
    }

    impl<'a> From<BlobRef<'a>> for ObjectRef<'a> {
        fn from(v: BlobRef<'a>) -> Self {
            ObjectRef::Blob(v)
        }
    }

    impl<'a> TryFrom<ObjectRef<'a>> for TagRef<'a> {
        type Error = ObjectRef<'a>;

        fn try_from(value: ObjectRef<'a>) -> Result<Self, Self::Error> {
            Ok(match value {
                ObjectRef::Tag(v) => v,
                _ => return Err(value),
            })
        }
    }

    impl<'a> TryFrom<ObjectRef<'a>> for CommitRef<'a> {
        type Error = ObjectRef<'a>;

        fn try_from(value: ObjectRef<'a>) -> Result<Self, Self::Error> {
            Ok(match value {
                ObjectRef::Commit(v) => v,
                _ => return Err(value),
            })
        }
    }

    impl<'a> TryFrom<ObjectRef<'a>> for TreeRef<'a> {
        type Error = ObjectRef<'a>;

        fn try_from(value: ObjectRef<'a>) -> Result<Self, Self::Error> {
            Ok(match value {
                ObjectRef::Tree(v) => v,
                _ => return Err(value),
            })
        }
    }

    impl<'a> TryFrom<ObjectRef<'a>> for BlobRef<'a> {
        type Error = ObjectRef<'a>;

        fn try_from(value: ObjectRef<'a>) -> Result<Self, Self::Error> {
            Ok(match value {
                ObjectRef::Blob(v) => v,
                _ => return Err(value),
            })
        }
    }
}

///
#[cfg(feature = "verbose-object-parsing-errors")]
pub mod decode {
    use crate::bstr::{BString, ByteSlice};

    /// The type to be used for parse errors.
    pub type ParseError<'a> = nom::error::VerboseError<&'a [u8]>;
    /// The owned type to be used for parse errors.
    pub type ParseErrorOwned = nom::error::VerboseError<BString>;

    /// A type to indicate errors during parsing and to abstract away details related to `nom`.
    #[derive(Debug, Clone)]
    pub struct Error {
        /// The actual error
        pub inner: ParseErrorOwned,
    }

    impl<'a> From<nom::Err<ParseError<'a>>> for Error {
        fn from(v: nom::Err<ParseError<'a>>) -> Self {
            Error {
                inner: match v {
                    nom::Err::Error(err) | nom::Err::Failure(err) => nom::error::VerboseError {
                        errors: err
                            .errors
                            .into_iter()
                            .map(|(i, v)| (i.as_bstr().to_owned(), v))
                            .collect(),
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

///
#[cfg(not(feature = "verbose-object-parsing-errors"))]
pub mod decode {
    /// The type to be used for parse errors, discards everything and is zero size
    pub type ParseError<'a> = ();
    /// The owned type to be used for parse errors, discards everything and is zero size
    pub type ParseErrorOwned = ();

    /// A type to indicate errors during parsing and to abstract away details related to `nom`.
    #[derive(Debug, Clone)]
    pub struct Error {
        /// The actual error
        pub inner: ParseErrorOwned,
    }

    impl<'a> From<nom::Err<ParseError<'a>>> for Error {
        fn from(v: nom::Err<ParseError<'a>>) -> Self {
            Error {
                inner: match v {
                    nom::Err::Error(err) | nom::Err::Failure(err) => err,
                    nom::Err::Incomplete(_) => unreachable!("we don't have streaming parsers"),
                },
            }
        }
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Ok(())
        }
    }

    impl std::error::Error for Error {}
}
