use anyhow::Context;

use crate::OutputFormat;

pub fn previous_branches(
    repo: gix::Repository,
    mut out: impl std::io::Write,
    format: OutputFormat,
) -> anyhow::Result<()> {
    let branches = repo
        .head()?
        .prior_checked_out_branches()?
        .context("The reflog for HEAD is required")?;
    match format {
        OutputFormat::Human => {
            for (name, id) in branches {
                writeln!(out, "{id} {name}")?;
            }
        }
        #[cfg(feature = "serde")]
        OutputFormat::Json => {
            serde_json::to_writer_pretty(&mut out, &branches)?;
        }
    }
    Ok(())
}
