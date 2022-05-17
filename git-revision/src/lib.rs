//! Interact with git revisions by parsing them from rev-specs and turning them into rev-specs.
//!
//! One can also describe revisions using a different algorithm.
#![forbid(unsafe_code, rust_2018_idioms)]
#![deny(missing_docs)]

/// Access to collections optimized for keys that are already a hash.
pub use hash_hasher;

///
pub mod describe;
pub mod parser;
pub use describe::function::describe;
