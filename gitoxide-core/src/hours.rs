use std::collections::BTreeSet;
use std::{
    collections::{hash_map::Entry, HashMap},
    io,
    path::Path,
    time::Instant,
};

use anyhow::{anyhow, bail};
use git_repository as git;
use git_repository::bstr::BStr;
use git_repository::{actor, bstr::ByteSlice, interrupt, objs, prelude::*, progress, Progress};
use itertools::Itertools;

/// Additional configuration for the hours estimation functionality.
pub struct Context<W> {
    /// Ignore github bots which match the `[bot]` search string.
    pub ignore_bots: bool,
    /// Show personally identifiable information before the summary. Includes names and email addresses.
    pub show_pii: bool,
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

    let (all_commits, is_shallow) = {
        let mut progress = progress.add_child("Traverse commit graph");
        let string_heap = &mut string_heap;
        std::thread::scope(
            move |scope| -> anyhow::Result<(Vec<actor::SignatureRef<'static>>, bool)> {
                let start = Instant::now();
                progress.init(None, progress::count("commits"));
                let (tx, rx) = std::sync::mpsc::channel::<Vec<u8>>();
                let mailmap = repo.open_mailmap();

                let handle = scope.spawn(move || -> anyhow::Result<Vec<actor::SignatureRef<'static>>> {
                    let mut out = Vec::new();
                    for commit_data in rx {
                        if let Some(author) = objs::CommitRefIter::from_bytes(&commit_data)
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

                            out.push(actor::SignatureRef {
                                name,
                                email,
                                time: author.time,
                            });
                        }
                    }
                    out.shrink_to_fit();
                    out.sort_by(|a, b| {
                        a.email.cmp(&b.email).then(
                            a.time
                                .seconds_since_unix_epoch
                                .cmp(&b.time.seconds_since_unix_epoch)
                                .reverse(),
                        )
                    });
                    Ok(out)
                });

                let commit_iter = interrupt::Iter::new(
                    commit_id.ancestors(|oid, buf| {
                        progress.inc();
                        repo.objects.find(oid, buf).map(|o| {
                            tx.send(o.data.to_owned()).ok();
                            objs::CommitRefIter::from_bytes(o.data)
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
                progress.show_throughput(start);
                Ok((handle.join().expect("no panic")?, is_shallow))
            },
        )?
    };

    if all_commits.is_empty() {
        bail!("No commits to process");
    }

    let start = Instant::now();
    let mut current_email = &all_commits[0].email;
    let mut slice_start = 0;
    let mut results_by_hours = Vec::new();
    let mut ignored_bot_commits = 0_u32;
    for (idx, elm) in all_commits.iter().enumerate() {
        if elm.email != *current_email {
            let estimate = estimate_hours(&all_commits[slice_start..idx]);
            slice_start = idx;
            current_email = &elm.email;
            if ignore_bots && estimate.name.contains_str(b"[bot]") {
                ignored_bot_commits += estimate.num_commits;
                continue;
            }
            results_by_hours.push(estimate);
        }
    }
    if let Some(commits) = all_commits.get(slice_start..) {
        results_by_hours.push(estimate_hours(commits));
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
        all_commits.len(),
        elapsed,
        all_commits.len() as f32 / elapsed.as_secs_f32()
    ));

    let num_unique_authors = results_by_hours.len();
    let (total_hours, total_commits) = results_by_hours
        .iter()
        .map(|e| (e.hours, e.num_commits))
        .reduce(|a, b| (a.0 + b.0, a.1 + b.1))
        .expect("at least one commit at this point");
    if show_pii {
        results_by_hours.sort_by(|a, b| a.hours.partial_cmp(&b.hours).unwrap_or(std::cmp::Ordering::Equal));
        for entry in results_by_hours.iter() {
            entry.write_to(total_hours, &mut out)?;
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
        all_commits.len() as u32 - ignored_bot_commits,
        "need to get all commits"
    );
    Ok(())
}

const MINUTES_PER_HOUR: f32 = 60.0;
const HOURS_PER_WORKDAY: f32 = 8.0;

fn estimate_hours(commits: &[actor::SignatureRef<'static>]) -> WorkByEmail {
    assert!(!commits.is_empty());
    const MAX_COMMIT_DIFFERENCE_IN_MINUTES: f32 = 2.0 * MINUTES_PER_HOUR;
    const FIRST_COMMIT_ADDITION_IN_MINUTES: f32 = 2.0 * MINUTES_PER_HOUR;

    let hours = FIRST_COMMIT_ADDITION_IN_MINUTES / 60.0
        + commits.iter().rev().tuple_windows().fold(
            0_f32,
            |hours, (cur, next): (&actor::SignatureRef<'_>, &actor::SignatureRef<'_>)| {
                let change_in_minutes =
                    (next.time.seconds_since_unix_epoch - cur.time.seconds_since_unix_epoch) as f32 / MINUTES_PER_HOUR;
                if change_in_minutes < MAX_COMMIT_DIFFERENCE_IN_MINUTES {
                    hours + change_in_minutes as f32 / MINUTES_PER_HOUR
                } else {
                    hours + (FIRST_COMMIT_ADDITION_IN_MINUTES / MINUTES_PER_HOUR)
                }
            },
        );
    let author = &commits[0];
    WorkByEmail {
        name: author.name,
        email: author.email,
        hours,
        num_commits: commits.len() as u32,
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
    }
}

impl<'a> From<&'a WorkByEmail> for WorkByPerson {
    fn from(w: &'a WorkByEmail) -> Self {
        WorkByPerson {
            name: vec![w.name],
            email: vec![w.email],
            hours: w.hours,
            num_commits: w.num_commits,
        }
    }
}

impl WorkByPerson {
    fn write_to(&self, total_hours: f32, mut out: impl std::io::Write) -> std::io::Result<()> {
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
        )
    }
}

#[derive(Debug)]
struct WorkByEmail {
    name: &'static BStr,
    email: &'static BStr,
    hours: f32,
    num_commits: u32,
}
