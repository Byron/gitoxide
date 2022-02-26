use crate::bstr::BStr;
use crate::{Commit, Tree};

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

pub use error::Error;

impl<'repo> Commit<'repo> {
    /// Parse the commits message into a [`MessageRef`][git_object::commit::MessageRef], after decoding the entire commit object.
    pub fn message(&self) -> Result<git_object::commit::MessageRef<'_>, git_object::decode::Error> {
        Ok(self.decode()?.message())
    }
    /// Decode the entire commit object in full and return the raw message bytes.
    pub fn message_raw(&self) -> Result<&'_ BStr, git_object::decode::Error> {
        Ok(self.decode()?.message)
    }
    /// Decode the commit and obtain the time at which the commit was created.
    ///
    /// For the time at which it was authored, refer to `.decode()?.author.time`.
    pub fn time(&self) -> Result<git_actor::Time, git_object::decode::Error> {
        Ok(self.decode()?.committer.time)
    }

    /// Decode the entire commit object and return it for accessing all commit information.
    ///
    /// Note that the returned commit object doesn't make object lookup easy but should be
    /// used for successive calls to string-ish information to avoid decoding the object
    /// unnecessarily.
    pub fn decode(&self) -> Result<git_object::CommitRef<'_>, git_object::decode::Error> {
        git_object::CommitRef::from_bytes(&self.data)
    }

    /// Parse the commit and return the the tree object it points to.
    pub fn tree(&self) -> Result<Tree<'repo>, Error> {
        let tree_id = self.tree_id().ok_or(Error::Decode)?;
        match self.handle.find_object(tree_id)?.try_into_tree() {
            Ok(tree) => Ok(tree),
            Err(crate::object::try_into::Error { actual, expected, .. }) => Err(Error::ObjectKind { actual, expected }),
        }
    }

    /// Parse the commit and return the the tree id it points to.
    pub fn tree_id(&self) -> Option<git_hash::ObjectId> {
        git_object::CommitRefIter::from_bytes(&self.data).tree_id()
    }
}
