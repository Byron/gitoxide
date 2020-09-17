#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]

mod porcelain;

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
