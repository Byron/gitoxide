#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]

mod porcelain;
mod shared;

use anyhow::Result;

fn main() -> Result<()> {
    porcelain::main()
}
