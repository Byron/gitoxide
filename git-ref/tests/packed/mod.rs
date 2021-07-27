use std::path::PathBuf;

pub fn write_packed_refs_with(input: &[u8]) -> crate::Result<(tempfile::TempDir, PathBuf)> {
    let dir = tempfile::tempdir()?;
    let packed_refs_path = dir.path().join("packed-refs");
    std::fs::write(&packed_refs_path, input)?;
    Ok((dir, packed_refs_path))
}

mod find;
pub mod iter;
mod open;
mod transaction;
