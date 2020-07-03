#![forbid(unsafe_code)]

mod porcelain;

#[cfg(feature = "lean-cli")]
mod shared;

use anyhow::Result;

#[cfg(feature = "pretty-cli")]
fn main() -> Result<()> {
    porcelain::pretty::main()
}

#[cfg(all(feature = "lean-cli", not(feature = "pretty-cli")))]
fn main() -> Result<()> {
    porcelain::lean::main()
}
