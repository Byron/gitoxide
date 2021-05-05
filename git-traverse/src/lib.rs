#![forbid(unsafe_code, rust_2018_idioms)]
#![deny(missing_docs)]
//! Various ways to traverse commit graphs and trees with implementations as iterator

/// Commit traversal
pub mod commit;

/// Tree traversal
pub mod tree;
