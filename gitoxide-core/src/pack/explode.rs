use anyhow::{Context, Result};
use git_features::progress::Progress;
use git_odb::pack;
use std::path::Path;

pub fn pack_or_pack_index<P>(path: impl AsRef<Path>, _progress: Option<P>, _delete_pack: bool) -> Result<()>
where
    P: Progress,
{
    let path = path.as_ref();
    let _bundle = pack::Bundle::at(path).with_context(|| {
        format!(
            "Could not find .idx or .pack file from given file at '{}'",
            path.display()
        )
    })?;
    Ok(())
}
