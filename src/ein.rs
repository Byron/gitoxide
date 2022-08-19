//! ## Feature Flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![deny(rust_2018_idioms)]
#![forbid(unsafe_code)]

mod porcelain;
mod shared;

use anyhow::Result;

fn main() -> Result<()> {
    porcelain::main()
}

#[cfg(not(feature = "pretty-cli"))]
compile_error!("Please set 'pretty-cli' feature flag");
