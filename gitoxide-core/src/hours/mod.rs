use std::{collections::BTreeSet, convert::Infallible, io, path::Path, sync::atomic::Ordering, time::Instant};

use anyhow::{anyhow, bail};
use git_repository as git;
use git_repository::{
    actor,
    bstr::{BStr, ByteSlice},
    interrupt,
    prelude::*,
    progress, Progress,
};

/// Additional configuration for the hours estimation functionality.
pub struct Context<W> {
    /// Ignore github bots which match the `[bot]` search string.
    pub ignore_bots: bool,
    /// Show personally identifiable information before the summary. Includes names and email addresses.
    pub show_pii: bool,
    /// Collect how many files have been added, removed and modified (without rename tracking).
    pub file_stats: bool,
    /// Collect how many lines in files have been added, removed and modified (without rename tracking).
    pub line_stats: bool,
    /// The amount of threads to use. If unset, use all cores, if 0 use al physical cores.
    pub threads: Option<usize>,
    /// Omit unifying identities by name and email which can lead to the same author appear multiple times
    /// due to using different names or email addresses.
    pub omit_unify_identities: bool,
    /// Where to write our output to
    pub out: W,
}

/// Estimate the hours it takes to produce the content of the repository in `_working_dir_`, with `_refname_` for
/// the start of the commit graph traversal.
///
/// * _working_dir_ - The directory containing a '.git/' folder.
/// * _refname_ - The name of the ref like 'main' or 'master' at which to start iterating the commit graph.
/// * _progress_ - A way to provide progress and performance information
pub fn estimate<W, P>(
    working_dir: &Path,
    rev_spec: &BStr,
    mut progress: P,
    Context {
        show_pii,
        ignore_bots,
        file_stats,
        line_stats,
        omit_unify_identities,
        threads,
        mut out,
    }: Context<W>,
) -> anyhow::Result<()>
where
    W: io::Write,
    P: Progress,
{
    let repo = git::discover(working_dir)?;
    let commit_id = repo.rev_parse_single(rev_spec)?.detach();
    let mut string_heap = BTreeSet::<&'static [u8]>::new();
    let needs_stats = file_stats || line_stats;
    let threads = {
        let t = threads.unwrap_or_else(num_cpus::get);
        (t == 0).then(num_cpus::get_physical).unwrap_or(t)
    };

    let (commit_authors, stats, is_shallow, skipped_merge_commits) = {
        let stat_progress = needs_stats.then(|| progress.add_child("extract stats")).map(|mut p| {
            p.init(None, progress::count("commits"));
            p
        });
        let stat_counter = stat_progress.as_ref().and_then(|p| p.counter());

        let change_progress = needs_stats.then(|| progress.add_child("find changes")).map(|mut p| {
            p.init(None, progress::count("modified files"));
            p
        });
        let change_counter = change_progress.as_ref().and_then(|p| p.counter());

        let lines_progress = line_stats.then(|| progress.add_child("find changes")).map(|mut p| {
            p.init(None, progress::count("diff lines"));
            p
        });
        let lines_counter = lines_progress.as_ref().and_then(|p| p.counter());

        let mut progress = progress.add_child("traverse commit graph");
        progress.init(None, progress::count("commits"));

        std::thread::scope(|scope| -> anyhow::Result<_> {
            let start = Instant::now();
            let (tx, rx) = std::sync::mpsc::channel::<(u32, Vec<u8>)>();
            let mailmap = repo.open_mailmap();

            let commit_thread = scope.spawn(move || -> anyhow::Result<Vec<_>> {
                let mut out = Vec::new();
                for (commit_idx, commit_data) in rx {
                    if let Ok(author) = git::objs::CommitRefIter::from_bytes(&commit_data)
                        .author()
                        .map(|author| mailmap.resolve_cow(author.trim()))
                    {
                        let mut string_ref = |s: &[u8]| -> &'static BStr {
                            match string_heap.get(s) {
                                Some(n) => n.as_bstr(),
                                None => {
                                    let sv: Vec<u8> = s.to_owned();
                                    string_heap.insert(Box::leak(sv.into_boxed_slice()));
                                    (*string_heap.get(s).expect("present")).as_ref()
                                }
                            }
                        };
                        let name = string_ref(author.name.as_ref());
                        let email = string_ref(author.email.as_ref());

                        out.push((
                            commit_idx,
                            actor::SignatureRef {
                                name,
                                email,
                                time: author.time,
                            },
                        ));
                    }
                }
                out.shrink_to_fit();
                out.sort_by(|a, b| {
                    a.1.email.cmp(b.1.email).then(
                        a.1.time
                            .seconds_since_unix_epoch
                            .cmp(&b.1.time.seconds_since_unix_epoch)
                            .reverse(),
                    )
                });
                Ok(out)
            });

            let (tx_tree_id, stat_threads) = needs_stats
                .then(|| {
                    let (tx, rx) =
                        crossbeam_channel::unbounded::<(u32, Option<git::hash::ObjectId>, git::hash::ObjectId)>();
                    let stat_workers = (0..threads)
                        .map(|_| {
                            scope.spawn({
                                let commit_counter = stat_counter.clone();
                                let change_counter = change_counter.clone();
                                let lines_counter = lines_counter.clone();
                                let mut repo = repo.clone();
                                repo.object_cache_size_if_unset(4 * 1024 * 1024);
                                let rx = rx.clone();
                                move || -> Result<_, git::object::tree::diff::for_each::Error> {
                                    let mut out = Vec::new();
                                    for (commit_idx, parent_commit, commit) in rx {
                                        if let Some(c) = commit_counter.as_ref() {
                                            c.fetch_add(1, Ordering::SeqCst);
                                        }
                                        if git::interrupt::is_triggered() {
                                            return Ok(out);
                                        }
                                        let mut files = FileStats::default();
                                        let mut lines = LineStats::default();
                                        let from = match parent_commit {
                                            Some(id) => {
                                                match repo.find_object(id).ok().and_then(|c| c.peel_to_tree().ok()) {
                                                    Some(tree) => tree,
                                                    None => continue,
                                                }
                                            }
                                            None => repo
                                                .find_object(git::hash::ObjectId::empty_tree(repo.object_hash()))
                                                .expect("always present")
                                                .into_tree(),
                                        };
                                        let to = match repo.find_object(commit).ok().and_then(|c| c.peel_to_tree().ok())
                                        {
                                            Some(c) => c,
                                            None => continue,
                                        };
                                        from.changes().track_filename().for_each_to_obtain_tree(&to, |change| {
                                            use git::object::tree::diff::change::Event::*;
                                            if let Some(c) = change_counter.as_ref() {
                                                c.fetch_add(1, Ordering::SeqCst);
                                            }
                                            match change.event {
                                                Addition { entry_mode, id } => {
                                                    if entry_mode.is_no_tree() {
                                                        files.added += 1;
                                                        add_lines(line_stats, lines_counter.as_deref(), &mut lines, id);
                                                    }
                                                }
                                                Deletion { entry_mode, id } => {
                                                    if entry_mode.is_no_tree() {
                                                        files.removed += 1;
                                                        remove_lines(
                                                            line_stats,
                                                            lines_counter.as_deref(),
                                                            &mut lines,
                                                            id,
                                                        );
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
                                                        files.added += 1;
                                                        add_lines(line_stats, lines_counter.as_deref(), &mut lines, id);
                                                    }
                                                    (true, false) => {
                                                        files.removed += 1;
                                                        add_lines(
                                                            line_stats,
                                                            lines_counter.as_deref(),
                                                            &mut lines,
                                                            previous_id,
                                                        );
                                                    }
                                                    (true, true) => {
                                                        files.modified += 1;
                                                        if line_stats {
                                                            let is_text_file = mime_guess::from_path(
                                                                git::path::from_bstr(change.location).as_ref(),
                                                            )
                                                            .first_or_text_plain()
                                                            .type_()
                                                                == mime_guess::mime::TEXT;
                                                            if let Some(Ok(diff)) =
                                                                is_text_file.then(|| change.event.diff()).flatten()
                                                            {
                                                                let mut nl = 0;
                                                                let counts = diff.line_counts();
                                                                nl += counts.insertions as usize
                                                                    + counts.removals as usize;
                                                                lines.added += counts.insertions as usize;
                                                                lines.removed += counts.removals as usize;
                                                                if let Some(c) = lines_counter.as_ref() {
                                                                    c.fetch_add(nl, Ordering::SeqCst);
                                                                }
                                                            }
                                                        }
                                                    }
                                                },
                                            }
                                            Ok::<_, Infallible>(Default::default())
                                        })?;
                                        out.push((commit_idx, files, lines));
                                    }
                                    Ok(out)
                                }
                            })
                        })
                        .collect::<Vec<_>>();
                    (Some(tx), stat_workers)
                })
                .unwrap_or_default();

            let mut commit_idx = 0_u32;
            let mut skipped_merge_commits = 0;
            let commit_iter = interrupt::Iter::new(
                commit_id.ancestors(|oid, buf| {
                    progress.inc();
                    repo.objects.find(oid, buf).map(|obj| {
                        tx.send((commit_idx, obj.data.to_owned())).ok();
                        if let Some((tx_tree, first_parent, commit)) = tx_tree_id.as_ref().and_then(|tx| {
                            let mut parents = git::objs::CommitRefIter::from_bytes(obj.data).parent_ids();
                            let res = parents
                                .next()
                                .map(|first_parent| (tx, Some(first_parent), oid.to_owned()));
                            match parents.next() {
                                Some(_) => {
                                    skipped_merge_commits += 1;
                                    None
                                }
                                None => res,
                            }
                        }) {
                            tx_tree.send((commit_idx, first_parent, commit)).ok();
                        }
                        commit_idx = commit_idx.checked_add(1).expect("less then 4 billion commits");
                        git::objs::CommitRefIter::from_bytes(obj.data)
                    })
                }),
                || anyhow!("Cancelled by user"),
            );
            let mut is_shallow = false;
            for c in commit_iter {
                match c? {
                    Ok(c) => c,
                    Err(git::traverse::commit::ancestors::Error::FindExisting { .. }) => {
                        is_shallow = true;
                        break;
                    }
                    Err(err) => return Err(err.into()),
                };
            }
            drop(tx);
            drop(tx_tree_id);
            progress.show_throughput(start);
            drop(progress);

            let stats_by_commit_idx = match stat_progress {
                Some(mut progress) => {
                    progress.set_max(Some(commit_idx as usize - skipped_merge_commits));
                    let mut stats = Vec::new();
                    for handle in stat_threads {
                        stats.extend(handle.join().expect("no panic")?);
                        if git::interrupt::is_triggered() {
                            bail!("Cancelled by user");
                        }
                    }
                    stats.sort_by_key(|t| t.0);
                    progress.show_throughput(start);
                    stats
                }
                None => Vec::new(),
            };
            if let Some(mut progress) = change_progress {
                progress.show_throughput(start);
            }
            if let Some(mut progress) = lines_progress {
                progress.show_throughput(start);
            }

            Ok((
                commit_thread.join().expect("no panic")?,
                stats_by_commit_idx,
                is_shallow,
                skipped_merge_commits,
            ))
        })?
    };

    if commit_authors.is_empty() {
        bail!("No commits to process");
    }

    let start = Instant::now();
    let mut current_email = &commit_authors[0].1.email;
    let mut slice_start = 0;
    let mut results_by_hours = Vec::new();
    let mut ignored_bot_commits = 0_u32;
    for (idx, (_, elm)) in commit_authors.iter().enumerate() {
        if elm.email != *current_email {
            let estimate = estimate_hours(&commit_authors[slice_start..idx], &stats);
            slice_start = idx;
            current_email = &elm.email;
            if ignore_bots && estimate.name.contains_str(b"[bot]") {
                ignored_bot_commits += estimate.num_commits;
                continue;
            }
            results_by_hours.push(estimate);
        }
    }
    if let Some(commits) = commit_authors.get(slice_start..) {
        results_by_hours.push(estimate_hours(commits, &stats));
    }

    let num_authors = results_by_hours.len();
    let mut results_by_hours = if !omit_unify_identities {
        deduplicate_identities(&results_by_hours)
    } else {
        results_by_hours
            .iter()
            .fold(Vec::with_capacity(results_by_hours.len()), |mut acc, e| {
                acc.push(e.into());
                acc
            })
    };
    let elapsed = start.elapsed();
    progress.done(format!(
        "Extracted and organized data from {} commits in {:?} ({:0.0} commits/s)",
        commit_authors.len(),
        elapsed,
        commit_authors.len() as f32 / elapsed.as_secs_f32()
    ));

    let num_unique_authors = results_by_hours.len();
    let (total_hours, total_commits, total_files, total_lines) = results_by_hours
        .iter()
        .map(|e| (e.hours, e.num_commits, e.files, e.lines))
        .reduce(|a, b| (a.0 + b.0, a.1 + b.1, a.2.clone().added(&b.2), a.3.clone().added(&b.3)))
        .expect("at least one commit at this point");
    if show_pii {
        results_by_hours.sort_by(|a, b| a.hours.partial_cmp(&b.hours).unwrap_or(std::cmp::Ordering::Equal));
        for entry in results_by_hours.iter() {
            entry.write_to(
                total_hours,
                file_stats.then(|| total_files),
                line_stats.then(|| total_lines),
                &mut out,
            )?;
            writeln!(out)?;
        }
    }
    writeln!(
        out,
        "total hours: {:.02}\ntotal 8h days: {:.02}\ntotal commits = {}{}\ntotal authors: {}",
        total_hours,
        total_hours / HOURS_PER_WORKDAY,
        total_commits,
        is_shallow.then(|| " (shallow)").unwrap_or_default(),
        num_authors
    )?;
    if file_stats {
        writeln!(
            out,
            "total files added/removed/modified/remaining: {}/{}/{}/{}",
            total_files.added,
            total_files.removed,
            total_files.modified,
            total_files.added - total_files.removed
        )?;
    }
    if line_stats {
        writeln!(
            out,
            "total lines added/removed/remaining: {}/{}/{}",
            total_lines.added,
            total_lines.removed,
            total_lines.added - total_lines.removed
        )?;
    }
    if !omit_unify_identities {
        writeln!(
            out,
            "total unique authors: {} ({:.02}% duplication)",
            num_unique_authors,
            (1.0 - (num_unique_authors as f32 / num_authors as f32)) * 100.0
        )?;
    }
    if ignored_bot_commits != 0 {
        writeln!(out, "commits by bots: {}", ignored_bot_commits)?;
    }
    if needs_stats && skipped_merge_commits != 0 {
        writeln!(out, "stats omitted for {} merge commits", skipped_merge_commits)?;
    }
    assert_eq!(
        total_commits,
        commit_authors.len() as u32 - ignored_bot_commits,
        "need to get all commits"
    );
    Ok(())
}

mod core;
use self::core::{deduplicate_identities, estimate_hours, HOURS_PER_WORKDAY};

mod util;
use util::{add_lines, remove_lines, FileStats, LineStats, WorkByEmail, WorkByPerson};
