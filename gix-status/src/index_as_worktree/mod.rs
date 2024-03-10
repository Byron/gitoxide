//! Changes between an index and a worktree.
///
#[allow(clippy::empty_docs)]
mod types;
pub use types::{Change, Conflict, Context, EntryStatus, Error, Options, Outcome, VisitEntry};

mod recorder;
pub use recorder::{Record, Recorder};

pub(super) mod function;
///
#[allow(clippy::empty_docs)]
pub mod traits;
