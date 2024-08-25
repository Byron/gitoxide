//! Provides functions to quote and possibly unquote strings with different quoting styles.
#![deny(rust_2018_idioms, missing_docs)]
#![forbid(unsafe_code)]

///
pub mod ansi_c;

mod single;
pub use single::single;
