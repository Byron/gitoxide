use std::collections::{hash_map::Entry, HashMap};

use gix::bstr::BStr;
use itertools::Itertools;

use crate::hours::{FileStats, LineStats, WorkByEmail, WorkByPerson};

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
                hours + change_in_minutes as f32 / MINUTES_PER_HOUR
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
