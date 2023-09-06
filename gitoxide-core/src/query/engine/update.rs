use std::{
    convert::Infallible,
    sync::atomic::{AtomicUsize, Ordering},
    time::Instant,
};

use anyhow::{anyhow, bail};
use gix::{
    bstr::{BStr, BString, ByteSlice},
    features::progress,
    object::tree::diff::rewrites::CopySource,
    odb::FindExt,
    parallel::{InOrderIter, SequenceId},
    prelude::ObjectIdExt,
    Count, Progress,
};
use rusqlite::{params, Statement, Transaction};

use crate::query::Options;

pub fn update(
    repo: &gix::Repository,
    con: &mut rusqlite::Connection,
    progress: &mut impl gix::NestedProgress,
    mut err: impl std::io::Write,
    Options {
        object_cache_size_mb,
        find_copies_harder,
        threads,
    }: Options,
) -> anyhow::Result<Vec<gix::ObjectId>> {
    let commit_id = repo.head_id()?.detach();
    let threads = gix::features::parallel::num_threads(threads);

    let mut stat_progress = {
        let mut p = progress.add_child("extract stats");
        p.init(None, progress::count("commits"));
        p
    };
    let stat_counter = stat_progress.counter();

    let mut db_progress = {
        let mut p = progress.add_child("db cache");
        p.init(None, progress::count("events"));
        p
    };
    let commit_counter = db_progress.counter();

    let change_progress = {
        let mut p = progress.add_child("find changes");
        p.init(None, progress::count("modified files"));
        p
    };
    let change_counter = change_progress.counter();

    let lines_progress = {
        let mut p = progress.add_child("find changes");
        p.init(None, progress::count("diff lines"));
        p
    };
    let lines_counter = lines_progress.counter();

    let mut traverse_progress = progress.add_child("traverse commit graph");
    traverse_progress.init(None, progress::count("commits"));

    let out = std::thread::scope(|scope| -> anyhow::Result<_> {
        struct CommitDiffStats {
            /// The id of the commit which was diffed with its predecessor
            id: gix::hash::ObjectId,
            changes: Vec<FileChange>,
        }
        let start = Instant::now();
        let (tx_stats, rx_stats) = std::sync::mpsc::channel::<Result<(SequenceId, Vec<CommitDiffStats>), Infallible>>();

        let mut all_commits =
            Vec::with_capacity(con.query_row("SELECT  COUNT(hash) from commits", [], |r| r.get::<_, usize>(0))?);
        for item in con
            .prepare("SELECT hash from commits ORDER BY ROWID")?
            .query_map([], |r| {
                Ok(gix::ObjectId::try_from(r.get_ref(0)?.as_bytes()?)
                    .unwrap_or_else(|_| gix::ObjectId::null(gix::hash::Kind::Sha1)))
            })?
        {
            all_commits.push(item?);
        }
        let mut known_commits = all_commits.clone();
        known_commits.sort();

        let db_thread = scope.spawn({
            move || -> anyhow::Result<()> {
                let trans = con.transaction()?;
                {
                    let Updates {
                        mut new_commit,
                        mut insert_commit_file,
                        mut insert_commit_file_with_source,
                        mut insert_file_path,
                    } = Updates::new(&trans)?;
                    for stats in InOrderIter::from(rx_stats.into_iter()) {
                        for CommitDiffStats { id, changes } in stats.expect("infallible") {
                            new_commit.execute(params![id.as_bytes()])?;
                            for change in changes {
                                insert_file_path.execute(params![change.relpath.to_str_lossy()])?;
                                let (has_diff, lines) = change.lines.map(|l| (true, l)).unwrap_or_default();
                                if let Some(source_relpath) = change.source_relpath {
                                    insert_file_path.execute(params![source_relpath.to_str_lossy()])?;
                                    insert_commit_file_with_source.execute(params![
                                        id.as_bytes(),
                                        change.relpath.to_str_lossy(),
                                        has_diff,
                                        lines.added,
                                        lines.removed,
                                        lines.before,
                                        lines.after,
                                        change.mode as usize,
                                        source_relpath.to_str_lossy(),
                                    ])?;
                                } else {
                                    insert_commit_file.execute(params![
                                        id.as_bytes(),
                                        change.relpath.to_str_lossy(),
                                        has_diff,
                                        lines.added,
                                        lines.removed,
                                        lines.before,
                                        lines.after,
                                        change.mode as usize,
                                    ])?;
                                }
                            }
                            commit_counter.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                }
                trans.commit()?;
                Ok(())
            }
        });

        let rewrites = {
            let mut r =
                gix::object::tree::diff::Rewrites::try_from_config(&repo.config_snapshot(), true)?.unwrap_or_default();
            r.copies = Some(gix::object::tree::diff::rewrites::Copies {
                source: if find_copies_harder {
                    CopySource::FromSetOfModifiedFilesAndSourceTree
                } else {
                    CopySource::FromSetOfModifiedFiles
                },
                percentage: None,
            });
            r
        };
        struct Task {
            commit: gix::hash::ObjectId,
            parent_commit: Option<gix::hash::ObjectId>,
            compute_stats: bool,
        }
        let (tx_tree_ids, stat_threads) = {
            let (tx, rx) = crossbeam_channel::unbounded::<(SequenceId, Vec<Task>)>();
            let stat_workers = (0..threads)
                .map(|_| {
                    scope.spawn({
                        let stat_counter = stat_counter.clone();
                        let change_counter = change_counter.clone();
                        let lines_counter = lines_counter.clone();
                        let tx_stats = tx_stats.clone();
                        let mut repo = repo.clone();
                        repo.object_cache_size_if_unset((object_cache_size_mb * 1024 * 1024) / threads);
                        let rx = rx.clone();
                        move || -> anyhow::Result<()> {
                            for (chunk_id, chunk) in rx {
                                let mut out_chunk = Vec::with_capacity(chunk.len());
                                for Task {
                                    parent_commit,
                                    commit,
                                    compute_stats,
                                } in chunk
                                {
                                    stat_counter.fetch_add(1, Ordering::SeqCst);
                                    if gix::interrupt::is_triggered() {
                                        return Ok(());
                                    }
                                    let mut out = Vec::new();
                                    if compute_stats {
                                        let from = match parent_commit {
                                            Some(id) => {
                                                match repo.find_object(id).ok().and_then(|c| c.peel_to_tree().ok()) {
                                                    Some(tree) => tree,
                                                    None => continue,
                                                }
                                            }
                                            None => repo.empty_tree(),
                                        };
                                        let to = match repo.find_object(commit).ok().and_then(|c| c.peel_to_tree().ok())
                                        {
                                            Some(c) => c,
                                            None => continue,
                                        };
                                        from.changes()?
                                            .track_path()
                                            .track_rewrites(Some(rewrites))
                                            .for_each_to_obtain_tree(&to, |change| {
                                                use gix::object::tree::diff::change::Event::*;
                                                change_counter.fetch_add(1, Ordering::SeqCst);
                                                match change.event {
                                                    Addition { entry_mode, id } => {
                                                        if entry_mode.is_blob_or_symlink() {
                                                            add_lines(&mut out, change.location, &lines_counter, id);
                                                        }
                                                    }
                                                    Modification {
                                                        entry_mode,
                                                        previous_entry_mode,
                                                        id,
                                                        previous_id,
                                                    } => match (previous_entry_mode.is_blob(), entry_mode.is_blob()) {
                                                        (false, false) => {}
                                                        (false, true) => {
                                                            add_lines(&mut out, change.location, &lines_counter, id);
                                                        }
                                                        (true, false) => {
                                                            add_lines(
                                                                &mut out,
                                                                change.location,
                                                                &lines_counter,
                                                                previous_id,
                                                            );
                                                        }
                                                        (true, true) => {
                                                            // TODO: use git attributes here to know if it's a binary file or not.
                                                            if let Some(Ok(diff)) = change.event.diff() {
                                                                let mut nl = 0;
                                                                let tokens = diff.line_tokens();
                                                                let counts = gix::diff::blob::diff(
                                                                    diff.algo,
                                                                    &tokens,
                                                                    gix::diff::blob::sink::Counter::default(),
                                                                );
                                                                nl += counts.insertions as usize
                                                                    + counts.removals as usize;
                                                                let lines = LineStats {
                                                                    added: counts.insertions as usize,
                                                                    removed: counts.removals as usize,
                                                                    before: tokens.before.len(),
                                                                    after: tokens.after.len(),
                                                                };
                                                                lines_counter.fetch_add(nl, Ordering::SeqCst);
                                                                out.push(FileChange {
                                                                    relpath: change.location.to_owned(),
                                                                    mode: FileMode::Modified,
                                                                    source_relpath: None,
                                                                    lines: Some(lines),
                                                                });
                                                            }
                                                        }
                                                    },
                                                    Deletion { entry_mode, id } => {
                                                        if entry_mode.is_blob_or_symlink() {
                                                            remove_lines(&mut out, change.location, &lines_counter, id);
                                                        }
                                                    }
                                                    Rewrite {
                                                        source_location,
                                                        diff,
                                                        copy,
                                                        ..
                                                    } => {
                                                        out.push(FileChange {
                                                            relpath: change.location.to_owned(),
                                                            source_relpath: Some(source_location.to_owned()),
                                                            mode: if copy { FileMode::Copy } else { FileMode::Rename },
                                                            lines: diff.map(|d| LineStats {
                                                                added: d.insertions as usize,
                                                                removed: d.removals as usize,
                                                                before: d.before as usize,
                                                                after: d.after as usize,
                                                            }),
                                                        });
                                                    }
                                                }
                                                Ok::<_, Infallible>(Default::default())
                                            })?;
                                        out_chunk.push(CommitDiffStats {
                                            id: commit,
                                            changes: out,
                                        });
                                    } else {
                                        out_chunk.push(CommitDiffStats {
                                            id: commit,
                                            changes: Vec::new(),
                                        })
                                    }
                                }
                                if tx_stats.send(Ok((chunk_id, out_chunk))).is_err() {
                                    bail!("Thread failed to send result");
                                }
                            }
                            Ok(())
                        }
                    })
                })
                .collect::<Vec<_>>();
            (tx, stat_workers)
        };
        drop(tx_stats);

        const CHUNK_SIZE: usize = 50;
        let mut chunk = Vec::with_capacity(CHUNK_SIZE);
        let mut chunk_id: SequenceId = 0;
        let commit_iter = gix::interrupt::Iter::new(
            commit_id.ancestors(|oid, buf| -> Result<_, gix::object::find::existing::Error> {
                let obj = repo.objects.find(oid, buf)?;
                traverse_progress.inc();
                if known_commits.binary_search(&oid.to_owned()).is_err() {
                    let res = {
                        let mut parents = gix::objs::CommitRefIter::from_bytes(obj.data).parent_ids();
                        let res = parents.next().map(|first_parent| (Some(first_parent), oid.to_owned()));
                        match parents.next() {
                            Some(_) => None,
                            None => res,
                        }
                    };
                    if let Some((first_parent, commit)) = res {
                        chunk.push(Task {
                            parent_commit: first_parent,
                            commit,
                            compute_stats: true,
                        });
                    } else {
                        chunk.push(Task {
                            parent_commit: None,
                            commit: oid.to_owned(),
                            compute_stats: false,
                        });
                    }
                    if chunk.len() == CHUNK_SIZE {
                        tx_tree_ids
                            .send((chunk_id, std::mem::replace(&mut chunk, Vec::with_capacity(CHUNK_SIZE))))
                            .ok();
                        chunk_id += 1;
                    }
                }
                Ok(gix::objs::CommitRefIter::from_bytes(obj.data))
            }),
            || anyhow!("Cancelled by user"),
        );
        let mut commits = Vec::new();
        for c in commit_iter {
            match c?.map(|c| c.id) {
                Ok(c) => {
                    if known_commits.binary_search(&c).is_err() {
                        commits.push(c);
                    } else {
                        break;
                    }
                }
                Err(gix::traverse::commit::ancestors::Error::FindExisting { .. }) => {
                    writeln!(err, "shallow repository - commit history is truncated").ok();
                    break;
                }
                Err(err) => return Err(err.into()),
            };
        }
        tx_tree_ids.send((chunk_id, chunk)).ok();
        drop(tx_tree_ids);
        let saw_new_commits = !commits.is_empty();
        if saw_new_commits {
            traverse_progress.show_throughput(start);
        }
        drop(traverse_progress);

        let stat_max = Some(commits.len());
        stat_progress.set_max(stat_max);
        db_progress.set_max(stat_max);
        for handle in stat_threads {
            handle.join().expect("no panic")?;
            if gix::interrupt::is_triggered() {
                bail!("Cancelled by user");
            }
        }
        if saw_new_commits {
            stat_progress.show_throughput(start);
            change_progress.show_throughput(start);
            lines_progress.show_throughput(start);
        }

        db_thread.join().expect("no panic")?;
        if saw_new_commits {
            db_progress.show_throughput(start);
        } else {
            db_progress.info("up to date".into());
        }

        commits.extend(all_commits);
        Ok(commits)
    })?;

    Ok(out)
}

fn add_lines(out: &mut Vec<FileChange>, path: &BStr, lines_counter: &AtomicUsize, id: gix::Id<'_>) {
    if let Ok(blob) = id.object() {
        let nl = blob.data.lines_with_terminator().count();
        let mut lines = LineStats::default();
        lines.added += nl;
        lines.after = nl;
        lines_counter.fetch_add(nl, Ordering::SeqCst);
        out.push(FileChange {
            relpath: path.to_owned(),
            mode: FileMode::Added,
            source_relpath: None,
            lines: Some(lines),
        });
    }
}

fn remove_lines(out: &mut Vec<FileChange>, path: &BStr, lines_counter: &AtomicUsize, id: gix::Id<'_>) {
    if let Ok(blob) = id.object() {
        let mut lines = LineStats::default();
        let nl = blob.data.lines_with_terminator().count();
        lines.removed += nl;
        lines.before = nl;
        lines_counter.fetch_add(nl, Ordering::SeqCst);
        out.push(FileChange {
            relpath: path.to_owned(),
            mode: FileMode::Removed,
            source_relpath: None,
            lines: Some(lines),
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub enum FileMode {
    Added = 1,
    Removed = 2,
    Modified = 3,
    Rename = 4,
    Copy = 5,
}

impl FileMode {
    pub fn as_str(&self) -> &'static str {
        use FileMode::*;
        match self {
            Added => "+",
            Removed => "-",
            Modified => "Δ",
            Rename => "➡",
            Copy => "⏸",
        }
    }
    pub fn from_usize(mode: usize) -> Option<Self> {
        use FileMode::*;
        match mode {
            1 => Added,
            2 => Removed,
            3 => Modified,
            4 => Rename,
            5 => Copy,
            _ => return None,
        }
        .into()
    }
}

#[derive(Debug)]
struct FileChange {
    relpath: BString,
    mode: FileMode,
    source_relpath: Option<BString>,
    lines: Option<LineStats>,
}

/// Line statistics for a particular commit.
#[derive(Debug, Default, Copy, Clone)]
struct LineStats {
    /// amount of added lines
    added: usize,
    /// amount of removed lines
    removed: usize,
    /// the amount of lines before the change.
    before: usize,
    /// the amount of lines after the change.
    after: usize,
}

struct Updates<'a> {
    new_commit: Statement<'a>,
    insert_commit_file: Statement<'a>,
    insert_commit_file_with_source: Statement<'a>,
    insert_file_path: Statement<'a>,
}

impl<'a> Updates<'a> {
    fn new(trans: &'a Transaction<'_>) -> rusqlite::Result<Self> {
        let new_commit = trans.prepare(
            r#"INSERT INTO
               commits(hash)
               VALUES(?)"#,
        )?;
        let insert_commit_file = trans.prepare(
            r#"
               INSERT INTO
               commit_file(hash, file_id, has_diff, lines_added, lines_removed, lines_before, lines_after, mode)
               VALUES(?, (SELECT files.file_id FROM files WHERE files.file_path = ?), ?, ?, ?, ?, ?, ?)
            "#,
        )?;
        let insert_commit_file_with_source = trans.prepare(
                r#"
               INSERT INTO
               commit_file(hash, file_id, has_diff, lines_added, lines_removed, lines_before, lines_after, mode, source_file_id)
               VALUES(?, (SELECT files.file_id FROM files WHERE files.file_path = ?), ?, ?, ?, ?, ?, ?, (SELECT files.file_id FROM files WHERE files.file_path = ?))
            "#,
            )?;

        let insert_file_path = trans.prepare(
            r#"
               INSERT OR IGNORE INTO
               files(file_path)
               VALUES(?)
            "#,
        )?;
        Ok(Updates {
            new_commit,
            insert_commit_file,
            insert_commit_file_with_source,
            insert_file_path,
        })
    }
}
