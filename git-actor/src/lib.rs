//! This crate provides ways of identifying an actor within the git repository both in shared/immutable and mutable variants.
#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, missing_docs)]
pub mod immutable;
pub mod mutable;

mod types;
pub use types::{Sign, Time};
