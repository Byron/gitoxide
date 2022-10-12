use git::prelude::FindExt;
use git_repository as git;
use std::ffi::OsString;
use std::{io::BufWriter, path::PathBuf};

pub fn from_tree(
    mut spec: OsString,
    index_path: Option<PathBuf>,
    force: bool,
    repo: git::Repository,
    mut out: impl std::io::Write,
) -> anyhow::Result<()> {
    spec.push("^{tree}");
    let spec = git::path::os_str_into_bstr(&spec)?;
    let tree = repo.rev_parse_single(spec)?;
    let state = git::index::State::from_tree(&tree, |oid, buf| repo.objects.find_tree_iter(oid, buf).ok())?;
    let options = git::index::write::Options {
        hash_kind: repo.object_hash(),
        extensions: Default::default(),
    };

    match index_path {
        Some(index_path) => {
            if index_path.is_file() && !force {
                anyhow::bail!(
                    "File at \"{}\" already exists, to overwrite use the '-f' flag",
                    index_path.display()
                );
            }
            let writer = BufWriter::new(std::fs::File::create(&index_path)?);
            state.write_to(writer, options)?;
        }
        None => {
            state.write_to(&mut out, options)?;
        }
    }

    Ok(())
}
