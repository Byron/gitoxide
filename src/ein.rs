#![deny(rust_2018_idioms, unsafe_code)]

mod porcelain;
mod shared;

use anyhow::Result;

fn main() -> Result<()> {
    porcelain::main()
}

#[cfg(not(feature = "pretty-cli"))]
compile_error!("Please set 'pretty-cli' feature flag");
