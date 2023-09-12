use crate::OutputFormat;
use anyhow::{bail, Context};
use gix::bstr::{BStr, BString};
use gix::index::Entry;
use gix::prelude::FindExt;
use gix::Progress;
use gix_status::index_as_worktree::content::FastEq;
use gix_status::index_as_worktree::Change;

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
    }: Options,
) -> anyhow::Result<()> {
    if format != OutputFormat::Human {
        bail!("Only human format is supported right now");
    }
    let mut index = repo.index()?;
    let index = gix::threading::make_mut(&mut index);
    let pathspec = repo.pathspec(
        pathspecs,
        true,
        index,
        gix::worktree::stack::state::attributes::Source::WorktreeThenIdMapping,
    )?;
    let mut progress = progress.add_child("traverse index");
    let start = std::time::Instant::now();
    gix_status::index_as_worktree(
        index,
        repo.work_dir()
            .context("This operation cannot be run on a bare repository")?,
        &mut Printer(out),
        FastEq,
        {
            let odb = repo.objects.clone().into_arc()?;
            move |id, buf| odb.find_blob(id, buf)
        },
        &mut progress,
        pathspec.detach()?,
        gix_status::index_as_worktree::Options {
            fs: repo.filesystem_options()?,
            thread_limit,
            stat: repo.stat_options()?,
        },
    )?;

    writeln!(err, "\nhead -> index and untracked files aren't implemented yet")?;
    progress.show_throughput(start);
    Ok(())
}

struct Printer<W>(W);

impl<'index, W> gix_status::index_as_worktree::VisitEntry<'index> for Printer<W>
where
    W: std::io::Write,
{
    type ContentChange = ();

    fn visit_entry(
        &mut self,
        entry: &'index Entry,
        rela_path: &'index BStr,
        change: Option<Change<Self::ContentChange>>,
        conflict: bool,
    ) {
        self.visit_inner(entry, rela_path, change, conflict).ok();
    }
}

impl<W: std::io::Write> Printer<W> {
    fn visit_inner(
        &mut self,
        _entry: &Entry,
        rela_path: &BStr,
        change: Option<Change<()>>,
        conflict: bool,
    ) -> anyhow::Result<()> {
        if let Some(change) = conflict
            .then_some('U')
            .or_else(|| change.as_ref().and_then(change_to_char))
        {
            writeln!(&mut self.0, "{change} {rela_path}")?;
        }
        Ok(())
    }
}

fn change_to_char(change: &Change<()>) -> Option<char> {
    // Known status letters: https://github.com/git/git/blob/6807fcfedab84bc8cd0fbf721bc13c4e68cda9ae/diff.h#L613
    Some(match change {
        Change::Removed => 'D',
        Change::Type => 'T',
        Change::Modification { .. } => 'M',
        Change::IntentToAdd => return None,
    })
}
