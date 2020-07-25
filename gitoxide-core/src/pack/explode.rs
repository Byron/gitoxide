use anyhow::Result;
use git_features::progress::Progress;
use std::path::Path;

pub fn pack_or_pack_index<P>(_path: impl AsRef<Path>, _progress: Option<P>, _delete_pack: bool) -> Result<()>
where
    P: Progress,
{
    Ok(())
}
