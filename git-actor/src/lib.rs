//! This crate provides ways of identifying an actor within the git repository both in shared/mutable and mutable variants.
//!
//! ## Feature Flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, missing_docs)]
use bstr::{BStr, BString};

///
pub mod signature;

const SPACE: &[u8; 1] = b" ";

/// A mutable signature is created by an actor at a certain time.
///
/// Note that this is not a cryptographical signature.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Signature {
    /// The actors name.
    pub name: BString,
    /// The actor's email.
    pub email: BString,
    /// The time stamp at which the signature is performed.
    pub time: Time,
}

/// A immutable signature is created by an actor at a certain time.
///
/// Note that this is not a cryptographical signature.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct SignatureRef<'a> {
    /// The actor's name.
    #[cfg_attr(feature = "serde1", serde(borrow))]
    pub name: &'a BStr,
    /// The actor's email.
    pub email: &'a BStr,
    /// The time stamp at which the signature was performed.
    pub time: Time,
}

/// Indicates if a number is positive or negative for use in [`Time`].
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
#[allow(missing_docs)]
pub enum Sign {
    Plus,
    Minus,
}

/// A timestamp with timezone.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Time {
    /// time in seconds from epoch.
    pub time: u32,
    /// time offset in seconds, may be negative to match the `sign` field.
    pub offset: i32,
    /// the sign of `offset`, used to encode `-0000` which would otherwise loose sign information.
    pub sign: Sign,
}

mod time;
