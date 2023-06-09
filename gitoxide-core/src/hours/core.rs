use std::collections::{hash_map::Entry, HashMap};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use gix::bstr::BStr;
use gix::odb::FindExt;
use itertools::Itertools;
use smallvec::SmallVec;

use crate::hours::util::{add_lines, remove_lines};
use crate::hours::{CommitIdx, FileStats, LineStats, WorkByEmail, WorkByPerson};

const MINUTES_PER_HOUR: f32 = 60.0;
pub const HOURS_PER_WORKDAY: f32 = 8.0;

pub fn estimate_hours(
    commits: &[(u32, gix::actor::SignatureRef<'static>)],
    stats: &[(u32, FileStats, LineStats)],
) -> WorkByEmail {
    assert!(!commits.is_empty());
    const MAX_COMMIT_DIFFERENCE_IN_MINUTES: f32 = 2.0 * MINUTES_PER_HOUR;
    const FIRST_COMMIT_ADDITION_IN_MINUTES: f32 = 2.0 * MINUTES_PER_HOUR;

    let hours_for_commits = commits.iter().map(|t| &t.1).rev().tuple_windows().fold(
        0_f32,
        |hours, (cur, next): (&gix::actor::SignatureRef<'_>, &gix::actor::SignatureRef<'_>)| {
            let change_in_minutes = (next
                .time
                .seconds_since_unix_epoch
                .saturating_sub(cur.time.seconds_since_unix_epoch)) as f32
                / MINUTES_PER_HOUR;
            if change_in_minutes < MAX_COMMIT_DIFFERENCE_IN_MINUTES {
                hours + change_in_minutes / MINUTES_PER_HOUR
            } else {
                hours + (FIRST_COMMIT_ADDITION_IN_MINUTES / MINUTES_PER_HOUR)
            }
        },
    );

    let author = &commits[0].1;
    let (files, lines) = (!stats.is_empty())
        .then(|| {
            commits
                .iter()
                .map(|t| &t.0)
                .fold((FileStats::default(), LineStats::default()), |mut acc, id| match stats
                    .binary_search_by(|t| t.0.cmp(id))
                {
                    Ok(idx) => {
                        let t = &stats[idx];
                        acc.0.add(&t.1);
                        acc.1.add(&t.2);
                        acc
                    }
                    Err(_) => acc,
                })
        })
        .unwrap_or_default();
    WorkByEmail {
        name: author.name,
        email: author.email,
        hours: FIRST_COMMIT_ADDITION_IN_MINUTES / 60.0 + hours_for_commits,
        num_commits: commits.len() as u32,
        files,
        lines,
    }
}

type CommitChangeLineCounters = (
    Option<Arc<AtomicUsize>>,
    Option<Arc<AtomicUsize>>,
    Option<Arc<AtomicUsize>>,
);

type SpawnResultWithReturnChannelAndWorkers<'scope> = (
    crossbeam_channel::Sender<Vec<(CommitIdx, Option<gix::hash::ObjectId>, gix::hash::ObjectId)>>,
    Vec<std::thread::ScopedJoinHandle<'scope, anyhow::Result<Vec<(CommitIdx, FileStats, LineStats)>>>>,
);

pub fn spawn_tree_delta_threads<'scope>(
    scope: &'scope std::thread::Scope<'scope, '_>,
    threads: usize,
    line_stats: bool,
    repo: gix::Repository,
    stat_counters: CommitChangeLineCounters,
) -> SpawnResultWithReturnChannelAndWorkers<'scope> {
    let (tx, rx) = crossbeam_channel::unbounded::<Vec<(CommitIdx, Option<gix::hash::ObjectId>, gix::hash::ObjectId)>>();
    let stat_workers = (0..threads)
        .map(|_| {
            scope.spawn({
                let stats_counters = stat_counters.clone();
                let mut repo = repo.clone();
                repo.object_cache_size_if_unset((850 * 1024 * 1024) / threads);
                let rx = rx.clone();
                move || -> Result<_, anyhow::Error> {
                    let mut out = Vec::new();
                    let (commit_counter, change_counter, lines_counter) = stats_counters;
                    let mut attributes = line_stats
                        .then(|| -> anyhow::Result<_> {
                            repo.index_or_load_from_head().map_err(Into::into).and_then(|index| {
                                repo.attributes(
                                    &index,
                                    gix::worktree::cache::state::attributes::Source::IdMapping,
                                    gix::worktree::cache::state::ignore::Source::IdMapping,
                                    None,
                                )
                                .map_err(Into::into)
                                .map(|attrs| {
                                    let matches = attrs.selected_attribute_matches(["binary", "text"]);
                                    (attrs, matches)
                                })
                            })
                        })
                        .transpose()?;
                    for chunk in rx {
                        for (commit_idx, parent_commit, commit) in chunk {
                            if let Some(c) = commit_counter.as_ref() {
                                c.fetch_add(1, Ordering::SeqCst);
                            }
                            if gix::interrupt::is_triggered() {
                                return Ok(out);
                            }
                            let mut files = FileStats::default();
                            let mut lines = LineStats::default();
                            let from = match parent_commit {
                                Some(id) => match repo.find_object(id).ok().and_then(|c| c.peel_to_tree().ok()) {
                                    Some(tree) => tree,
                                    None => continue,
                                },
                                None => repo.empty_tree(),
                            };
                            let to = match repo.find_object(commit).ok().and_then(|c| c.peel_to_tree().ok()) {
                                Some(c) => c,
                                None => continue,
                            };
                            from.changes()?
                                .track_filename()
                                .track_rewrites(None)
                                .for_each_to_obtain_tree(&to, |change| {
                                    use gix::object::tree::diff::change::Event::*;
                                    if let Some(c) = change_counter.as_ref() {
                                        c.fetch_add(1, Ordering::SeqCst);
                                    }
                                    match change.event {
                                        Rewrite { .. } => {
                                            unreachable!("we turned that off")
                                        }
                                        Addition { entry_mode, id } => {
                                            if entry_mode.is_no_tree() {
                                                files.added += 1;
                                                add_lines(line_stats, lines_counter.as_deref(), &mut lines, id);
                                            }
                                        }
                                        Deletion { entry_mode, id } => {
                                            if entry_mode.is_no_tree() {
                                                files.removed += 1;
                                                remove_lines(line_stats, lines_counter.as_deref(), &mut lines, id);
                                            }
                                        }
                                        Modification {
                                            entry_mode,
                                            previous_entry_mode,
                                            id,
                                            previous_id,
                                        } => {
                                            match (previous_entry_mode.is_blob(), entry_mode.is_blob()) {
                                                (false, false) => {}
                                                (false, true) => {
                                                    files.added += 1;
                                                    add_lines(line_stats, lines_counter.as_deref(), &mut lines, id);
                                                }
                                                (true, false) => {
                                                    files.removed += 1;
                                                    remove_lines(
                                                        line_stats,
                                                        lines_counter.as_deref(),
                                                        &mut lines,
                                                        previous_id,
                                                    );
                                                }
                                                (true, true) => {
                                                    files.modified += 1;
                                                    if let Some((attrs, matches)) = attributes.as_mut() {
                                                        let entry = attrs.at_entry(
                                                            change.location,
                                                            Some(false),
                                                            |id, buf| repo.objects.find_blob(id, buf),
                                                        )?;
                                                        let is_text_file = if entry.matching_attributes(matches) {
                                                            let attrs: SmallVec<[_; 2]> =
                                                                matches.iter_selected().collect();
                                                            let binary = &attrs[0];
                                                            let text = &attrs[1];
                                                            !binary.assignment.state.is_set()
                                                                && !text.assignment.state.is_unset()
                                                        } else {
                                                            // In the absence of binary or text markers, we assume it's text.
                                                            true
                                                        };

                                                        if let Some(Ok(diff)) =
                                                            is_text_file.then(|| change.event.diff()).flatten()
                                                        {
                                                            let mut nl = 0;
                                                            let counts = diff.line_counts();
                                                            nl += counts.insertions as usize + counts.removals as usize;
                                                            lines.added += counts.insertions as usize;
                                                            lines.removed += counts.removals as usize;
                                                            if let Some(c) = lines_counter.as_ref() {
                                                                c.fetch_add(nl, Ordering::SeqCst);
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Ok::<_, std::io::Error>(Default::default())
                                })?;
                            out.push((commit_idx, files, lines));
                        }
                    }
                    Ok(out)
                }
            })
        })
        .collect::<Vec<_>>();
    (tx, stat_workers)
}

pub fn deduplicate_identities(persons: &[WorkByEmail]) -> Vec<WorkByPerson> {
    let mut email_to_index = HashMap::<&'static BStr, usize>::with_capacity(persons.len());
    let mut name_to_index = HashMap::<&'static BStr, usize>::with_capacity(persons.len());
    let mut out = Vec::<WorkByPerson>::with_capacity(persons.len());
    for person_by_email in persons {
        match email_to_index.entry(person_by_email.email) {
            Entry::Occupied(email_entry) => {
                out[*email_entry.get()].merge(person_by_email);
                name_to_index.insert(person_by_email.name, *email_entry.get());
            }
            Entry::Vacant(email_entry) => match name_to_index.entry(person_by_email.name) {
                Entry::Occupied(name_entry) => {
                    out[*name_entry.get()].merge(person_by_email);
                    email_entry.insert(*name_entry.get());
                }
                Entry::Vacant(name_entry) => {
                    let idx = out.len();
                    name_entry.insert(idx);
                    email_entry.insert(idx);
                    out.push(person_by_email.into());
                }
            },
        }
    }
    out
}
