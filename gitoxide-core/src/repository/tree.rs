use anyhow::bail;
use std::io;
use std::path::PathBuf;

use crate::OutputFormat;
use git_repository as git;
use git_repository::prelude::ObjectIdExt;

mod entries {
    use git_repository as git;
    use std::collections::VecDeque;

    use crate::repository::tree::format_entry;
    use git::bstr::{BStr, BString};
    use git::hash::oid;
    use git::objs::tree::EntryRef;
    use git::traverse::tree::visit::Action;
    use git_repository::bstr::{ByteSlice, ByteVec};

    pub struct Traverse<'a> {
        pub num_trees: usize,
        pub num_links: usize,
        pub num_blobs: usize,
        pub num_blobs_exec: usize,
        pub num_submodules: usize,
        pub num_bytes: u64,
        pub repo: git::Repository,
        pub out: &'a mut dyn std::io::Write,
        path: BString,
        path_deque: VecDeque<BString>,
    }

    impl<'a> Traverse<'a> {
        pub fn new(repo: git::Repository, out: &'a mut dyn std::io::Write) -> Self {
            Traverse {
                num_trees: 0,
                num_links: 0,
                num_blobs: 0,
                num_blobs_exec: 0,
                num_submodules: 0,
                num_bytes: 0,
                repo,
                out,
                path: BString::default(),
                path_deque: VecDeque::new(),
            }
        }

        pub(crate) fn count_bytes(&mut self, oid: &oid) {
            if let Ok(obj) = self.repo.find_object(oid) {
                self.num_bytes += obj.data.len() as u64;
            }
        }

        fn pop_element(&mut self) {
            if let Some(pos) = self.path.rfind_byte(b'/') {
                self.path.resize(pos, 0);
            } else {
                self.path.clear();
            }
        }

        fn push_element(&mut self, name: &BStr) {
            if !self.path.is_empty() {
                self.path.push(b'/');
            }
            self.path.push_str(name);
        }
    }

    impl<'a> git::traverse::tree::Visit for Traverse<'a> {
        fn pop_front_tracked_path_and_set_current(&mut self) {
            self.path = self.path_deque.pop_front().expect("every parent is set only once");
        }

        fn push_back_tracked_path_component(&mut self, component: &BStr) {
            self.push_element(component);
            self.path_deque.push_back(self.path.clone());
        }

        fn push_path_component(&mut self, component: &BStr) {
            self.push_element(component);
        }

        fn pop_path_component(&mut self) {
            self.pop_element()
        }

        fn visit_tree(&mut self, _entry: &EntryRef<'_>) -> Action {
            self.num_trees += 1;
            Action::Continue
        }

        fn visit_nontree(&mut self, entry: &EntryRef<'_>) -> Action {
            use git::objs::tree::EntryMode::*;
            format_entry(&mut *self.out, entry, self.path.as_bstr(), None).ok();
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
        let mut delegate = entries::Traverse::new(tree_repo, out);
        tree.traverse().breadthfirst(&mut delegate)?;
    } else {
        for entry in tree.iter() {
            let entry = entry?;
            format_entry(
                &mut *out,
                &entry.inner,
                entry.inner.filename,
                extended
                    .then(|| entry.id().object().map(|o| o.data.len()))
                    .transpose()?,
            )?;
        }
    }

    Ok(())
}

fn format_entry(
    mut out: impl io::Write,
    entry: &git::objs::tree::EntryRef<'_>,
    filename: &git::bstr::BStr,
    _size: Option<usize>,
) -> std::io::Result<()> {
    use git::objs::tree::EntryMode::*;
    writeln!(
        out,
        "{} {} {}",
        match entry.mode {
            Tree => "TREE",
            Blob => "BLOB",
            BlobExecutable => " EXE",
            Link => "LINK",
            Commit => "SUBM",
        },
        entry.oid,
        filename
    )
}
