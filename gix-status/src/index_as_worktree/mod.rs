//! Changes between an index and a worktree.
///
mod types;
pub use types::{Change, Error, Options, VisitEntry};

mod recorder;
pub use recorder::{Record, Recorder};

pub(crate) mod function;
///
pub mod traits;
