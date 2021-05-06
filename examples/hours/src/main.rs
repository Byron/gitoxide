use anyhow::{anyhow, bail};
use git_hash::{bstr::ByteSlice, ObjectId};
use git_object::bstr::BString;
use git_odb::find::FindExt;
use git_traverse::commit;
use itertools::Itertools;
use rayon::prelude::*;
use std::{path::PathBuf, time::Instant};

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args();
    let repo_git_dir = args
        .nth(1)
        .ok_or_else(|| anyhow!("First argument is the .git directory to work in"))
        .and_then(|p| {
            let p = PathBuf::from(p).join(".git").canonicalize()?;
            if p.extension().unwrap_or_default() == "git"
                || p.file_name().unwrap_or_default() == ".git"
                || p.join("HEAD").is_file()
            {
                Ok(p)
            } else {
                Err(anyhow!(
                    "Path '{}' needs to be a directory containing '.git/'",
                    p.display()
                ))
            }
        })?;
    let commit_id = args
        .next()
        .ok_or_else(|| {
            anyhow!("Second argument is the name of the branch from which to start iteration, like 'main' or 'master'")
        })
        .and_then(|name| {
            ObjectId::from_hex(
                &std::fs::read(repo_git_dir.join("refs").join("heads").join(name))?
                    .as_bstr()
                    .trim(),
            )
            .map_err(Into::into)
        })?;
    let repo_objects_dir = {
        let mut d = repo_git_dir;
        d.push("objects");
        d
    };

    eprintln!("Getting all commits…");
    let start = Instant::now();
    let all_commits = {
        let db = git_odb::linked::Db::at(&repo_objects_dir)?;
        let mut pack_cache = git_odb::pack::cache::Never;
        let mut commits = Vec::<Vec<u8>>::default();
        for commit in commit::Ancestors::new(Some(commit_id), commit::ancestors::State::default(), |oid, buf| {
            db.find_existing(oid, buf, &mut pack_cache).ok().map(|o| {
                commits.push(o.data.to_owned());
                git_object::immutable::CommitIter::from_bytes(o.data)
            })
        }) {
            commit?;
        }
        commits
    };
    let elapsed = start.elapsed();
    eprintln!(
        "Found {} commits and extracted their data in {:?} ({:0.0} commits/s)",
        all_commits.len(),
        elapsed,
        all_commits.len() as f32 / elapsed.as_secs_f32()
    );

    eprintln!("Getting all commit data…");
    let start = Instant::now();
    #[allow(clippy::redundant_closure)]
    let mut all_commits: Vec<CommitInfo> = all_commits
        .into_par_iter()
        .map(|commit_data: Vec<u8>| {
            git_object::immutable::CommitIter::from_bytes(&commit_data)
                .signatures()
                .next()
                .map(|author| git_object::mutable::Signature::from(author))
        })
        .try_fold(
            || Vec::new(),
            |mut out: Vec<_>, item| {
                out.push(item?);
                Some(out)
            },
        )
        .try_reduce(
            || Vec::new(),
            |mut out, vec| {
                out.extend(vec.into_iter());
                Some(out)
            },
        )
        .ok_or_else(|| anyhow!("An error occurred when decoding commits - one commit could not be parsed"))?;
    let elapsed = start.elapsed();
    eprintln!(
        "Obtained {} commits in {:?} ({:0.0} commits/s)",
        all_commits.len(),
        elapsed,
        all_commits.len() as f32 / elapsed.as_secs_f32()
    );
    all_commits.sort_by(|a, b| a.email.cmp(&b.email));
    if all_commits.is_empty() {
        bail!("No commits to process");
    }
    let mut current_email = &all_commits[0].email;
    let mut slice_start = 0;
    let mut results_by_hours = Vec::new();
    for (idx, elm) in all_commits.iter().enumerate() {
        if elm.email != *current_email {
            results_by_hours.push(estimate_hours(&all_commits[slice_start..idx]));
            slice_start = idx;
            current_email = &elm.email;
        }
    }
    if let Some(commits) = all_commits.get(slice_start..) {
        results_by_hours.push(estimate_hours(commits));
    }

    results_by_hours.sort_by(|a, b| a.hours.cmp(&b.hours));
    println!("{:#?}", results_by_hours);
    let (total_hours, total_commits) = results_by_hours
        .iter()
        .map(|e| (e.hours, e.num_commits))
        .reduce(|a, b| (a.0 + b.0, a.1 + b.1))
        .expect("at least one commit at this point");
    println!("total hours: {}, total commits = {}", total_hours, total_commits);
    assert_eq!(total_commits, all_commits.len() as u32, "need to get all commits");
    Ok(())
}

fn estimate_hours(commits: &[CommitInfo]) -> WorkByPerson {
    assert!(!commits.is_empty());
    let hours = commits.iter().rev().tuple_windows().fold(
        0,
        |hours, (cur, next): (&git_object::mutable::Signature, &git_object::mutable::Signature)| {
            const MAX_COMMIT_DIFFERENCE_IN_MINUTES: u32 = 2 * 60;
            const FIRST_COMMIT_ADDITION_IN_MINUTES: u32 = 2 * 60;
            let change_in_minutes =
                (((next.time.time as i32 + next.time.offset) - (cur.time.time as i32 + cur.time.offset)) / 60) as u32;

            if change_in_minutes < MAX_COMMIT_DIFFERENCE_IN_MINUTES {
                hours + (change_in_minutes as f32 / 60_f32).round() as u32
            } else {
                hours + (FIRST_COMMIT_ADDITION_IN_MINUTES / 60)
            }
        },
    );
    let author = &commits[0];
    WorkByPerson {
        name: author.name.to_owned(),
        email: author.email.to_owned(),
        hours,
        num_commits: commits.len() as u32,
    }
}

#[derive(Debug)]
struct WorkByPerson {
    name: BString,
    email: BString,
    hours: u32,
    num_commits: u32,
}

type CommitInfo = git_object::mutable::Signature;
