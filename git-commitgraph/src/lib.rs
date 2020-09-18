#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]

pub mod graph;
pub mod graph_file;

pub use graph::{Graph, GraphPosition};
pub use graph_file::commit_data::CommitData;

pub const MAX_COMMITS: u32 = (1 << 30) + (1 << 29) + (1 << 28) - 1;
