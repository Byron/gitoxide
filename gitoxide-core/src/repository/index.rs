use git::prelude::FindExt;
use git_repository as git;
use std::ffi::OsString;
use std::path::PathBuf;

pub fn from_tree(
    mut spec: OsString,
    index_path: Option<PathBuf>,
    force: bool,
    repo: git::Repository,
) -> anyhow::Result<()> {
    spec.push("^{tree}");
    let spec = git::path::os_str_into_bstr(&spec)?;
    let tree = repo.rev_parse_single(spec)?;
    let index = git::index::State::from_tree(&tree, |oid, buf| repo.objects.find_tree_iter(oid, buf).ok())?;
    let options = git::index::write::Options::default();

    match index_path {
        Some(index_path) => {
            if index_path.is_file() && !force {
                anyhow::bail!(
                    "File at \"{}\" already exists, to overwrite use the '-f' flag",
                    index_path.display()
                );
            }
            let mut index = git::index::File::from_state(index, index_path);
            index.write(options)?;
        }
        None => {
            let index = git::index::File::from_state(index, std::path::PathBuf::new());
            let mut out = Vec::with_capacity(512 * 1024);
            index.write_to(&mut out, options)?;
        }
    }

    Ok(())
}
