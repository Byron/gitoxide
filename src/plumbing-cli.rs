#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]

mod plumbing;
mod shared;

use anyhow::Result;

#[cfg(feature = "pretty-cli")]
fn main() -> Result<()> {
    plumbing::pretty::main()
}

#[cfg(all(feature = "lean-cli", not(feature = "pretty-cli")))]
fn main() -> Result<()> {
    plumbing::lean::main()
}

#[cfg(not(any(feature = "pretty-cli", feature = "lean-cli")))]
compile_error!("Please set 'lean-cli' or 'pretty-cli' feature flags");
