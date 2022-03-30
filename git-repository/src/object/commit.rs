use crate::{bstr::BStr, Commit, DetachedObject, Tree};

mod error {
    use crate::object;

    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        FindExistingObject(#[from] object::find::existing::OdbError),
        #[error("The commit could not be decoded fully or partially")]
        Decode,
        #[error("Expected object of type {}, but got {}", .expected, .actual)]
        ObjectKind {
            expected: git_object::Kind,
            actual: git_object::Kind,
        },
    }
}

use crate::id::Ancestors;

pub use error::Error;

impl<'repo> Commit<'repo> {
    /// Create an owned instance of this object, copying our data in the process.
    pub fn detached(&self) -> DetachedObject {
        DetachedObject {
            id: self.id,
            kind: git_object::Kind::Commit,
            data: self.data.clone(),
        }
    }

    /// Sever the connection to the `Repository` and turn this instance into a standalone object.
    pub fn detach(self) -> DetachedObject {
        self.into()
    }
}

impl<'repo> Commit<'repo> {
    /// Turn this objects id into a shortened id with a length in hex as configured by `core.abbrev`.
    pub fn short_id(&self) -> Result<git_hash::Prefix, crate::id::prefix::Error> {
        use crate::ext::ObjectIdExt;
        self.id.attach(self.repo).prefix()
    }

    /// Parse the commits message into a [`MessageRef`][git_object::commit::MessageRef]
    pub fn message(&self) -> Result<git_object::commit::MessageRef<'_>, Error> {
        Ok(git_object::commit::MessageRef::from_bytes(self.message_raw()?))
    }
    /// Decode the entire commit object in full and return the raw message bytes.
    pub fn message_raw(&self) -> Result<&'_ BStr, Error> {
        git_object::CommitRefIter::from_bytes(&self.data)
            .message()
            .ok_or(Error::Decode)
    }
    /// Decode the commit and obtain the time at which the commit was created.
    ///
    /// For the time at which it was authored, refer to `.decode()?.author.time`.
    pub fn time(&self) -> Result<git_actor::Time, Error> {
        Ok(self.committer()?.time)
    }

    /// Decode the entire commit object and return it for accessing all commit information.
    ///
    /// It will allocate only if there are more than 2 parents.
    ///
    /// Note that the returned commit object does make lookup easy and should be
    /// used for successive calls to string-ish information to avoid decoding the object
    /// more than once.
    pub fn decode(&self) -> Result<git_object::CommitRef<'_>, git_object::decode::Error> {
        git_object::CommitRef::from_bytes(&self.data)
    }

    /// Return an iterator over tokens, representing this commit piece by piece.
    pub fn iter(&self) -> git_object::CommitRefIter<'_> {
        git_object::CommitRefIter::from_bytes(&self.data)
    }

    /// Return the commits author.
    pub fn author(&self) -> Result<git_actor::SignatureRef<'_>, Error> {
        git_object::CommitRefIter::from_bytes(&self.data)
            .author()
            .ok_or(Error::Decode)
    }

    /// Return the commits committer.
    pub fn committer(&self) -> Result<git_actor::SignatureRef<'_>, Error> {
        git_object::CommitRefIter::from_bytes(&self.data)
            .committer()
            .ok_or(Error::Decode)
    }

    /// Decode this commits parent ids on the fly without allocating.
    // TODO: tests
    pub fn parent_ids(&self) -> impl Iterator<Item = crate::Id<'repo>> + '_ {
        use crate::ext::ObjectIdExt;
        let repo = self.repo;
        git_object::CommitRefIter::from_bytes(&self.data)
            .parent_ids()
            .map(move |id| id.attach(repo))
    }

    /// Parse the commit and return the the tree object it points to.
    pub fn tree(&self) -> Result<Tree<'repo>, Error> {
        let tree_id = self.tree_id().ok_or(Error::Decode)?;
        match self.repo.find_object(tree_id)?.try_into_tree() {
            Ok(tree) => Ok(tree),
            Err(crate::object::try_into::Error { actual, expected, .. }) => Err(Error::ObjectKind { actual, expected }),
        }
    }

    /// Parse the commit and return the the tree id it points to.
    pub fn tree_id(&self) -> Option<git_hash::ObjectId> {
        git_object::CommitRefIter::from_bytes(&self.data).tree_id()
    }

    /// Return our id own id with connection to this repository.
    pub fn id(&self) -> crate::Id<'repo> {
        use crate::ext::ObjectIdExt;
        self.id.attach(self.repo)
    }

    /// Obtain a platform for traversing ancestors of this commit.
    pub fn ancestors(&self) -> Ancestors<'repo> {
        self.id().ancestors()
    }
}
