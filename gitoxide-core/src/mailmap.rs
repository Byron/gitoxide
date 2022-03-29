use crate::OutputFormat;
use anyhow::{bail, Context};
use git_repository as git;
use std::collections::HashSet;
use std::io::Write;
use std::path::Path;

pub const PROGRESS_RANGE: std::ops::RangeInclusive<u8> = 1..=2;

pub fn verify(path: impl AsRef<Path>, format: OutputFormat, mut out: impl Write) -> anyhow::Result<()> {
    if format != OutputFormat::Human {
        bail!("Only 'human' format is currently supported");
    }
    let path = path.as_ref();
    let buf = std::fs::read(path).with_context(|| format!("Failed to read mailmap file at '{}'", path.display()))?;
    let mut err_count = 0;
    for err in git::mailmap::parse(&buf).filter_map(Result::err) {
        err_count += 1;
        writeln!(out, "{}", err)?;
    }

    let mut seen = HashSet::<(_, _)>::default();
    for entry in git::mailmap::parse(&buf).filter_map(Result::ok) {
        if !seen.insert((entry.old_email(), entry.old_name())) {
            writeln!(
                out,
                "NOTE: entry ({:?}, {:?}) -> ({:?}, {:?}) is being overwritten",
                entry.old_email(),
                entry.old_name(),
                entry.new_email(),
                entry.new_name()
            )?;
        }
    }

    if err_count == 0 {
        writeln!(out, "{} lines OK", git::mailmap::parse(&buf).count())?;
        Ok(())
    } else {
        bail!("{} lines in '{}' could not be parsed", err_count, path.display());
    }
}
