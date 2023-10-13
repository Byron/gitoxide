//! This is the documentation of the binaries that come with `gitoxide`. These are called `gix` and `ein`.
//!
//! #### `gix`
//!
//! A developer tool to allow using `gitoxide` algorithms and functionality outside of the test suite. It will be unstable as long as
//! the `gix` crate is unstable and is explicitly not to be understood as `git` replacement.
//!
//! #### `ein`
//!
//! A program to eventually become the most convenient way to do typical operations on `git` repositories, with all tooling one typically
//! needs built right into it.
//! For now, it's most useful for its assorted set of `tools` which help to build automations or learn something about `git` repositories.
//!
//! ## Feature Flags
//!
//! Feature configuration can be complex and this document seeks to provide an overview.
//!
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![deny(rust_2018_idioms)]
#![allow(missing_docs)]
#![forbid(unsafe_code)]

/// everything in common beteween the `gix` and `ein` binaries.
pub mod shared;
