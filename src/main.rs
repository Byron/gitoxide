#![forbid(unsafe_code)]

#[cfg(feature = "pretty-cli")]
mod pretty;

#[cfg(all(feature = "lean-cli", not(feature = "pretty-cli")))]
mod lean;

use anyhow::Result;

#[cfg(feature = "pretty-cli")]
fn main() -> Result<()> {
    pretty::main()
}

#[cfg(all(feature = "lean-cli", not(feature = "pretty-cli")))]
fn main() -> Result<()> {
    lean::main()
}
