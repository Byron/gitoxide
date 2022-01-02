use std::path::PathBuf;

use anyhow::{Context as AnyhowContext, Result};

pub fn init(directory: Option<PathBuf>) -> Result<git_repository::Path> {
    git_repository::path::create::into(directory.unwrap_or_default(), git_repository::Kind::WorkTree)
        .with_context(|| "Repository initialization failed")
}

pub mod verify {
    use std::{path::PathBuf, sync::atomic::AtomicBool};

    use git_repository::Progress;

    use crate::OutputFormat;

    pub const PROGRESS_RANGE: std::ops::RangeInclusive<u8> = 1..=4;

    pub fn integrity(
        repo: PathBuf,
        _format: OutputFormat,
        _out: impl std::io::Write,
        progress: impl Progress,
        should_interrupt: &AtomicBool,
    ) -> anyhow::Result<()> {
        let repo = git_repository::open(repo)?;
        // TODO: a way to get the pack cache from a handle
        repo.objects.verify_integrity(
            progress,
            should_interrupt,
            git_repository::odb::pack::index::verify::integrity::Options::default(),
        )?;
        Ok(())
    }
}
