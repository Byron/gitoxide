use crate::OutputFormat;
use git_repository as git;
use std::io;
use std::path::PathBuf;

pub fn entries(
    repository: PathBuf,
    format: OutputFormat,
    mut out: impl io::Write,
    mut err: impl io::Write,
) -> anyhow::Result<()> {
    if format == OutputFormat::Human {
        writeln!(err, "Defaulting to JSON as human format isn't implemented").ok();
    }

    let repo = git::open(repository)?.apply_environment();
    let mut mailmap = git::mailmap::Snapshot::default();
    if let Err(e) = repo.load_mailmap_into(&mut mailmap) {
        writeln!(err, "Error while loading mailmap: {}", e).ok();
    }

    Ok(())
}
