#![forbid(unsafe_code)]
#![deny(missing_docs, rust_2018_idioms)]
#![allow(dead_code)]
//! This crate aims to parse all `git` configuration files found in the wild and apply the same restrictions by default.
//! It will do so while providing helpful error messages in case of syntax errors.
//!
//! Editing is fully supported and non-destructive.
//!
//! Additionally it's a stated goal as well to apply such restrictions only when values are read and optionally allow
//! a less limited character set. This opens up the git configuration format to other languages than English.

///
pub mod file;
pub use file::File;

pub(crate) mod parse;

/// A module with specialized value types as they exist within git config files.
pub mod value;

/// Spanned items refer to their content using [`Span`]s, thus they act like a pointer into a byte buffer representing the config file.
///
/// These are inherently read-only, as they do not own any data but rather point to a buffer they don't even know.
mod spanned;

/// Owned versions of what can be found in `spanned`, which allows these items to be altered.
///
/// All of these will *may* remember their originating `span` as `Some(…)`, which is the entire region in the config file they point to. This is important
/// in case of updates. New owned items thus don't have a `span`, represented by `None`.
pub mod owned;

/// Borrowed items are nothing more than a fancy 'handle' to an item stored in a file, which can be made editable to make updates.
pub mod borrowed;

mod decode;
