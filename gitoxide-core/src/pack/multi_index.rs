use anyhow::bail;
use std::{io::BufWriter, path::PathBuf, sync::atomic::AtomicBool};

use crate::OutputFormat;
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

mod info {
    use std::path::PathBuf;

    #[cfg_attr(feature = "serde1", derive(serde::Serialize))]
    pub struct Statistics {
        pub path: PathBuf,
        pub num_objects: u32,
        pub index_names: Vec<PathBuf>,
        pub object_hash: String,
    }
}

pub fn info(
    multi_index_path: PathBuf,
    format: OutputFormat,
    out: impl std::io::Write,
    mut err: impl std::io::Write,
) -> anyhow::Result<()> {
    let file = git::odb::pack::multi_index::File::at(&multi_index_path)?;
    if format == OutputFormat::Human {
        writeln!(err, "Defaulting to JSON as human format isn't implemented").ok();
    }
    #[cfg(feature = "serde1")]
    serde_json::to_writer_pretty(
        out,
        &info::Statistics {
            path: multi_index_path,
            num_objects: file.num_objects(),
            index_names: file.index_names().to_vec(),
            object_hash: file.object_hash().to_string(),
        },
    )?;
    Ok(())
}

pub fn entries(multi_index_path: PathBuf, format: OutputFormat, mut out: impl std::io::Write) -> anyhow::Result<()> {
    if format != OutputFormat::Human {
        bail!("Only human format is supported right now");
    }
    let file = git::odb::pack::multi_index::File::at(&multi_index_path)?;
    for entry in file.iter() {
        writeln!(out, "{} {} {}", entry.oid, entry.pack_index, entry.pack_offset)?;
    }
    Ok(())
}
