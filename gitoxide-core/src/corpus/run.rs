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
