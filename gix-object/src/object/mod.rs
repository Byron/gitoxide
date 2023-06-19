use crate::{Blob, Commit, Object, Tag, Tree};

mod convert;

mod write {
    use std::io;

    use crate::{Kind, Object, ObjectRef, WriteTo};

    /// Serialization
    impl<'a> WriteTo for ObjectRef<'a> {
        /// Write the contained object to `out` in the git serialization format.
        fn write_to(&self, out: impl io::Write) -> io::Result<()> {
            match self {
                Self::Tree(v) => v.write_to(out),
                Self::Blob(v) => v.write_to(out),
                Self::Commit(v) => v.write_to(out),
                Self::Tag(v) => v.write_to(out),
            }
        }

        fn size(&self) -> usize {
            match self {
                Self::Tree(v) => v.size(),
                Self::Blob(v) => v.size(),
                Self::Commit(v) => v.size(),
                Self::Tag(v) => v.size(),
            }
        }

        fn kind(&self) -> Kind {
            self.kind()
        }
    }

    /// Serialization
    impl WriteTo for Object {
        /// Write the contained object to `out` in the git serialization format.
        fn write_to(&self, out: impl io::Write) -> io::Result<()> {
            match self {
                Self::Tree(v) => v.write_to(out),
                Self::Blob(v) => v.write_to(out),
                Self::Commit(v) => v.write_to(out),
                Self::Tag(v) => v.write_to(out),
            }
        }

        fn size(&self) -> usize {
            match self {
                Self::Tree(v) => v.size(),
                Self::Blob(v) => v.size(),
                Self::Commit(v) => v.size(),
                Self::Tag(v) => v.size(),
            }
        }

        fn kind(&self) -> Kind {
            self.kind()
        }
    }
}

/// Convenient extraction of typed object.
impl Object {
    /// Turns this instance into a [`Blob`][Blob], panic otherwise.
    pub fn into_blob(self) -> Blob {
        match self {
            Object::Blob(v) => v,
            _ => panic!("BUG: not a blob"),
        }
    }
    /// Turns this instance into a [`Commit`][Commit] panic otherwise.
    pub fn into_commit(self) -> Commit {
        match self {
            Object::Commit(v) => v,
            _ => panic!("BUG: not a commit"),
        }
    }
    /// Turns this instance into a [`Tree`][Tree] panic otherwise.
    pub fn into_tree(self) -> Tree {
        match self {
            Object::Tree(v) => v,
            _ => panic!("BUG: not a tree"),
        }
    }
    /// Turns this instance into a [`Tag`][Tag] panic otherwise.
    pub fn into_tag(self) -> Tag {
        match self {
            Object::Tag(v) => v,
            _ => panic!("BUG: not a tag"),
        }
    }
    /// Turns this instance into a [`Blob`][Blob] if it is one.
    #[allow(clippy::result_large_err)]
    pub fn try_into_blob(self) -> Result<Blob, Self> {
        match self {
            Object::Blob(v) => Ok(v),
            _ => Err(self),
        }
    }
    /// Turns this instance into a [`BlobRef`][BlobRef] if it is a blob.
    pub fn try_into_blob_ref(&self) -> Option<BlobRef<'_>> {
        match self {
            Object::Blob(v) => Some(v.to_ref()),
            _ => None,
        }
    }
    /// Turns this instance into a [`Commit`][Commit] if it is one.
    #[allow(clippy::result_large_err)]
    pub fn try_into_commit(self) -> Result<Commit, Self> {
        match self {
            Object::Commit(v) => Ok(v),
            _ => Err(self),
        }
    }
    /// Turns this instance into a [`Tree`][Tree] if it is one.
    #[allow(clippy::result_large_err)]
    pub fn try_into_tree(self) -> Result<Tree, Self> {
        match self {
            Object::Tree(v) => Ok(v),
            _ => Err(self),
        }
    }
    /// Turns this instance into a [`Tag`][Tag] if it is one.
    #[allow(clippy::result_large_err)]
    pub fn try_into_tag(self) -> Result<Tag, Self> {
        match self {
            Object::Tag(v) => Ok(v),
            _ => Err(self),
        }
    }

    /// Returns a [`Blob`][Blob] if it is one.
    pub fn as_blob(&self) -> Option<&Blob> {
        match self {
            Object::Blob(v) => Some(v),
            _ => None,
        }
    }
    /// Returns a [`Commit`][Commit] if it is one.
    pub fn as_commit(&self) -> Option<&Commit> {
        match self {
            Object::Commit(v) => Some(v),
            _ => None,
        }
    }
    /// Returns a [`Tree`][Tree] if it is one.
    pub fn as_tree(&self) -> Option<&Tree> {
        match self {
            Object::Tree(v) => Some(v),
            _ => None,
        }
    }
    /// Returns a [`Tag`][Tag] if it is one.
    pub fn as_tag(&self) -> Option<&Tag> {
        match self {
            Object::Tag(v) => Some(v),
            _ => None,
        }
    }
    /// Returns the kind of object stored in this instance.
    pub fn kind(&self) -> crate::Kind {
        match self {
            Object::Tree(_) => crate::Kind::Tree,
            Object::Blob(_) => crate::Kind::Blob,
            Object::Commit(_) => crate::Kind::Commit,
            Object::Tag(_) => crate::Kind::Tag,
        }
    }
}

use crate::{
    decode::{loose_header, Error as DecodeError, LooseHeaderDecodeError},
    BlobRef, CommitRef, Kind, ObjectRef, TagRef, TreeRef,
};

#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum LooseDecodeError {
    #[error(transparent)]
    InvalidHeader(#[from] LooseHeaderDecodeError),
    #[error(transparent)]
    InvalidContent(#[from] DecodeError),
}

impl<'a> ObjectRef<'a> {
    /// Deserialize an object from a loose serialisation
    pub fn from_loose(data: &'a [u8]) -> Result<ObjectRef<'a>, LooseDecodeError> {
        let (kind, size, offset) = loose_header(data)?;

        let body = &data[offset..]
            .get(..size)
            .ok_or(LooseHeaderDecodeError::InvalidHeader {
                message: "object data was shorter than its size declared in the header",
            })?;

        Ok(Self::from_bytes(kind, body)?)
    }

    /// Deserialize an object of `kind` from the given `data`.
    pub fn from_bytes(kind: Kind, data: &'a [u8]) -> Result<ObjectRef<'a>, crate::decode::Error> {
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
