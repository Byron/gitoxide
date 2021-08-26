//! This crate provides types for [read-only git objects][crate::ObjectRef] backed by bytes provided in git's serialization format
//! as well as [mutable versions][Object] of these. The latter can be serialized into git's serialization format for objects.
#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, missing_docs)]

use std::borrow::Cow;

/// For convenience to allow using `bstr` without adding it to own cargo manifest.
pub use bstr;
use bstr::{BStr, BString, ByteSlice};
use smallvec::SmallVec;
use tree::Entry;
pub use types::{Error, Kind};

use crate::tree::EntryRef;

pub mod immutable;
pub mod mutable;

///
pub mod commit;
///
pub mod tag;
///
pub mod tree;

mod blob;

mod encode;
mod types;

/// A chunk of any [`data`][BlobRef::data].
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct BlobRef<'a> {
    /// The bytes themselves.
    pub data: &'a [u8],
}

/// A mutable chunk of any [`data`][Blob::data].
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Blob {
    /// The data itself.
    pub data: Vec<u8>,
}

/// A git commit parsed using [`from_bytes()`][CommitRef::from_bytes()].
///
/// A commit encapsulates information about a point in time at which the state of the repository is recorded, usually after a
/// change which is documented in the commit `message`.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct CommitRef<'a> {
    /// HEX hash of tree object we point to. Usually 40 bytes long.
    ///
    /// Use [`tree()`][CommitRef::tree()] to obtain a decoded version of it.
    #[cfg_attr(feature = "serde1", serde(borrow))]
    pub tree: &'a BStr,
    /// HEX hash of each parent commit. Empty for first commit in repository.
    pub parents: SmallVec<[&'a BStr; 2]>,
    /// Who wrote this commit.
    pub author: git_actor::SignatureRef<'a>,
    /// Who committed this commit.
    ///
    /// This may be different from the `author` in case the author couldn't write to the repository themselves and
    /// is commonly encountered with contributed commits.
    pub committer: git_actor::SignatureRef<'a>,
    /// The name of the message encoding, otherwise [UTF-8 should be assumed](https://github.com/git/git/blob/e67fbf927dfdf13d0b21dc6ea15dc3c7ef448ea0/commit.c#L1493:L1493).
    pub encoding: Option<&'a BStr>,
    /// The commit message documenting the change.
    pub message: &'a BStr,
    /// Extra header fields, in order of them being encountered, made accessible with the iterator returned by [`extra_headers()`][CommitRef::extra_headers()].
    pub extra_headers: Vec<(&'a BStr, Cow<'a, BStr>)>,
}

/// A mutable git commit, representing an annotated state of a working tree along with a reference to its historical commits.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Commit {
    /// The hash of recorded working tree state.
    pub tree: git_hash::ObjectId,
    /// Hash of each parent commit. Empty for the first commit in repository.
    pub parents: SmallVec<[git_hash::ObjectId; 1]>,
    /// Who wrote this commit.
    pub author: git_actor::Signature,
    /// Who committed this commit.
    ///
    /// This may be different from the `author` in case the author couldn't write to the repository themselves and
    /// is commonly encountered with contributed commits.
    pub committer: git_actor::Signature,
    /// The name of the message encoding, otherwise [UTF-8 should be assumed](https://github.com/git/git/blob/e67fbf927dfdf13d0b21dc6ea15dc3c7ef448ea0/commit.c#L1493:L1493).
    pub encoding: Option<BString>,
    /// The commit message documenting the change.
    pub message: BString,
    /// Extra header fields, in order of them being encountered, made accessible with the iterator returned
    /// by [`extra_headers()`][Commit::extra_headers()].
    pub extra_headers: Vec<(BString, BString)>,
}

/// Represents a git tag, commonly indicating a software release.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct TagRef<'a> {
    /// The hash in hexadecimal being the object this tag points to. Use [`target()`][TagRef::target()] to obtain a byte representation.
    #[cfg_attr(feature = "serde1", serde(borrow))]
    pub target: &'a BStr,
    /// The kind of object that `target` points to.
    pub target_kind: crate::Kind,
    /// The name of the tag, e.g. "v1.0".
    pub name: &'a BStr,
    /// The author of the tag.
    pub tagger: Option<git_actor::SignatureRef<'a>>,
    /// The message describing this release.
    pub message: &'a BStr,
    /// A cryptographic signature over the entire content of the serialized tag object thus far.
    pub pgp_signature: Option<&'a BStr>,
}

/// Like [`TagRef`], but as `Iterator` to support entirely allocation free parsing.
/// It's particularly useful to dereference only the target chain.
pub struct TagRefIter<'a> {
    data: &'a [u8],
    state: tag::ref_iter::State,
}

/// A mutable git tag.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Tag {
    /// The hash this tag is pointing to.
    pub target: git_hash::ObjectId,
    /// The kind of object this tag is pointing to.
    pub target_kind: crate::Kind,
    /// The name of the tag, e.g. "v1.0".
    pub name: BString,
    /// The message describing the tag.
    pub message: BString,
    /// The tags author.
    pub signature: Option<git_actor::Signature>,
    /// A pgp signature over all bytes of the encoded tag, excluding the pgp signature itself.
    pub pgp_signature: Option<BString>,
}

/// An signature_ref object representing [`Trees`][TreeRef], [`Blobs`][BlobRef], [`Commits`][CommitRef], or [`Tags`][TagRef].
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
#[allow(missing_docs)]
pub enum ObjectRef<'a> {
    #[cfg_attr(feature = "serde1", serde(borrow))]
    Tree(TreeRef<'a>),
    Blob(BlobRef<'a>),
    Commit(CommitRef<'a>),
    Tag(TagRef<'a>),
}

/// A mutable object representing [`Trees`][Tree], [`Blobs`][Blob], [`Commits`][Commit] or [`Tags`][Tag].
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::large_enum_variant, missing_docs)]
pub enum Object {
    Tree(Tree),
    Blob(Blob),
    Commit(Commit),
    Tag(Tag),
}
/// A directory snapshot containing files (blobs), directories (trees) and submodules (commits).
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct TreeRef<'a> {
    /// The directories and files contained in this tree.
    #[cfg_attr(feature = "serde1", serde(borrow))]
    pub entries: Vec<EntryRef<'a>>,
}

/// A directory snapshot containing files (blobs), directories (trees) and submodules (commits), lazily evaluated.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct TreeRefIter<'a> {
    /// The directories and files contained in this tree.
    #[cfg_attr(feature = "serde1", serde(borrow))]
    data: &'a [u8],
}

/// A mutable Tree, containing other trees, blobs or commits.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Tree {
    /// The directories and files contained in this tree. They must be and remain sorted by [`filename`][Entry::filename].
    pub entries: Vec<Entry>,
}
