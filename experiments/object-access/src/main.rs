//! Currently this experiment fails as `locate(…)` can't actually find objects. My guess is that
//! it can't find objects in packs for some reason because it could find some objects there were
//! probably loose, but failed right away after a `git gc`.
//! Let's see if a unit test can reproduce this too, right now this functionality is entirely untested
//! I think.
use anyhow::anyhow;
use std::{
    io::BufRead,
    path::{Path, PathBuf},
    time::Instant,
};

fn main() -> anyhow::Result<()> {
    if atty::is(atty::Stream::Stdin) {
        anyhow::bail!("Need object hashes on stdin, one per line");
    }
    let mut repo_git_dir = std::env::args()
        .skip(1)
        .next()
        .ok_or_else(|| anyhow!("First argument is the .git directory to work in"))
        .and_then(|p| {
            let p = PathBuf::from(p).canonicalize()?;
            if p.extension().unwrap_or_default() == "git" || p.file_name().unwrap_or_default() == ".git" {
                Ok(p)
            } else {
                Err(anyhow!("Path '{}' needs to be a .git directory", p.display()))
            }
        })?;

    let start = Instant::now();
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    let hashes = stdin.lines().collect::<Result<Vec<String>, _>>()?;
    let elapsed = start.elapsed();
    println!("{} objects (collected in {:?}", hashes.len(), elapsed);

    let start = Instant::now();
    let bytes = do_git2(hashes.as_slice(), &repo_git_dir)?;
    let elapsed = start.elapsed();
    let objs_per_sec = |elapsed: std::time::Duration| hashes.len() as f32 / elapsed.as_secs_f32();

    println!(
        "libgit2:  confirmed {} bytes in {:?} ({:0.0} objects/s)",
        bytes,
        elapsed,
        objs_per_sec(elapsed)
    );

    let start = Instant::now();
    let bytes = do_parallel_git2(hashes.as_slice(), &repo_git_dir)?;
    let elapsed = start.elapsed();
    println!(
        "parallel libgit2:  confirmed {} bytes in {:?} ({:0.0} objects/s)",
        bytes,
        elapsed,
        objs_per_sec(elapsed)
    );

    let start = Instant::now();
    repo_git_dir.push("objects");
    let bytes = do_gitoxide(hashes.as_slice(), &repo_git_dir)?;
    let elapsed = start.elapsed();
    println!(
        "gitoxide: confirmed {} bytes in {:?} ({:0.0} objects/s)",
        bytes,
        elapsed,
        objs_per_sec(elapsed)
    );

    let start = Instant::now();
    let bytes = do_parallel_gitoxide(hashes.as_slice(), &repo_git_dir)?;
    let elapsed = start.elapsed();
    println!(
        "parallel gitoxide: confirmed {} bytes in {:?} ({:0.0} objects/s)",
        bytes,
        elapsed,
        objs_per_sec(elapsed)
    );

    Ok(())
}

fn do_git2(hashes: &[String], git_dir: &Path) -> anyhow::Result<u64> {
    git2::opts::strict_hash_verification(false);
    let repo = git2::Repository::open(git_dir)?;
    let odb = repo.odb()?;
    let mut bytes = 0u64;
    for hash in hashes {
        let hash = git2::Oid::from_str(&hash)?;
        let obj = odb.read(hash)?;
        bytes += obj.len() as u64;
    }
    Ok(bytes)
}

fn do_parallel_git2(hashes: &[String], git_dir: &Path) -> anyhow::Result<u64> {
    use rayon::prelude::*;
    git2::opts::strict_hash_verification(false);
    let bytes = std::sync::atomic::AtomicU64::default();
    hashes.par_iter().try_for_each_init::<_, _, _, anyhow::Result<_>>(
        || git2::Repository::open(git_dir).expect("git directory is valid"),
        |repo, hash| {
            let odb = repo.odb()?;
            let hash = git2::Oid::from_str(&hash)?;
            let obj = odb.read(hash)?;
            bytes.fetch_add(obj.len() as u64, std::sync::atomic::Ordering::Relaxed);
            Ok(())
        },
    )?;

    Ok(bytes.load(std::sync::atomic::Ordering::Acquire))
}

fn do_gitoxide(hashes: &[String], objects_dir: &Path) -> anyhow::Result<u64> {
    let odb = git_odb::compound::Db::at(objects_dir)?;
    let mut buf = Vec::new();
    let mut bytes = 0u64;
    for hash in hashes {
        let hash = git_hash::owned::Digest::from_40_bytes_in_hex(hash.as_bytes())?;
        let obj = odb.locate(hash.to_borrowed(), &mut buf)?.expect("object must exist");
        bytes += obj.size() as u64;
    }
    Ok(bytes)
}

fn do_parallel_gitoxide(hashes: &[String], objects_dir: &Path) -> anyhow::Result<u64> {
    use rayon::prelude::*;
    let odb = git_odb::compound::Db::at(objects_dir)?;
    let bytes = std::sync::atomic::AtomicU64::default();
    hashes
        .par_iter()
        .try_for_each_init::<_, _, _, anyhow::Result<_>>(Vec::new, |mut buf, hash| {
            let hash = git_hash::owned::Digest::from_40_bytes_in_hex(hash.as_bytes())?;
            let obj = odb.locate(hash.to_borrowed(), &mut buf)?.expect("object must exist");
            bytes.fetch_add(obj.size() as u64, std::sync::atomic::Ordering::Relaxed);
            Ok(())
        })?;

    Ok(bytes.load(std::sync::atomic::Ordering::Acquire))
}
