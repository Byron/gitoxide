use git_repository as git;
use std::path::Path;

#[allow(unused)]
pub fn entries(
    index_path: impl AsRef<Path>,
    out: impl std::io::Write,
    object_hash: git::hash::Kind,
) -> anyhow::Result<()> {
    let file = git::index::File::at(
        index_path.as_ref(),
        git::index::decode::Options {
            object_hash,
            ..Default::default()
        },
    )?;
    todo!("print entries")
}
