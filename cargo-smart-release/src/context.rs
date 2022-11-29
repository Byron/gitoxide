use cargo_metadata::{
    camino::{Utf8Path, Utf8PathBuf},
    Metadata, Package,
};
use git_repository as git;

use crate::version::BumpSpec;

pub struct Context {
    pub root: Utf8PathBuf,
    pub meta: Metadata,
    pub repo: git::Repository,
    pub crate_names: Vec<String>,
    pub crates_index: crate::crates_index::Index,
    pub history: Option<crate::commit::History>,
    pub bump: BumpSpec,
    pub bump_dependencies: BumpSpec,
}

impl Context {
    pub fn new(
        crate_names: Vec<String>,
        force_history_segmentation: bool,
        bump: BumpSpec,
        bump_dependencies: BumpSpec,
    ) -> anyhow::Result<Self> {
        let meta = cargo_metadata::MetadataCommand::new().exec()?;
        let root = meta.workspace_root.clone();
        let repo = git::discover(&root)?;
        let crates_index = crate::crates_index::Index::new_cargo_default()?;
        let history = (force_history_segmentation
            || matches!(bump, BumpSpec::Auto)
            || matches!(bump_dependencies, BumpSpec::Auto))
        .then(|| crate::git::history::collect(&repo))
        .transpose()?
        .flatten();
        Ok(Context {
            root,
            repo,
            meta,
            crate_names: fill_in_root_crate_if_needed(crate_names)?,
            crates_index,
            history,
            bump,
            bump_dependencies,
        })
    }

    pub(crate) fn repo_relative_path<'a>(&self, p: &'a Package) -> Option<&'a Utf8Path> {
        let dir = p
            .manifest_path
            .parent()
            .expect("parent of a file is always present")
            .strip_prefix(&self.root)
            .expect("workspace members are relative to the root directory");

        if dir.as_os_str().is_empty() {
            None
        } else {
            dir.into()
        }
    }
}

fn fill_in_root_crate_if_needed(crate_names: Vec<String>) -> anyhow::Result<Vec<String>> {
    Ok(if crate_names.is_empty() {
        let current_dir = std::env::current_dir()?;
        let manifest = current_dir.join("Cargo.toml");
        let dir_name = current_dir
            .file_name()
            .expect("a valid directory with a name")
            .to_str()
            .expect("directory is UTF8 representable");
        let crate_name = if manifest.is_file() {
            cargo_toml::Manifest::from_path(manifest)
                .map(|manifest| manifest.package.map_or(dir_name.to_owned(), |p| p.name))
                .unwrap_or_else(|_| dir_name.to_owned())
        } else {
            dir_name.to_owned()
        };
        log::warn!(
            "Using '{}' as crate name as no one was provided. Specify one if this isn't correct",
            crate_name
        );
        vec![crate_name]
    } else {
        crate_names
    })
}
