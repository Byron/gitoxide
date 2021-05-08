#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]

#[cfg(all(not(feature = "lean-cli"), feature = "pretty-cli"))]
mod porcelain;
#[cfg(all(not(feature = "lean-cli"), feature = "pretty-cli"))]
mod shared;

use anyhow::Result;

#[cfg(all(not(feature = "lean-cli"), feature = "pretty-cli"))]
fn main() -> Result<()> {
    porcelain::main()
}

#[cfg(feature = "lean-cli")]
fn main() -> Result<()> {
    eprintln!("There is no lean version of the porcelain CLI. Please build with the 'pretty-cli' feature flag.");
    Ok(())
}
