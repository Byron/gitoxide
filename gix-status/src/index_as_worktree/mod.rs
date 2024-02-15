//! Changes between an index and a worktree.
///
mod types;
pub use types::{Change, Conflict, Context, EntryStatus, Error, Options, Outcome, VisitEntry};

mod recorder;
pub use recorder::{Record, Recorder};

pub(super) mod function;
///
pub mod traits;
