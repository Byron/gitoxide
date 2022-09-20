use std::collections::BTreeSet;
use std::convert::Infallible;
use std::sync::atomic::Ordering;
use std::{
    collections::{hash_map::Entry, HashMap},
    io,
    path::Path,
    time::Instant,
};

use anyhow::{anyhow, bail};
use git_repository as git;
use git_repository::bstr::BStr;
use git_repository::{actor, bstr::ByteSlice, interrupt, prelude::*, progress, Progress};
use itertools::Itertools;

/// Additional configuration for the hours estimation functionality.
pub struct Context<W> {
    /// Ignore github bots which match the `[bot]` search string.
    pub ignore_bots: bool,
    /// Show personally identifiable information before the summary. Includes names and email addresses.
    pub show_pii: bool,
    /// Collect additional information like tree changes and changed lines.
    pub stats: bool,
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
        stats,
        omit_unify_identities,
        mut out,
    }: Context<W>,
) -> anyhow::Result<()>
where
    W: io::Write,
    P: Progress,
{
    let repo = git::discover(working_dir)?.apply_environment();
    let commit_id = repo.rev_parse_single(rev_spec)?.detach();
    let mut string_heap = BTreeSet::<&'static [u8]>::new();

    let (commit_authors, stats, is_shallow) = {
        let stat_progress = stats.then(|| progress.add_child("extract stats")).map(|mut p| {
            p.init(None, progress::count("commits"));
            p
        });
        let stat_counter = stat_progress.as_ref().and_then(|p| p.counter());

        let mut progress = progress.add_child("traverse commit graph");
        progress.init(None, progress::count("commits"));

        std::thread::scope(|scope| -> anyhow::Result<_> {
            let start = Instant::now();
            let (tx, rx) = std::sync::mpsc::channel::<(u32, Vec<u8>)>();
            let mailmap = repo.open_mailmap();

            let commit_thread = scope.spawn(move || -> anyhow::Result<Vec<_>> {
                let mut out = Vec::new();
                for (commit_idx, commit_data) in rx {
                    if let Some(author) = git::objs::CommitRefIter::from_bytes(&commit_data)
                        .author()
                        .map(|author| mailmap.resolve_cow(author.trim()))
                        .ok()
                    {
                        let mut string_ref = |s: &[u8]| -> &'static BStr {
                            match string_heap.get(s) {
                                Some(n) => n.as_bstr(),
                                None => {
                                    let sv: Vec<u8> = s.to_owned().into();
                                    string_heap.insert(Box::leak(sv.into_boxed_slice()));
                                    (*string_heap.get(s).expect("present")).as_ref()
                                }
                            }
                        };
                        let name = string_ref(author.name.as_ref());
                        let email = string_ref(&author.email.as_ref());

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
                    a.1.email.cmp(&b.1.email).then(
                        a.1.time
                            .seconds_since_unix_epoch
                            .cmp(&b.1.time.seconds_since_unix_epoch)
                            .reverse(),
                    )
                });
                Ok(out)
            });

            let (tx_tree_id, stat_threads) = stats
                .then(|| {
                    let num_threads = num_cpus::get().saturating_sub(1 /*main thread*/).max(1);
                    let (tx, rx) = flume::unbounded::<(u32, Option<git::hash::ObjectId>, git::hash::ObjectId)>();
                    let stat_workers = (0..num_threads)
                        .map(|_| {
                            scope.spawn({
                                let counter = stat_counter.clone();
                                let mut repo = repo.clone();
                                repo.object_cache_size_if_unset(4 * 1024 * 1024);
                                let rx = rx.clone();
                                move || -> Result<_, git::object::tree::diff::Error> {
                                    let mut out = Vec::new();
                                    for (commit_idx, parent_commit, commit) in rx {
                                        if let Some(c) = counter.as_ref() {
                                            c.fetch_add(1, Ordering::SeqCst);
                                        }
                                        let mut stat = Stats::default();
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
                                        from.changes().for_each_to_obtain_tree(&to, |change| {
                                            use git::object::tree::diff::change::Event::*;
                                            match change.event {
                                                Addition { entry_mode, .. } => {
                                                    if entry_mode.is_no_tree() {
                                                        stat.added += 1
                                                    }
                                                }
                                                Deletion { entry_mode, .. } => {
                                                    if entry_mode.is_no_tree() {
                                                        stat.removed += 1
                                                    }
                                                }
                                                Modification { entry_mode, .. } => {
                                                    if entry_mode.is_no_tree() {
                                                        stat.modified += 1;
                                                    }
                                                }
                                            }
                                            Ok::<_, Infallible>(Default::default())
                                        })?;
                                        out.push((commit_idx, stat));
                                    }
                                    Ok(out)
                                }
                            })
                        })
                        .collect::<Vec<_>>();
                    (Some(tx), stat_workers)
                })
                .unwrap_or_else(Default::default);

            let mut commit_idx = 0_u32;
            let commit_iter = interrupt::Iter::new(
                commit_id.ancestors(|oid, buf| {
                    progress.inc();
                    repo.objects.find(oid, buf).map(|obj| {
                        tx.send((commit_idx, obj.data.to_owned())).ok();
                        if let Some((tx_tree, first_parent, commit)) = tx_tree_id.as_ref().and_then(|tx| {
                            git::objs::CommitRefIter::from_bytes(obj.data)
                                .parent_ids()
                                .next()
                                .map(|first_parent| (tx, Some(first_parent), oid.to_owned()))
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
                    progress.set_max(Some(commit_idx as usize));
                    let mut stats = Vec::new();
                    for handle in stat_threads {
                        stats.extend(handle.join().expect("no panic")?);
                    }
                    stats.sort_by_key(|t| t.0);
                    progress.show_throughput(start);
                    stats
                }
                None => Vec::new(),
            };

            Ok((
                commit_thread.join().expect("no panic")?,
                stats_by_commit_idx,
                is_shallow,
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
    let (total_hours, total_commits, total_stats) = results_by_hours
        .iter()
        .map(|e| (e.hours, e.num_commits, e.stats))
        .reduce(|a, b| (a.0 + b.0, a.1 + b.1, a.2.clone().added(&b.2)))
        .expect("at least one commit at this point");
    if show_pii {
        results_by_hours.sort_by(|a, b| a.hours.partial_cmp(&b.hours).unwrap_or(std::cmp::Ordering::Equal));
        let show_stats = !stats.is_empty();
        for entry in results_by_hours.iter() {
            entry.write_to(total_hours, show_stats, &mut out)?;
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
    if !stats.is_empty() {
        writeln!(
            out,
            "total files added/removed/modified: {}/{}/{}",
            total_stats.added, total_stats.removed, total_stats.modified
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
        writeln!(out, "commits by bots: {}", ignored_bot_commits,)?;
    }
    assert_eq!(
        total_commits,
        commit_authors.len() as u32 - ignored_bot_commits,
        "need to get all commits"
    );
    Ok(())
}

const MINUTES_PER_HOUR: f32 = 60.0;
const HOURS_PER_WORKDAY: f32 = 8.0;

fn estimate_hours(commits: &[(u32, actor::SignatureRef<'static>)], stats: &[(u32, Stats)]) -> WorkByEmail {
    assert!(!commits.is_empty());
    const MAX_COMMIT_DIFFERENCE_IN_MINUTES: f32 = 2.0 * MINUTES_PER_HOUR;
    const FIRST_COMMIT_ADDITION_IN_MINUTES: f32 = 2.0 * MINUTES_PER_HOUR;

    let hours_for_commits = commits.iter().map(|t| &t.1).rev().tuple_windows().fold(
        0_f32,
        |hours, (cur, next): (&actor::SignatureRef<'_>, &actor::SignatureRef<'_>)| {
            let change_in_minutes = (next
                .time
                .seconds_since_unix_epoch
                .saturating_sub(cur.time.seconds_since_unix_epoch)) as f32
                / MINUTES_PER_HOUR;
            if change_in_minutes < MAX_COMMIT_DIFFERENCE_IN_MINUTES {
                hours + change_in_minutes as f32 / MINUTES_PER_HOUR
            } else {
                hours + (FIRST_COMMIT_ADDITION_IN_MINUTES / MINUTES_PER_HOUR)
            }
        },
    );

    let author = &commits[0].1;
    WorkByEmail {
        name: author.name,
        email: author.email,
        hours: FIRST_COMMIT_ADDITION_IN_MINUTES / 60.0 + hours_for_commits,
        num_commits: commits.len() as u32,
        stats: (!stats.is_empty())
            .then(|| {
                commits.iter().map(|t| &t.0).fold(Stats::default(), |mut acc, id| {
                    match stats.binary_search_by(|t| t.0.cmp(id)) {
                        Ok(idx) => {
                            acc.add(&stats[idx].1);
                            acc
                        }
                        Err(_) => acc,
                    }
                })
            })
            .unwrap_or_default(),
    }
}

fn deduplicate_identities(persons: &[WorkByEmail]) -> Vec<WorkByPerson> {
    let mut email_to_index = HashMap::<&'static BStr, usize>::with_capacity(persons.len());
    let mut name_to_index = HashMap::<&'static BStr, usize>::with_capacity(persons.len());
    let mut out = Vec::<WorkByPerson>::with_capacity(persons.len());
    for person_by_email in persons {
        match email_to_index.entry(person_by_email.email) {
            Entry::Occupied(email_entry) => {
                out[*email_entry.get()].merge(person_by_email);
                name_to_index.insert(&person_by_email.name, *email_entry.get());
            }
            Entry::Vacant(email_entry) => match name_to_index.entry(&person_by_email.name) {
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

#[derive(Debug)]
struct WorkByPerson {
    name: Vec<&'static BStr>,
    email: Vec<&'static BStr>,
    hours: f32,
    num_commits: u32,
    stats: Stats,
}

impl<'a> WorkByPerson {
    fn merge(&mut self, other: &'a WorkByEmail) {
        if !self.name.contains(&&other.name) {
            self.name.push(&other.name);
        }
        if !self.email.contains(&&other.email) {
            self.email.push(&other.email);
        }
        self.num_commits += other.num_commits;
        self.hours += other.hours;
        self.stats.add(&other.stats);
    }
}

impl<'a> From<&'a WorkByEmail> for WorkByPerson {
    fn from(w: &'a WorkByEmail) -> Self {
        WorkByPerson {
            name: vec![w.name],
            email: vec![w.email],
            hours: w.hours,
            num_commits: w.num_commits,
            stats: w.stats,
        }
    }
}

impl WorkByPerson {
    fn write_to(&self, total_hours: f32, show_stats: bool, mut out: impl std::io::Write) -> std::io::Result<()> {
        writeln!(
            out,
            "{} <{}>",
            self.name.iter().join(", "),
            self.email.iter().join(", ")
        )?;
        writeln!(out, "{} commits found", self.num_commits)?;
        writeln!(
            out,
            "total time spent: {:.02}h ({:.02} 8h days, {:.02}%)",
            self.hours,
            self.hours / HOURS_PER_WORKDAY,
            (self.hours / total_hours) * 100.0
        )?;
        if show_stats {
            writeln!(
                out,
                "total files added/removed/modified: {}/{}/{}",
                self.stats.added, self.stats.removed, self.stats.modified
            )?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct WorkByEmail {
    name: &'static BStr,
    email: &'static BStr,
    hours: f32,
    num_commits: u32,
    stats: Stats,
}

/// Statistics for a particular commit.
#[derive(Debug, Default, Copy, Clone)]
struct Stats {
    /// amount of added files
    added: usize,
    /// amount of removed files
    removed: usize,
    /// amount of modified files
    modified: usize,
}

impl Stats {
    fn add(&mut self, other: &Stats) -> &mut Self {
        self.added += other.added;
        self.removed += other.removed;
        self.modified += other.modified;
        self
    }

    fn added(&self, other: &Stats) -> Self {
        let mut a = *self;
        a.add(other);
        a
    }
}
