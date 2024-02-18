use anyhow::{bail, Context};
use gix::bstr::ByteSlice;
use gix::{
    bstr::{BStr, BString},
    index::Entry,
    Progress,
};
use gix_status::index_as_worktree::{traits::FastEq, Change, Conflict, EntryStatus};
use std::path::{Path, PathBuf};

use crate::OutputFormat;

pub enum Submodules {
    /// display all information about submodules, including ref changes, modifications and untracked files.
    All,
    /// Compare only the configuration of the superprojects commit with the actually checked out `HEAD` commit.
    RefChange,
    /// See if there are worktree modifications compared to the index, but do not check for untracked files.
    Modifications,
}

pub struct Options {
    pub format: OutputFormat,
    pub submodules: Submodules,
    pub thread_limit: Option<usize>,
    pub statistics: bool,
    pub allow_write: bool,
}

pub fn show(
    repo: gix::Repository,
    pathspecs: Vec<BString>,
    out: impl std::io::Write,
    mut err: impl std::io::Write,
    mut progress: impl gix::NestedProgress,
    Options {
        format,
        // TODO: implement this
        submodules: _,
        thread_limit,
        allow_write,
        statistics,
    }: Options,
) -> anyhow::Result<()> {
    if format != OutputFormat::Human {
        bail!("Only human format is supported right now");
    }
    let mut index = repo.index_or_empty()?;
    let index = gix::threading::make_mut(&mut index);
    let mut progress = progress.add_child("traverse index");
    let start = std::time::Instant::now();
    let stack = repo
        .attributes_only(
            index,
            gix::worktree::stack::state::attributes::Source::WorktreeThenIdMapping,
        )?
        .detach();
    let pathspec = gix::Pathspec::new(&repo, false, pathspecs.iter().map(|p| p.as_bstr()), true, || {
        Ok(stack.clone())
    })?;
    let options = gix_status::index_as_worktree::Options {
        fs: repo.filesystem_options()?,
        thread_limit,
        stat: repo.stat_options()?,
    };
    let prefix = repo.prefix()?.unwrap_or(Path::new(""));
    let mut printer = Printer {
        out,
        changes: Vec::new(),
        prefix: prefix.to_owned(),
    };
    let filter_pipeline = repo
        .filter_pipeline(Some(gix::hash::ObjectId::empty_tree(repo.object_hash())))?
        .0
        .into_parts()
        .0;
    let ctx = gix_status::index_as_worktree::Context {
        pathspec: pathspec.into_parts().0,
        stack,
        filter: filter_pipeline,
        should_interrupt: &gix::interrupt::IS_INTERRUPTED,
    };
    let mut collect = gix::dir::walk::delegate::Collect::default();
    let (outcome, walk_outcome) = gix::features::parallel::threads(|scope| -> anyhow::Result<_> {
        // TODO: it's either this, or not running both in parallel and setting UPTODATE flags whereever
        //       there is no modification. This can save disk queries as dirwalk can then trust what's in
        //       the index regarding the type.
        // NOTE: collect here as rename-tracking needs that anyway.
        let walk_outcome = gix::features::parallel::build_thread()
            .name("gix status::dirwalk".into())
            .spawn_scoped(scope, {
                let repo = repo.clone().into_sync();
                let index = &index;
                let collect = &mut collect;
                move || -> anyhow::Result<_> {
                    let repo = repo.to_thread_local();
                    let outcome = repo.dirwalk(
                        index,
                        pathspecs,
                        repo.dirwalk_options()?
                            .emit_untracked(gix::dir::walk::EmissionMode::CollapseDirectory),
                        collect,
                    )?;
                    Ok(outcome.dirwalk)
                }
            })?;

        let outcome = gix_status::index_as_worktree(
            index,
            repo.work_dir()
                .context("This operation cannot be run on a bare repository")?,
            &mut printer,
            FastEq,
            Submodule,
            repo.objects.clone().into_arc()?,
            &mut progress,
            ctx,
            options,
        )?;

        let walk_outcome = walk_outcome.join().expect("no panic")?;
        Ok((outcome, walk_outcome))
    })?;

    for entry in collect
        .into_entries_by_path()
        .into_iter()
        .filter_map(|(entry, dir_status)| dir_status.is_none().then_some(entry))
    {
        writeln!(
            printer.out,
            "{status: >3} {rela_path}",
            status = "?",
            rela_path = gix::path::relativize_with_prefix(&gix::path::from_bstr(entry.rela_path), prefix).display()
        )?;
    }

    if outcome.entries_to_update != 0 && allow_write {
        {
            let entries = index.entries_mut();
            for (entry_index, change) in printer.changes {
                let entry = &mut entries[entry_index];
                match change {
                    ApplyChange::SetSizeToZero => {
                        entry.stat.size = 0;
                    }
                    ApplyChange::NewStat(new_stat) => {
                        entry.stat = new_stat;
                    }
                }
            }
        }
        index.write(gix::index::write::Options {
            extensions: Default::default(),
            skip_hash: false, // TODO: make this based on configuration
        })?;
    }

    if statistics {
        writeln!(err, "{outcome:#?}").ok();
        writeln!(err, "{walk_outcome:#?}").ok();
    }

    writeln!(err, "\nhead -> index and untracked files aren't implemented yet")?;
    progress.show_throughput(start);
    Ok(())
}

#[derive(Clone)]
struct Submodule;

impl gix_status::index_as_worktree::traits::SubmoduleStatus for Submodule {
    type Output = ();
    type Error = std::convert::Infallible;

    fn status(&mut self, _entry: &Entry, _rela_path: &BStr) -> Result<Option<Self::Output>, Self::Error> {
        Ok(None)
    }
}

struct Printer<W> {
    out: W,
    changes: Vec<(usize, ApplyChange)>,
    prefix: PathBuf,
}

enum ApplyChange {
    SetSizeToZero,
    NewStat(gix::index::entry::Stat),
}

impl<'index, W> gix_status::index_as_worktree::VisitEntry<'index> for Printer<W>
where
    W: std::io::Write,
{
    type ContentChange = ();
    type SubmoduleStatus = ();

    fn visit_entry(
        &mut self,
        _entries: &'index [Entry],
        _entry: &'index Entry,
        entry_index: usize,
        rela_path: &'index BStr,
        status: EntryStatus<Self::ContentChange>,
    ) {
        self.visit_inner(entry_index, rela_path, status).ok();
    }
}

impl<W: std::io::Write> Printer<W> {
    fn visit_inner(&mut self, entry_index: usize, rela_path: &BStr, status: EntryStatus<()>) -> std::io::Result<()> {
        let char_storage;
        let status = match status {
            EntryStatus::Conflict(conflict) => as_str(conflict),
            EntryStatus::Change(change) => {
                if matches!(
                    change,
                    Change::Modification {
                        set_entry_stat_size_zero: true,
                        ..
                    }
                ) {
                    self.changes.push((entry_index, ApplyChange::SetSizeToZero))
                }
                char_storage = change_to_char(&change);
                std::str::from_utf8(std::slice::from_ref(&char_storage)).expect("valid ASCII")
            }
            EntryStatus::NeedsUpdate(stat) => {
                self.changes.push((entry_index, ApplyChange::NewStat(stat)));
                return Ok(());
            }
            EntryStatus::IntentToAdd => "A",
        };

        let rela_path = gix::path::from_bstr(rela_path);
        let display_path = gix::path::relativize_with_prefix(&rela_path, &self.prefix);
        writeln!(&mut self.out, "{status: >3} {}", display_path.display())
    }
}

fn as_str(c: Conflict) -> &'static str {
    match c {
        Conflict::BothDeleted => "DD",
        Conflict::AddedByUs => "AU",
        Conflict::DeletedByThem => "UD",
        Conflict::AddedByThem => "UA",
        Conflict::DeletedByUs => "DU",
        Conflict::BothAdded => "AA",
        Conflict::BothModified => "UU",
    }
}

fn change_to_char(change: &Change<()>) -> u8 {
    // Known status letters: https://github.com/git/git/blob/6807fcfedab84bc8cd0fbf721bc13c4e68cda9ae/diff.h#L613
    match change {
        Change::Removed => b'D',
        Change::Type => b'T',
        Change::SubmoduleModification(_) => b'M',
        Change::Modification {
            executable_bit_changed, ..
        } => {
            if *executable_bit_changed {
                b'X'
            } else {
                b'M'
            }
        }
    }
}
