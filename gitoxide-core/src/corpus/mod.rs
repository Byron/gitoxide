pub const PROGRESS_RANGE: std::ops::RangeInclusive<u8> = 0..=3;
pub(crate) type Progress = gix::progress::DoOrDiscard<gix::progress::prodash::tree::Item>;

pub struct Engine {
    progress: Progress,
    con: rusqlite::Connection,
    gitoxide_version: String,
}

pub struct RunOutcome {
    /// the relative path to the repositories that could not be found on disk
    pub missing_repos_rela_paths: usize,
}

pub(crate) mod db;
pub(crate) mod engine;

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

pub(crate) mod run {
    use crate::corpus;
    use crate::corpus::{Run, Task};
    use std::path::Path;
    use std::sync::atomic::AtomicBool;

    impl Task {
        pub fn perform(
            &self,
            run: &mut Run,
            repo: &Path,
            progress: &mut corpus::Progress,
            threads: Option<usize>,
            should_interrupt: &AtomicBool,
        ) {
            let start = std::time::Instant::now();
            if let Err(err) = self.execute.execute(repo, progress, threads, should_interrupt) {
                run.error = Some(format!("{err:#?}"))
            }
            run.duration = start.elapsed();
        }
    }

    /// Note that once runs have been recorded, the implementation must not change anymore to keep it comparable.
    /// If changes have be done, rather change the name of the owning task to start a new kind of task.
    pub(crate) trait Execute {
        fn execute(
            &self,
            repo: &Path,
            progress: &mut corpus::Progress,
            threads: Option<usize>,
            should_interrupt: &AtomicBool,
        ) -> anyhow::Result<()>;
    }

    pub(crate) static ALL: &[Task] = &[Task {
        short_name: "OPNR",
        description: "open repository (isolated)",
        execute_exclusive: false,
        execute: &OpenRepo,
    }];

    struct OpenRepo;

    impl Execute for OpenRepo {
        fn execute(
            &self,
            repo: &Path,
            _progress: &mut corpus::Progress,
            _threads: Option<usize>,
            _should_interrupt: &AtomicBool,
        ) -> anyhow::Result<()> {
            gix::open_opts(repo, gix::open::Options::isolated())?;
            Ok(())
        }
    }
}
