#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]

mod plumbing;
mod shared;

use anyhow::Result;

fn main() -> Result<()> {
    plumbing::main()
}
