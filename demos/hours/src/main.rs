use anyhow::anyhow;
use git_hash::{bstr::ByteSlice, ObjectId};
use git_object::bstr::BString;
use git_odb::find::FindExt;
use git_traverse::commit;
use rayon::prelude::*;
use std::{path::PathBuf, time::Instant};

const GITOXIDE_STATIC_CACHE_SIZE: usize = 64;

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
        let mut d = repo_git_dir.clone();
        d.push("objects");
        d
    };
    let db = git_odb::linked::Db::at(&repo_objects_dir)?;

    eprintln!("Getting all commits…");
    let start = Instant::now();
    let mut pack_cache = git_odb::pack::cache::lru::StaticLinkedList::<GITOXIDE_STATIC_CACHE_SIZE>::default();
    let all_commits = commit::Ancestors::new(Some(commit_id), commit::ancestors::State::default(), |oid, buf| {
        db.find_existing_commit_iter(oid, buf, &mut pack_cache).ok()
    })
    .collect::<Result<Vec<_>, _>>()?;
    let elapsed = start.elapsed();
    println!(
        "Found {} commits in {:?} ({:0.0} commits/s)",
        all_commits.len(),
        elapsed,
        all_commits.len() as f32 / elapsed.as_secs_f32()
    );

    eprintln!("Getting all commit data…");
    let start = Instant::now();
    let all_commits = all_commits
        .into_par_iter()
        .try_fold::<_, anyhow::Result<_>, _, _>(
            || {
                (
                    Vec::new(),
                    git_odb::pack::cache::lru::MemoryCappedHashmap::new(64_000_000),
                )
            },
            |(mut commits, mut cache): (Vec<CommitInfo>, git_odb::pack::cache::lru::MemoryCappedHashmap), commit_id| {
                let mut buf_reuse_me = Vec::new();
                let commit = db.find_existing_commit(commit_id, &mut buf_reuse_me, &mut cache)?;
                commits.push(CommitInfo::from(commit));
                Ok((commits, cache))
            },
        )
        .map(|res| res.map(|(commits, _)| commits))
        .try_reduce(
            || Vec::new(),
            |mut a: Vec<CommitInfo>, b: Vec<CommitInfo>| {
                a.extend(b.into_iter());
                Ok(a)
            },
        )?;
    let elapsed = start.elapsed();
    println!(
        "Obtained {} commits in {:?} ({:0.0} commits/s)",
        all_commits.len(),
        elapsed,
        all_commits.len() as f32 / elapsed.as_secs_f32()
    );

    Ok(())
}

struct CommitInfo {
    email: BString,
    name: BString,
    hours: u32,
    num_commits: u32,
}

impl From<git_object::immutable::Commit<'_>> for CommitInfo {
    fn from(c: git_object::immutable::Commit<'_>) -> Self {
        CommitInfo {
            email: c.author.email.to_owned(),
            name: c.author.name.to_owned(),
            hours: 0,
            num_commits: 0,
        }
    }
}
