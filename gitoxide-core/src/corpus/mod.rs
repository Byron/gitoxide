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

pub(crate) mod trace {
    use rusqlite::params;
    use std::path::Path;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::{Arc, Mutex};
    use tracing_forest::tree::Tree;
    use tracing_subscriber::layer::SubscriberExt;

    pub fn override_thread_subscriber(
        db_path: impl AsRef<Path>,
    ) -> anyhow::Result<(tracing::subscriber::DefaultGuard, Arc<AtomicU32>)> {
        let current_id = Arc::new(AtomicU32::default());
        let processor = tracing_forest::Printer::new().formatter(StoreTreeToDb {
            con: Arc::new(Mutex::new(rusqlite::Connection::open(&db_path)?)),
            run_id: current_id.clone(),
        });
        let subscriber = tracing_subscriber::Registry::default().with(tracing_forest::ForestLayer::from(processor));
        let guard = tracing::subscriber::set_default(subscriber);
        Ok((guard, current_id))
    }

    pub struct StoreTreeToDb {
        pub con: Arc<Mutex<rusqlite::Connection>>,
        pub run_id: Arc<AtomicU32>,
    }
    impl tracing_forest::printer::Formatter for StoreTreeToDb {
        type Error = rusqlite::Error;

        fn fmt(&self, tree: &Tree) -> Result<String, Self::Error> {
            let json = serde_json::to_string_pretty(&tree).expect("serialization to string always works");
            let run_id = self.run_id.load(Ordering::SeqCst);
            self.con
                .lock()
                .unwrap()
                .execute("UPDATE run SET spans_json = ?1 WHERE id = ?2", params![json, run_id])?;
            Ok(String::new())
        }
    }
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
