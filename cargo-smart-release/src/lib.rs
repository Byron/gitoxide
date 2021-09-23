pub struct ChangeLog {
    _segments: Vec<changelog::Segment>,
}

pub mod changelog;
pub mod command;
pub mod commit;

mod context;
pub use context::Context;

pub mod git;
pub mod traverse;
mod utils;
