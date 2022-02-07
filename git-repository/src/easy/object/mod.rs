//!
use std::convert::TryInto;

use git_hash::ObjectId;
pub use git_object::Kind;

use crate::{
    easy,
    easy::{Commit, DetachedObject, Object, Tree},
};

mod errors;
pub(crate) mod cache {
    pub use git_pack::cache::object::MemoryCappedHashmap;
}
pub use errors::{conversion, find, write};
///
pub mod commit;
mod impls;
pub mod peel;
///
pub mod tree;

///
pub mod try_into {
    #[derive(thiserror::Error, Debug)]
    #[allow(missing_docs)]
    #[error("Object named {id} was supposed to be of kind {expected}, but was kind {actual}.")]
    pub struct Error {
        pub actual: git_object::Kind,
        pub expected: git_object::Kind,
        pub id: git_hash::ObjectId,
    }
}

impl DetachedObject {
    /// Infuse this owned object with an [`easy::Handle`].
    pub fn attach(self, handle: &easy::Handle) -> Object<'_> {
        Object {
            id: self.id,
            kind: self.kind,
            data: self.data,
            handle,
        }
    }
}

impl<'repo> Object<'repo> {
    pub(crate) fn from_data(id: impl Into<ObjectId>, kind: Kind, data: Vec<u8>, handle: &'repo easy::Handle) -> Self {
        Object {
            id: id.into(),
            kind,
            data,
            handle,
        }
    }

    /// Transform this object into a tree, or panic if it is none.
    pub fn into_tree(self) -> Tree<'repo> {
        match self.try_into() {
            Ok(tree) => tree,
            Err(this) => panic!("Tried to use {} as tree, but was {}", this.id, this.kind),
        }
    }

    /// Transform this object into a commit, or panic if it is none.
    pub fn into_commit(self) -> Commit<'repo> {
        match self.try_into() {
            Ok(commit) => commit,
            Err(this) => panic!("Tried to use {} as commit, but was {}", this.id, this.kind),
        }
    }

    /// Transform this object into a commit, or return it as part of the `Err` if it is no commit.
    pub fn try_into_commit(self) -> Result<Commit<'repo>, try_into::Error> {
        self.try_into().map_err(|this: Self| try_into::Error {
            id: this.id,
            actual: this.kind,
            expected: git_object::Kind::Commit,
        })
    }

    /// Transform this object into a tree, or return it as part of the `Err` if it is no tree.
    pub fn try_into_tree(self) -> Result<Tree<'repo>, try_into::Error> {
        self.try_into().map_err(|this: Self| try_into::Error {
            id: this.id,
            actual: this.kind,
            expected: git_object::Kind::Tree,
        })
    }
}

impl<'repo> Object<'repo> {
    /// Create an owned instance of this object, copying our data in the process.
    pub fn to_owned(&self) -> DetachedObject {
        DetachedObject {
            id: self.id,
            kind: self.kind,
            data: self.data.clone(),
        }
    }

    /// Turn this instance into an owned one, copying our data in the process.
    pub fn into_owned(mut self) -> DetachedObject {
        DetachedObject {
            id: self.id,
            kind: self.kind,
            data: std::mem::take(&mut self.data),
        }
    }

    /// Sever the connection to `Easy` and turn this instance into a standalone object.
    ///
    /// Note that the data buffer will be copied in the process.
    pub fn detach(self) -> DetachedObject {
        self.into()
    }
}

impl<'repo> Object<'repo> {
    /// Obtain a fully parsed commit whose fields reference our data buffer,
    ///
    /// # Panic
    ///
    /// - this object is not a commit
    /// - the commit could not be decoded
    pub fn to_commit_ref(&self) -> git_object::CommitRef<'_> {
        self.try_to_commit_ref().expect("BUG: need a commit")
    }

    /// Obtain a fully parsed commit whose fields reference our data buffer.
    pub fn try_to_commit_ref(&self) -> Result<git_object::CommitRef<'_>, conversion::Error> {
        git_object::Data::new(self.kind, &self.data)
            .decode()?
            .into_commit()
            .ok_or(conversion::Error::UnexpectedType {
                expected: git_object::Kind::Commit,
                actual: self.kind,
            })
    }

    /// Obtain a an iterator over commit tokens like in [`to_commit_iter()`][Object::try_to_commit_ref_iter()].
    ///
    /// # Panic
    ///
    /// - this object is not a commit
    pub fn to_commit_ref_iter(&self) -> git_object::CommitRefIter<'_> {
        git_object::Data::new(self.kind, &self.data)
            .try_into_commit_iter()
            .expect("BUG: This object must be a commit")
    }

    /// Obtain a commit token iterator from the data in this instance, if it is a commit.
    pub fn try_to_commit_ref_iter(&self) -> Option<git_object::CommitRefIter<'_>> {
        git_object::Data::new(self.kind, &self.data).try_into_commit_iter()
    }

    /// Obtain a tag token iterator from the data in this instance.
    ///
    /// # Panic
    ///
    /// - this object is not a tag
    pub fn to_tag_ref_iter(&self) -> git_object::TagRefIter<'_> {
        git_object::Data::new(self.kind, &self.data)
            .try_into_tag_iter()
            .expect("BUG: this object must be a tag")
    }

    /// Obtain a tag token iterator from the data in this instance.
    ///
    /// # Panic
    ///
    /// - this object is not a tag
    pub fn try_to_tag_ref_iter(&self) -> Option<git_object::TagRefIter<'_>> {
        git_object::Data::new(self.kind, &self.data).try_into_tag_iter()
    }

    /// Obtain a tag object from the data in this instance.
    ///
    /// # Panic
    ///
    /// - this object is not a tag
    /// - the tag could not be decoded
    pub fn to_tag_ref(&self) -> git_object::TagRef<'_> {
        self.try_to_tag_ref().expect("BUG: need tag")
    }

    /// Obtain a fully parsed tag object whose fields reference our data buffer.
    pub fn try_to_tag_ref(&self) -> Result<git_object::TagRef<'_>, conversion::Error> {
        git_object::Data::new(self.kind, &self.data)
            .decode()?
            .into_tag()
            .ok_or(conversion::Error::UnexpectedType {
                expected: git_object::Kind::Tag,
                actual: self.kind,
            })
    }
}
