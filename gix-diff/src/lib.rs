//! Algorithms for diffing various git object types and for generating patches, highly optimized for performance.
//! ## Feature Flags
#![cfg_attr(
    all(doc, feature = "document-features"),
    doc = ::document_features::document_features!()
)]
#![cfg_attr(all(doc, feature = "document-features"), feature(doc_cfg, doc_auto_cfg))]
#![deny(missing_docs, rust_2018_idioms)]
#![forbid(unsafe_code)]

/// Re-export for use in public API.
#[cfg(feature = "blob")]
pub use gix_command as command;
/// Re-export for use in public API.
#[cfg(feature = "blob")]
pub use gix_object as object;

/// A structure to capture how to perform rename and copy tracking, used by the [rewrites::Tracker].
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg(feature = "blob")]
pub struct Rewrites {
    /// If `Some(…)`, also find copies. `None` is the default which does not try to detect copies at all.
    ///
    /// Note that this is an even more expensive operation than detecting renames stemming from additions and deletions
    /// as the resulting set to search through is usually larger.
    pub copies: Option<rewrites::Copies>,
    /// The percentage of similarity needed for files to be considered renamed, defaulting to `Some(0.5)`.
    /// This field is similar to `git diff -M50%`.
    ///
    /// If `None`, files are only considered equal if their content matches 100%.
    /// Note that values greater than 1.0 have no different effect than 1.0.
    pub percentage: Option<f32>,
    /// The amount of files to consider for fuzzy rename or copy tracking. Defaults to 1000, meaning that only 1000*1000
    /// combinations can be tested for fuzzy matches, i.e. the ones that try to find matches by comparing similarity.
    /// If 0, there is no limit.
    ///
    /// If the limit would not be enough to test the entire set of combinations, the algorithm will trade in precision and not
    /// run the fuzzy version of identity tests at all. That way results are never partial.
    pub limit: usize,
}

/// Contains a [Tracker](rewrites::Tracker) to detect rewrites.
#[cfg(feature = "blob")]
pub mod rewrites;

///
#[allow(clippy::empty_docs)]
pub mod tree;

///
#[cfg(feature = "blob")]
pub mod blob;
