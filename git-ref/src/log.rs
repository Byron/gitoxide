use git_hash::ObjectId;
use git_object::bstr::BString;

/// A parsed ref log line that can be changed
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Line {
    /// The previous object id. Can be a null-sha to indicate this is a line for a new ref.
    pub previous_oid: ObjectId,
    /// The new object id. Can be a null-sha to indicate this ref is being deleted.
    pub new_oid: ObjectId,
    /// The signature of the currently configured committer.
    pub signature: git_actor::Signature,
    /// The message providing details about the operation performed in this log line.
    pub message: BString,
}
