pub const PROGRESS_RANGE: std::ops::RangeInclusive<u8> = 0..=5;

pub struct Engine {
    con: rusqlite::Connection,
    state: engine::State,
}

pub struct RunOutcome {
    /// the relative path to the repositories that could not be found on disk
    pub missing_repos_rela_paths: usize,
}

pub(crate) mod db;
pub mod engine;

/// Contains all information necessary to run a task.
pub(crate) struct Task {
    /// The unique name of the task, which must not be changed after creating it.
    ///
    /// However, if it is changed it will be treated as new kind of task entirely and won't compare
    /// to previous runs of the task.
    short_name: &'static str,
    /// Explain in greater detail what the task is doing.
    description: &'static str,
    /// `true` if the task cannot be run in parallel as it needs all resources by itself.
    execute_exclusive: bool,
    /// The actual implementation
    execute: &'static (dyn run::Execute + Send + Sync),
}

pub(crate) struct Run {
    /// Our own ID for finding the respective database row.
    id: db::Id,
    /// The time at which the run was inserted.
    duration: std::time::Duration,
    error: Option<String>,
}

pub(crate) mod run;
pub(crate) mod trace;
