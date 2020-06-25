use anyhow::{Context, Result};
use std::{io, path::Path};

pub fn init() -> Result<()> {
    git_repository::init::repository().with_context(|| "Repository initialization failed")
}

pub fn verify_pack_or_pack_index(path: impl AsRef<Path>, mut out: impl io::Write) -> Result<()> {
    let pack = git_odb::pack::File::at(path).with_context(|| "Could not open pack file")?;
    pack.verify_checksum().with_context(|| "Failed")?;
    writeln!(out, "OK")?;
    Ok(())
}
