//! Changes between an index and a worktree.
///
mod types;
pub use types::{Change, Error, Options, VisitEntry};

mod recorder;
pub use recorder::Recorder;

///
pub mod content;
pub(crate) mod function;
