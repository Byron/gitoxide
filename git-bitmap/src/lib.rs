#![deny(unsafe_code, missing_docs, rust_2018_idioms)]
#![allow(missing_docs, dead_code)]
//! An implementation of the shared parts of git bitmaps used in `git-pack`, `git-index` and `git-worktree`.
//!
//! Note that many tests are performed indirectly by tests in the aforementioned consumer crates.
