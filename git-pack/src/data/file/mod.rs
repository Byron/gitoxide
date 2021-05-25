///
pub mod decode_entry;
mod init;
///
pub mod verify;

/// A return value of a resolve function, which given an [`ObjectId`][git_hash::ObjectId] determines where an object can be found.
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum ResolvedBase {
    /// Indicate an object is within this pack, at the given entry, and thus can be looked up locally.
    InPack(crate::data::Entry),
    /// Indicates the object of `kind` was found outside of the pack, and its data was written into an output
    /// vector which now has a length of `end`.
    #[allow(missing_docs)]
    OutOfPack { kind: git_object::Kind, end: usize },
}
