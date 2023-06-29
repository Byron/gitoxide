//! Date and time parsing similar to what git can do.
//!
//! Note that this is not a general purpose time library.
//! ## Feature Flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![deny(missing_docs, rust_2018_idioms)]
#![forbid(unsafe_code)]

///
pub mod time;

///
pub mod parse;
pub use parse::function::parse;

/// A timestamp with timezone.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Time {
    /// time in seconds since epoch.
    pub seconds: SecondsSinceUnixEpoch,
    /// time offset in seconds, may be negative to match the `sign` field.
    pub offset: OffsetInSeconds,
    /// the sign of `offset`, used to encode `-0000` which would otherwise loose sign information.
    pub sign: time::Sign,
}

/// The amount of seconds since unix epoch.
///
/// Note that negative dates represent times before the unix epoch.
///
/// ### Deviation
///
/// `git` only supports dates *from* the UNIX epoch, whereas we chose to be more flexible at the expense of stopping time
/// a few million years before the heat-death of the universe.
pub type SecondsSinceUnixEpoch = i64;
/// time offset in seconds.
pub type OffsetInSeconds = i32;
