use bstr::BString;

mod error {
    use crate::fetch::refs::parse;

    /// The error returned by [refs()][super::refs()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        Io(#[from] std::io::Error),
        #[error(transparent)]
        Transport(#[from] git_transport::client::Error),
        #[error(transparent)]
        Parse(#[from] parse::Error),
    }
}
pub use error::Error;

///
pub mod parse {
    use bstr::BString;

    /// The error returned when parsing References/refs from the server response.
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        Io(#[from] std::io::Error),
        #[error(transparent)]
        Id(#[from] git_hash::decode::Error),
        #[error("{symref:?} could not be parsed. A symref is expected to look like <NAME>:<target>.")]
        MalformedSymref { symref: BString },
        #[error("{0:?} could not be parsed. A V1 ref line should be '<hex-hash> <path>'.")]
        MalformedV1RefLine(String),
        #[error(
            "{0:?} could not be parsed. A V2 ref line should be '<hex-hash> <path>[ (peeled|symref-target):<value>'."
        )]
        MalformedV2RefLine(String),
        #[error("The ref attribute {attribute:?} is unknown. Found in line {line:?}")]
        UnkownAttribute { attribute: String, line: String },
        #[error("{message}")]
        InvariantViolation { message: &'static str },
    }
}

/// A git reference, commonly referred to as 'ref', as returned by a git server before sending a pack.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Ref {
    /// A ref pointing to a `tag` object, which in turns points to an `object`, usually a commit
    Peeled {
        /// The path at which the ref is located, like `/refs/heads/main`.
        path: BString,
        /// The hash of the tag the ref points to.
        tag: git_hash::ObjectId,
        /// The hash of the object the `tag` points to.
        object: git_hash::ObjectId,
    },
    /// A ref pointing to a commit object
    Direct {
        /// The path at which the ref is located, like `/refs/heads/main`.
        path: BString,
        /// The hash of the object the ref points to.
        object: git_hash::ObjectId,
    },
    /// A symbolic ref pointing to `target` ref, which in turn points to an `object`
    Symbolic {
        /// The path at which the symbolic ref is located, like `/refs/heads/main`.
        path: BString,
        /// The path of the ref the symbolic ref points to, see issue [#205] for details
        ///
        /// [#205]: https://github.com/Byron/gitoxide/issues/205
        target: BString,
        /// The hash of the object the `target` ref points to.
        object: git_hash::ObjectId,
    },
}

impl Ref {
    /// Provide shared fields referring to the ref itself, namely `(path, object id)`.
    /// In case of peeled refs, the tag object itself is returned as it is what the path refers to.
    pub fn unpack(&self) -> (&BString, &git_hash::ObjectId) {
        match self {
            Ref::Direct { path, object, .. }
            | Ref::Peeled { path, tag: object, .. } // the tag acts as reference
            | Ref::Symbolic { path, object, .. } => (path, object),
        }
    }
}

pub(crate) mod function;

#[cfg(any(feature = "blocking-client", feature = "async-client"))]
pub(crate) mod shared;

#[cfg(feature = "async-client")]
mod async_io;
#[cfg(feature = "async-client")]
pub use async_io::{from_v1_refs_received_as_part_of_handshake_and_capabilities, from_v2_refs};

#[cfg(feature = "blocking-client")]
mod blocking_io;
#[cfg(feature = "blocking-client")]
pub use blocking_io::{from_v1_refs_received_as_part_of_handshake_and_capabilities, from_v2_refs};
