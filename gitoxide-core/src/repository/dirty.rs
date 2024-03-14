use crate::OutputFormat;
use anyhow::bail;

pub enum Mode {
    IsClean,
    IsDirty,
}

pub fn check(
    repo: gix::Repository,
    mode: Mode,
    out: &mut dyn std::io::Write,
    format: OutputFormat,
) -> anyhow::Result<()> {
    if format != OutputFormat::Human {
        bail!("JSON output isn't implemented yet");
    }
    let is_dirty = repo.is_dirty()?;
    let res = match (is_dirty, mode) {
        (false, Mode::IsClean) => Ok("The repository is clean"),
        (true, Mode::IsClean) => Err("The repository has changes"),
        (false, Mode::IsDirty) => Err("The repository is clean"),
        (true, Mode::IsDirty) => Ok("The repository has changes"),
    };

    let suffix = "(not counting untracked files)";
    match res {
        Ok(msg) => writeln!(out, "{msg} {suffix}")?,
        Err(msg) => bail!("{msg} {suffix}"),
    }
    Ok(())
}
