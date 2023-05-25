use std::{borrow::Cow, io};

use anyhow::bail;
use gix::Tree;

use crate::OutputFormat;

mod entries {
    use std::collections::VecDeque;

    use gix::{
        bstr::{BStr, BString, ByteSlice, ByteVec},
        objs::tree::EntryRef,
        traverse::tree::visit::Action,
    };

    use crate::repository::tree::format_entry;

    #[cfg_attr(feature = "serde", derive(serde::Serialize))]
    #[derive(Default)]
    pub struct Statistics {
        pub num_trees: usize,
        pub num_links: usize,
        pub num_blobs: usize,
        pub num_blobs_exec: usize,
        pub num_submodules: usize,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub bytes: Option<u64>,
        #[cfg_attr(feature = "serde", serde(skip))]
        pub num_bytes: u64,
    }

    pub struct Traverse<'repo, 'a> {
        pub stats: Statistics,
        repo: Option<&'repo gix::Repository>,
        out: Option<&'a mut dyn std::io::Write>,
        path: BString,
        path_deque: VecDeque<BString>,
    }

    impl<'repo, 'a> Traverse<'repo, 'a> {
        pub fn new(repo: Option<&'repo gix::Repository>, out: Option<&'a mut dyn std::io::Write>) -> Self {
            Traverse {
                stats: Default::default(),
                repo,
                out,
                path: BString::default(),
                path_deque: VecDeque::new(),
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

    impl<'repo, 'a> gix::traverse::tree::Visit for Traverse<'repo, 'a> {
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
            self.stats.num_trees += 1;
            Action::Continue
        }

        fn visit_nontree(&mut self, entry: &EntryRef<'_>) -> Action {
            use gix::objs::tree::EntryMode::*;
            let size = self
                .repo
                .and_then(|repo| repo.find_object(entry.oid).map(|o| o.data.len()).ok());
            if let Some(out) = &mut self.out {
                format_entry(out, entry, self.path.as_bstr(), size).ok();
            }
            if let Some(size) = size {
                self.stats.num_bytes += size as u64;
            }

            match entry.mode {
                Commit => self.stats.num_submodules += 1,
                Blob => self.stats.num_blobs += 1,
                BlobExecutable => self.stats.num_blobs_exec += 1,
                Link => self.stats.num_links += 1,
                Tree => unreachable!("BUG"),
            }
            Action::Continue
        }
    }
}

#[cfg_attr(not(feature = "serde"), allow(unused_variables))]
pub fn info(
    repo: gix::Repository,
    treeish: Option<&str>,
    extended: bool,
    format: OutputFormat,
    out: impl io::Write,
    mut err: impl io::Write,
) -> anyhow::Result<()> {
    if format == OutputFormat::Human {
        writeln!(err, "Only JSON is implemented - using that instead")?;
    }

    let tree = treeish_to_tree(treeish, &repo)?;

    let mut delegate = entries::Traverse::new(extended.then_some(&repo), None);
    tree.traverse().breadthfirst(&mut delegate)?;

    #[cfg(feature = "serde")]
    {
        delegate.stats.bytes = extended.then_some(delegate.stats.num_bytes);
        serde_json::to_writer_pretty(out, &delegate.stats)?;
    }

    Ok(())
}

pub fn entries(
    repo: gix::Repository,
    treeish: Option<&str>,
    recursive: bool,
    extended: bool,
    format: OutputFormat,
    mut out: impl io::Write,
) -> anyhow::Result<()> {
    if format != OutputFormat::Human {
        bail!("Only human output format is supported at the moment");
    }

    let tree = treeish_to_tree(treeish, &repo)?;

    if recursive {
        let mut delegate = entries::Traverse::new(extended.then_some(&repo), Some(&mut out));
        tree.traverse().breadthfirst(&mut delegate)?;
    } else {
        for entry in tree.iter() {
            let entry = entry?;
            format_entry(
                &mut out,
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

fn treeish_to_tree<'repo>(treeish: Option<&str>, repo: &'repo gix::Repository) -> anyhow::Result<Tree<'repo>> {
    let spec = treeish.map_or_else(|| "@^{tree}".into(), |spec| format!("{spec}^{{tree}}"));
    Ok(repo.rev_parse_single(spec.as_str())?.object()?.into_tree())
}

fn format_entry(
    mut out: impl io::Write,
    entry: &gix::objs::tree::EntryRef<'_>,
    filename: &gix::bstr::BStr,
    size: Option<usize>,
) -> std::io::Result<()> {
    use gix::objs::tree::EntryMode::*;
    writeln!(
        out,
        "{} {}{} {}",
        match entry.mode {
            Tree => "TREE",
            Blob => "BLOB",
            BlobExecutable => " EXE",
            Link => "LINK",
            Commit => "SUBM",
        },
        entry.oid,
        size.map_or_else(|| "".into(), |s| Cow::Owned(format!(" {s}"))),
        filename
    )
}
