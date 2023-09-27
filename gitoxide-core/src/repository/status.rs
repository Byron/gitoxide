use crate::OutputFormat;
use anyhow::{bail, Context};
use gix::bstr::{BStr, BString};
use gix::index::Entry;
use gix::prelude::FindExt;
use gix::Progress;
use gix_status::index_as_worktree::traits::FastEq;
use gix_status::index_as_worktree::{Change, Conflict, EntryStatus};

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
        statistics,
    }: Options,
) -> anyhow::Result<()> {
    if format != OutputFormat::Human {
        bail!("Only human format is supported right now");
    }
    let mut index = repo.index_or_empty()?;
    let index = gix::threading::make_mut(&mut index);
    let pathspec = repo.pathspec(
        pathspecs,
        true,
        index,
        gix::worktree::stack::state::attributes::Source::WorktreeThenIdMapping,
    )?;
    let mut progress = progress.add_child("traverse index");
    let start = std::time::Instant::now();
    let options = gix_status::index_as_worktree::Options {
        fs: repo.filesystem_options()?,
        thread_limit,
        stat: repo.stat_options()?,
        attributes: match repo
            .attributes_only(
                index,
                gix::worktree::stack::state::attributes::Source::WorktreeThenIdMapping,
            )?
            .detach()
            .state_mut()
        {
            gix::worktree::stack::State::AttributesStack(attrs) => std::mem::take(attrs),
            // TODO: this should be nicer by creating attributes directly, but it's a private API
            _ => unreachable!("state must be attributes stack only"),
        },
    };
    let outcome = gix_status::index_as_worktree(
        index,
        repo.work_dir()
            .context("This operation cannot be run on a bare repository")?,
        &mut Printer(out),
        FastEq,
        Submodule,
        {
            let odb = repo.objects.clone().into_arc()?;
            move |id, buf| odb.find_blob(id, buf)
        },
        &mut progress,
        pathspec.detach()?,
        repo.filter_pipeline(Some(gix::hash::ObjectId::empty_tree(repo.object_hash())))?
            .0
            .into_parts()
            .0,
        &gix::interrupt::IS_INTERRUPTED,
        options,
    )?;

    if statistics {
        writeln!(err, "{outcome:#?}").ok();
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

struct Printer<W>(W);

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
        _entry_index: usize,
        rela_path: &'index BStr,
        status: EntryStatus<Self::ContentChange>,
    ) {
        self.visit_inner(rela_path, status).ok();
    }
}

impl<W: std::io::Write> Printer<W> {
    fn visit_inner(&mut self, rela_path: &BStr, status: EntryStatus<()>) -> std::io::Result<()> {
        let char_storage;
        let status = match status {
            EntryStatus::Conflict(conflict) => as_str(conflict),
            EntryStatus::Change(change) => {
                char_storage = change_to_char(&change);
                std::str::from_utf8(std::slice::from_ref(&char_storage)).expect("valid ASCII")
            }
            EntryStatus::NeedsUpdate(_) => return Ok(()),
            EntryStatus::IntentToAdd => "A",
        };

        writeln!(&mut self.0, "{status: >3} {rela_path}")
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
