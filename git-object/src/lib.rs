//! This crate provides types for [read-only git objects][crate::ObjectRef] backed by bytes provided in git's serialization format
//! as well as [mutable versions][Object] of these. Both types of objects can be encoded.
//! ## Feature Flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, missing_docs)]

use std::borrow::Cow;

/// For convenience to allow using `bstr` without adding it to own cargo manifest.
pub use bstr;
use bstr::{BStr, BString, ByteSlice};
use smallvec::SmallVec;

///
pub mod commit;
mod object;
///
pub mod tag;
///
pub mod tree;

mod blob;
///
pub mod data;

mod traits;
pub use traits::WriteTo;

pub mod encode;
pub(crate) mod parse;

///
pub mod kind;

/// The four types of objects that git differentiates. #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[allow(missing_docs)]
pub enum Kind {
    Tree,
    Blob,
    Commit,
    Tag,
}
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

/// Like [`CommitRef`][crate::CommitRef], but as `Iterator` to support (up to) entirely allocation free parsing.
/// It's particularly useful to traverse the commit graph without ever allocating arrays for parents.
pub struct CommitRefIter<'a> {
    data: &'a [u8],
    state: commit::ref_iter::State,
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
    /// The tags author.
    pub tagger: Option<git_actor::Signature>,
    /// The message describing the tag.
    pub message: BString,
    /// A pgp signature over all bytes of the encoded tag, excluding the pgp signature itself.
    pub pgp_signature: Option<BString>,
}

/// Immutable objects are read-only structures referencing most data from [a byte slice][crate::ObjectRef::from_bytes()].
///
/// Immutable objects are expected to be deserialized from bytes that acts as backing store, and they
/// cannot be mutated or serialized. Instead, one will [convert][crate::ObjectRef::into_owned()] them into their [`mutable`][Object] counterparts
/// which support mutation and serialization.
///
/// An `ObjectRef` is representing [`Trees`][TreeRef], [`Blobs`][BlobRef], [`Commits`][CommitRef], or [`Tags`][TagRef].
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

/// Mutable objects with each field being separately allocated and changeable.
///
/// Mutable objects are Commits, Trees, Blobs and Tags that can be changed and serialized.
///
/// They either created using object [construction][Object] or by [deserializing existing objects][ObjectRef::from_bytes()]
/// and converting these [into mutable copies][ObjectRef::into_owned()] for adjustments.
///
/// An `Object` is representing [`Trees`][Tree], [`Blobs`][Blob], [`Commits`][Commit] or [`Tags`][Tag].
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
    pub entries: Vec<tree::EntryRef<'a>>,
}

/// A directory snapshot containing files (blobs), directories (trees) and submodules (commits), lazily evaluated.
#[derive(Default, PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
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
    /// The directories and files contained in this tree. They must be and remain sorted by [`filename`][tree::Entry::filename].
    pub entries: Vec<tree::Entry>,
}

impl Tree {
    /// Return an empty tree which serializes to a well-known hash
    pub fn empty() -> Self {
        Tree { entries: Vec::new() }
    }
}

/// A borrowed object using a slice as backing buffer, or in other words a bytes buffer that knows the kind of object it represents.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub struct Data<'a> {
    /// kind of object
    pub kind: Kind,
    /// decoded, decompressed data, owned by a backing store.
    pub data: &'a [u8],
}

///
pub mod decode {
    #[cfg(feature = "verbose-object-parsing-errors")]
    mod _decode {
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
    }

    ///
    #[cfg(not(feature = "verbose-object-parsing-errors"))]
    mod _decode {
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
    }
    pub use _decode::{Error, ParseError, ParseErrorOwned};
    impl std::error::Error for Error {}

    use quick_error::quick_error;
    quick_error! {
        /// Returned by [`loose_header()`]
        #[derive(Debug)]
        #[allow(missing_docs)]
        pub enum LooseHeaderDecodeError {
            ParseIntegerError(
                source: btoi::ParseIntegerError,
                message: &'static str,
                number: Vec<u8>
            ) {
                display("{}: {:?}", message, std::str::from_utf8(number))
            }
            InvalidHeader(s: &'static str) {
                display("{}", s)
            }
            ObjectHeader(err: super::kind::Error) { from() }
        }
    }

    use bstr::ByteSlice;
    /// Decode a loose object header, being `<kind> <size>\0`, returns
    /// ([`kind`](super::Kind), `size`, `consumed bytes`).
    ///
    /// `size` is the uncompressed size of the payload in bytes.
    pub fn loose_header(input: &[u8]) -> Result<(super::Kind, usize, usize), LooseHeaderDecodeError> {
        use LooseHeaderDecodeError::*;
        let kind_end = input.find_byte(0x20).ok_or(InvalidHeader("Expected '<type> <size>'"))?;
        let kind = super::Kind::from_bytes(&input[..kind_end])?;
        let size_end = input
            .find_byte(0x0)
            .ok_or(InvalidHeader("Did not find 0 byte in header"))?;
        let size_bytes = &input[kind_end + 1..size_end];
        let size = btoi::btoi(size_bytes).map_err(|source| {
            ParseIntegerError(
                source,
                "Object size in header could not be parsed",
                size_bytes.to_owned(),
            )
        })?;
        Ok((kind, size, size_end + 1))
    }
}
