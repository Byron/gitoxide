use anyhow::{anyhow, Context, Result};
use std::{io, path::Path};

pub fn init() -> Result<()> {
    git_repository::init::repository().with_context(|| "Repository initialization failed")
}

pub fn verify_pack_or_pack_index(path: impl AsRef<Path>, mut out: impl io::Write) -> Result<()> {
    let path = path.as_ref();
    let ext = path.extension()
        .and_then(|ext| ext.to_str())
        .ok_or_else(|| anyhow!("Cannot determine file type on path without extension '{}', expecting default extensions 'idx' and 'pack'", path.display()))?;
    match ext {
        "pack" => {
            let pack = git_odb::pack::File::at(path).with_context(|| "Could not open pack file")?;
            pack.verify_checksum()?;
        }
        "idx" => {
            let idx = git_odb::pack::index::File::at(path)
                .with_context(|| "Could not open pack index file")?;
            idx.verify_checksum_of_index()?;
        }
        ext => {
            return Err(anyhow!(
                "Unknown extension {:?}, expecting 'idx' or 'pack'",
                ext
            ))
        }
    }
    writeln!(out, "OK")?;
    Ok(())
}
