#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]

mod plumbing;
mod shared;

use anyhow::Result;

#[cfg(feature = "pretty-cli")]
fn main() -> Result<()> {
    plumbing::main()
}

#[cfg(not(feature = "pretty-cli"))]
compile_error!("Please set 'pretty-cli' feature flag");
