#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]

pub mod file;
pub mod graph;

pub use file::commit::Commit;
pub use graph::{Graph, GraphPosition};

pub const MAX_COMMITS: u32 = (1 << 30) + (1 << 29) + (1 << 28) - 1;
