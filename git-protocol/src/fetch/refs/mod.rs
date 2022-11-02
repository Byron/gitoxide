use bstr::{BStr, BString};

mod error {
    use crate::fetch::refs::parse;

    /// The error returned by [refs()][crate::fetch::refs()].
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
        /// The name at which the ref is located, like `refs/tags/1.0`.
        full_ref_name: BString,
        /// The hash of the tag the ref points to.
        tag: git_hash::ObjectId,
        /// The hash of the object the `tag` points to.
        object: git_hash::ObjectId,
    },
    /// A ref pointing to a commit object
    Direct {
        /// The name at which the ref is located, like `refs/heads/main`.
        full_ref_name: BString,
        /// The hash of the object the ref points to.
        object: git_hash::ObjectId,
    },
    /// A symbolic ref pointing to `target` ref, which in turn points to an `object`
    Symbolic {
        /// The name at which the symbolic ref is located, like `HEAD`.
        full_ref_name: BString,
        /// The path of the ref the symbolic ref points to, like `refs/heads/main`.
        ///
        /// See issue [#205] for details
        ///
        /// [#205]: https://github.com/Byron/gitoxide/issues/205
        target: BString,
        /// The hash of the object the `target` ref points to.
        object: git_hash::ObjectId,
    },
    /// `HEAD` is unborn on the remote and just points to the initial, unborn branch.
    Unborn {
        /// The path of the ref the symbolic ref points to, like `refs/heads/main`.
        target: BString,
    },
}

impl Ref {
    /// Provide shared fields referring to the ref itself, namely `(name, target, [peeled])`.
    /// In case of peeled refs, the tag object itself is returned as it is what the ref directly refers to, and target of the tag is returned
    /// as `peeled`.
    /// If `unborn`, the first object id will be the null oid.
    pub fn unpack(&self) -> (&BStr, Option<&git_hash::oid>, Option<&git_hash::oid>) {
        match self {
            Ref::Direct { full_ref_name, object }
            | Ref::Symbolic {
                full_ref_name, object, ..
            } => (full_ref_name.as_ref(), Some(object), None),
            Ref::Peeled {
                full_ref_name,
                tag: object,
                object: peeled,
            } => (full_ref_name.as_ref(), Some(object), Some(peeled)),
            Ref::Unborn { target: _ } => ("HEAD".into(), None, None),
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
