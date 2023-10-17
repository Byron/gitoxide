//! The purpose of this crate is to abstract the user interface of `gix` (the command-line interface) from the actual implementation.
//! That way, one day it's possible to provide alternative frontends, including user interfaces.
//!
//! ### What is `gix`?
//!
//! `gix` is a tool to aid developers of `gitoxide` run their code in real-world situations and to validate the `gix` API.
//! This makes it more of a test-bed than a tool that could ever rival `git` in terms of feature-set.
//!
//! That said, `gix` may actively carve out a niche for itself where it sees the greatest benefits for users of `git`.
//!
//! ### This crate is internal - use `gix` instead.
//!
//! It's important to understand that this crate consider itself an implementation detail of the `gix` CLI and is not meant to be
//! used for external consumption by means of `cargo` dependency. This is emphasized by there being no other documentation.
//! There is also no intention of ever stabilizing this crate.
//!
//! If you want to get started with what powers `gix`, please take a look at the `gix` crate which provides all the building
//! blocks to create any application, including a carbon-copy of `git` itself (at least aspirationally as not all capabilities are
//! available in `gix` yet).
//!
//! For users of `gix`, this codebase might serve as elaborate example as most of not all of its APIs are used here.
//!
//! ## Feature Flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![cfg_attr(feature = "async-client", allow(unused))]
#![deny(rust_2018_idioms)]
#![forbid(unsafe_code)]

use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum OutputFormat {
    Human,
    #[cfg(feature = "serde")]
    Json,
}

impl OutputFormat {
    pub fn variants() -> &'static [&'static str] {
        &[
            "human",
            #[cfg(feature = "serde")]
            "json",
        ]
    }
}

impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s_lc = s.to_ascii_lowercase();
        Ok(match s_lc.as_str() {
            "human" => OutputFormat::Human,
            #[cfg(feature = "serde")]
            "json" => OutputFormat::Json,
            _ => return Err(format!("Invalid output format: '{s}'")),
        })
    }
}

pub mod commitgraph;
#[cfg(feature = "corpus")]
pub mod corpus;
pub mod net;

#[cfg(feature = "estimate-hours")]
pub mod hours;
pub mod index;
pub mod mailmap;
#[cfg(feature = "organize")]
pub mod organize;
pub mod pack;
#[cfg(feature = "query")]
pub mod query;
pub mod repository;

mod discover;
pub use discover::discover;

#[cfg(all(feature = "async-client", feature = "blocking-client"))]
compile_error!("Cannot set both 'blocking-client' and 'async-client' features as they are mutually exclusive");
