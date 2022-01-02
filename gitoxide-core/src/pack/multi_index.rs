use std::{io::BufWriter, path::PathBuf, sync::atomic::AtomicBool};

use git_repository as git;
use git_repository::Progress;

pub const PROGRESS_RANGE: std::ops::RangeInclusive<u8> = 1..=3;

pub fn verify(multi_index_path: PathBuf, progress: impl Progress, should_interrupt: &AtomicBool) -> anyhow::Result<()> {
    git::odb::pack::multi_index::File::at(multi_index_path)?.verify_integrity_fast(progress, should_interrupt)?;
    Ok(())
}

pub fn create(
    index_paths: Vec<PathBuf>,
    output_path: PathBuf,
    progress: impl Progress,
    should_interrupt: &AtomicBool,
    object_hash: git::hash::Kind,
) -> anyhow::Result<()> {
    let mut out = BufWriter::new(git::lock::File::acquire_to_update_resource(
        output_path,
        git::lock::acquire::Fail::Immediately,
        None,
    )?);
    git::odb::pack::multi_index::File::write_from_index_paths(
        index_paths,
        &mut out,
        progress,
        should_interrupt,
        git::odb::pack::multi_index::write::Options { object_hash },
    )?;
    out.into_inner()?.commit()?;
    Ok(())
}
