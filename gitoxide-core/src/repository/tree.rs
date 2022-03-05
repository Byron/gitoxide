use anyhow::bail;
use std::io;
use std::path::PathBuf;

use crate::OutputFormat;
use git_repository as git;
use git_repository::prelude::ObjectIdExt;

mod entries {
    use git_repository as git;

    use git::hash::oid;
    use git::objs::{bstr::BStr, tree::EntryRef};
    use git::traverse::tree::visit::Action;

    pub struct Traverse {
        pub num_trees: usize,
        pub num_links: usize,
        pub num_blobs: usize,
        pub num_blobs_exec: usize,
        pub num_submodules: usize,
        pub num_bytes: u64,
        pub repo: git::Repository,
    }

    impl Traverse {
        pub fn new(repo: git::Repository) -> Self {
            Traverse {
                num_trees: 0,
                num_links: 0,
                num_blobs: 0,
                num_blobs_exec: 0,
                num_submodules: 0,
                num_bytes: 0,
                repo,
            }
        }

        pub(crate) fn count_bytes(&mut self, oid: &oid) {
            if let Ok(obj) = self.repo.find_object(oid) {
                self.num_bytes += obj.data.len() as u64;
            }
        }
    }

    impl git::traverse::tree::Visit for Traverse {
        fn pop_front_tracked_path_and_set_current(&mut self) {}

        fn push_back_tracked_path_component(&mut self, _component: &BStr) {}

        fn push_path_component(&mut self, _component: &BStr) {}

        fn pop_path_component(&mut self) {}

        fn visit_tree(&mut self, _entry: &EntryRef<'_>) -> Action {
            self.num_trees += 1;
            Action::Continue
        }

        fn visit_nontree(&mut self, entry: &EntryRef<'_>) -> Action {
            use git::objs::tree::EntryMode::*;
            match entry.mode {
                Commit => self.num_submodules += 1,
                Blob => {
                    self.count_bytes(entry.oid);
                    self.num_blobs += 1
                }
                BlobExecutable => {
                    self.count_bytes(entry.oid);
                    self.num_blobs_exec += 1
                }
                Link => self.num_links += 1,
                Tree => unreachable!("BUG"),
            }
            Action::Continue
        }
    }
}

pub fn entries(
    repository: PathBuf,
    treeish: Option<&str>,
    recursive: bool,
    extended: bool,
    format: OutputFormat,
    out: &mut dyn io::Write,
    _err: &mut dyn io::Write,
) -> anyhow::Result<()> {
    if format == OutputFormat::Json {
        bail!("Only human output format is supported at the moment");
    }

    let tree_repo = git::open(repository)?;
    let mut repo = tree_repo.clone().apply_environment();
    repo.object_cache_size(128 * 1024);

    let tree = match treeish {
        Some(hex) => git::hash::ObjectId::from_hex(hex.as_bytes())
            .map(|id| id.attach(&repo))?
            .object()?
            .try_into_tree()?,
        None => repo.head()?.peel_to_commit_in_place()?.tree()?,
    };

    if recursive {
    } else {
        for entry in tree.iter() {
            let entry = entry?;
            format_entry(
                &mut *out,
                &entry.inner,
                extended
                    .then(|| entry.id().object().map(|o| o.data.len()))
                    .transpose()?,
            )?;
        }
    }

    let mut delegate = entries::Traverse::new(tree_repo);
    tree.traverse().breadthfirst(&mut delegate)?;
    Ok(())
}

fn format_entry(
    mut _out: impl io::Write,
    _entry: &git::objs::tree::EntryRef<'_>,
    _size: Option<usize>,
) -> std::io::Result<()> {
    todo!()
}
