#![forbid(unsafe_code)]

mod plumbing;
#[cfg(feature = "lean-cli")]
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
