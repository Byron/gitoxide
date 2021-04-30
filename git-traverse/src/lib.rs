#![forbid(unsafe_code, rust_2018_idioms)]
#![deny(missing_docs)]
//! Various ways to traverse commit graphs and trees with implementations as iterator

/// Iterators over various kind of git objects
pub mod iter;
