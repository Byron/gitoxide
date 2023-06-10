//! This crate provides ways of identifying an actor within the git repository both in shared and mutable variants.
//!
//! ## Feature Flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![deny(missing_docs, rust_2018_idioms)]
#![forbid(unsafe_code)]

/// The re-exported `bstr` crate.
///
/// For convenience to allow using `bstr` without adding it to own cargo manifest.
pub use bstr;
use bstr::{BStr, BString};
/// The re-exported `gix-date` crate.
///
/// For convenience to allow using `gix-date` without adding it to own cargo manifest.
pub use gix_date as date;
use gix_date::Time;

mod identity;
///
pub mod signature;

/// A person with name and email.
#[derive(Default, PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Identity {
    /// The actors name.
    pub name: BString,
    /// The actor's email.
    pub email: BString,
}

/// A person with name and email, as reference.
#[derive(Default, PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdentityRef<'a> {
    /// The actors name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub name: &'a BStr,
    /// The actor's email.
    pub email: &'a BStr,
}

/// A mutable signature is created by an actor at a certain time.
///
/// Note that this is not a cryptographical signature.
#[derive(Default, PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[derive(Default, PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SignatureRef<'a> {
    /// The actor's name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub name: &'a BStr,
    /// The actor's email.
    pub email: &'a BStr,
    /// The time stamp at which the signature was performed.
    pub time: gix_date::Time,
}
