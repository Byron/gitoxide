//! An experiment to see how a multi-call binary could look like.
//! For CI this would mean longer compile times though as it rebuilds `gix`
//! with varying compile flags, which also means that it recompiles all source or `ein`.
//!
//! However, doing this could be interesting for distribution if the files are hard-linked
//! instead of copied, which is why it is left here.
#![deny(unsafe_code, rust_2018_idioms)]

use anyhow::{bail, Result};

mod plumbing;
mod porcelain;

#[cfg(feature = "pretty-cli")]
fn main() -> Result<()> {
    match std::env::current_exe()?
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("gix")
    {
        "gix" => plumbing::main(),
        "ein" => porcelain::main(),
        unknown => bail!("Executable named '{unknown}' cannot be launched. Exe must be named either `gix` or `ein`."),
    }
}

#[cfg(not(feature = "pretty-cli"))]
compile_error!("Please set 'pretty-cli' feature flag");
