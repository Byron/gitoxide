//! The `gitoxide` plumbing.
//! ## Feature Flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![deny(unsafe_code, rust_2018_idioms)]

mod plumbing;
mod shared;

use anyhow::Result;

#[cfg(feature = "pretty-cli")]
fn main() -> Result<()> {
    plumbing::main()
}

#[cfg(not(feature = "pretty-cli"))]
compile_error!("Please set 'pretty-cli' feature flag");
