#![forbid(unsafe_code)]

mod porcelain;

use anyhow::Result;

#[cfg(feature = "pretty-cli")]
fn main() -> Result<()> {
    porcelain::pretty::main()
}

#[cfg(all(feature = "lean-cli", not(feature = "pretty-cli")))]
fn main() -> Result<()> {
    porcelain::lean::main()
}
