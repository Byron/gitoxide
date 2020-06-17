#![forbid(unsafe_code)]

pub mod borrowed;
mod types;

#[cfg(test)]
mod tests;

pub use types::*;
