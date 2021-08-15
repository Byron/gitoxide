use std::{path::Path, time::Instant};

use anyhow::anyhow;
use git_repository::{hash::ObjectId, odb, prelude::*, Repository};

const GITOXIDE_STATIC_CACHE_SIZE: usize = 64;
const GITOXIDE_CACHED_OBJECT_DATA_PER_THREAD_IN_BYTES: usize = 60_000_000;

fn main() -> anyhow::Result<()> {
    let repo_git_dir = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("First argument is the .git directory to work in"))?;
    let repo = git_repository::discover(repo_git_dir)?;

    let hashes = {
        let start = Instant::now();
        let hashes = repo.odb.iter().collect::<Result<Vec<_>, _>>()?;
        let elapsed = start.elapsed();
        println!("gitoxide: {} objects (collected in {:?}", hashes.len(), elapsed);
        hashes
    };

    let objs_per_sec = |elapsed: std::time::Duration| hashes.len() as f32 / elapsed.as_secs_f32();
    let start = Instant::now();
    do_gitoxide_in_parallel(&hashes, &repo, || odb::pack::cache::Never, AccessMode::ObjectExists)?;
    let elapsed = start.elapsed();
    println!(
        "parallel gitoxide : confirmed {} objects exists in {:?} ({:0.0} objects/s)",
        hashes.len(),
        elapsed,
        objs_per_sec(elapsed)
    );

    let start = Instant::now();
    let bytes = do_gitoxide_in_parallel(
        &hashes,
        &repo,
        || odb::pack::cache::lru::MemoryCappedHashmap::new(GITOXIDE_CACHED_OBJECT_DATA_PER_THREAD_IN_BYTES),
        AccessMode::ObjectData,
    )?;
    let elapsed = start.elapsed();
    println!(
        "parallel gitoxide (cache = {:.0}MB): confirmed {} bytes in {:?} ({:0.0} objects/s)",
        GITOXIDE_CACHED_OBJECT_DATA_PER_THREAD_IN_BYTES as f32 / (1024 * 1024) as f32,
        bytes,
        elapsed,
        objs_per_sec(elapsed)
    );

    let start = Instant::now();
    let bytes = do_gitoxide_in_parallel(
        &hashes,
        &repo,
        odb::pack::cache::lru::StaticLinkedList::<GITOXIDE_STATIC_CACHE_SIZE>::default,
        AccessMode::ObjectData,
    )?;
    let elapsed = start.elapsed();
    println!(
        "parallel gitoxide (small static cache): confirmed {} bytes in {:?} ({:0.0} objects/s)",
        bytes,
        elapsed,
        objs_per_sec(elapsed)
    );

    let start = Instant::now();
    let bytes = do_gitoxide_in_parallel(&hashes, &repo, odb::pack::cache::Never::default, AccessMode::ObjectData)?;
    let elapsed = start.elapsed();
    println!(
        "parallel gitoxide (uncached): confirmed {} bytes in {:?} ({:0.0} objects/s)",
        bytes,
        elapsed,
        objs_per_sec(elapsed)
    );

    let start = Instant::now();
    let bytes = do_parallel_git2(hashes.as_slice(), repo.git_dir())?;
    let elapsed = start.elapsed();
    println!(
        "parallel libgit2:  confirmed {} bytes in {:?} ({:0.0} objects/s)",
        bytes,
        elapsed,
        objs_per_sec(elapsed)
    );

    let start = Instant::now();
    let bytes = do_gitoxide(&hashes, &repo, || {
        odb::pack::cache::lru::MemoryCappedHashmap::new(GITOXIDE_CACHED_OBJECT_DATA_PER_THREAD_IN_BYTES)
    })?;
    let elapsed = start.elapsed();
    let objs_per_sec = |elapsed: std::time::Duration| hashes.len() as f32 / elapsed.as_secs_f32();
    println!(
        "gitoxide (cache = {:.0}MB): confirmed {} bytes in {:?} ({:0.0} objects/s)",
        GITOXIDE_CACHED_OBJECT_DATA_PER_THREAD_IN_BYTES as f32 / (1024 * 1024) as f32,
        bytes,
        elapsed,
        objs_per_sec(elapsed)
    );

    let start = Instant::now();
    let bytes = do_gitoxide(
        &hashes,
        &repo,
        odb::pack::cache::lru::StaticLinkedList::<GITOXIDE_STATIC_CACHE_SIZE>::default,
    )?;
    let elapsed = start.elapsed();
    println!(
        "gitoxide (small static cache): confirmed {} bytes in {:?} ({:0.0} objects/s)",
        bytes,
        elapsed,
        objs_per_sec(elapsed)
    );

    let start = Instant::now();
    let bytes = do_gitoxide(&hashes, &repo, odb::pack::cache::Never::default)?;
    let elapsed = start.elapsed();
    println!(
        "gitoxide (uncached): confirmed {} bytes in {:?} ({:0.0} objects/s)",
        bytes,
        elapsed,
        objs_per_sec(elapsed)
    );

    let start = Instant::now();
    let bytes = do_git2(hashes.as_slice(), repo.git_dir())?;
    let elapsed = start.elapsed();

    println!(
        "libgit2:  confirmed {} bytes in {:?} ({:0.0} objects/s)",
        bytes,
        elapsed,
        objs_per_sec(elapsed)
    );

    Ok(())
}

fn do_git2(hashes: &[ObjectId], git_dir: &Path) -> anyhow::Result<u64> {
    git2::opts::strict_hash_verification(false);
    let repo = git2::Repository::open(git_dir)?;
    let odb = repo.odb()?;
    let mut bytes = 0u64;
    for hash in hashes {
        let hash = git2::Oid::from_bytes(hash.as_slice())?;
        let obj = odb.read(hash)?;
        bytes += obj.len() as u64;
    }
    Ok(bytes)
}

fn do_parallel_git2(hashes: &[ObjectId], git_dir: &Path) -> anyhow::Result<u64> {
    use rayon::prelude::*;
    git2::opts::strict_hash_verification(false);
    let bytes = std::sync::atomic::AtomicU64::default();
    hashes.par_iter().try_for_each_init::<_, _, _, anyhow::Result<_>>(
        || git2::Repository::open(git_dir).expect("git directory is valid"),
        |repo, hash| {
            let odb = repo.odb()?;
            let hash = git2::Oid::from_bytes(hash.as_slice())?;
            let obj = odb.read(hash)?;
            bytes.fetch_add(obj.len() as u64, std::sync::atomic::Ordering::Relaxed);
            Ok(())
        },
    )?;

    Ok(bytes.load(std::sync::atomic::Ordering::Acquire))
}

fn do_gitoxide<C>(hashes: &[ObjectId], repo: &Repository, new_cache: impl FnOnce() -> C) -> anyhow::Result<u64>
where
    C: odb::pack::cache::DecodeEntry,
{
    let mut buf = Vec::new();
    let mut bytes = 0u64;
    let mut cache = new_cache();
    for hash in hashes {
        let obj = repo.odb.find_existing(hash, &mut buf, &mut cache)?;
        bytes += obj.data.len() as u64;
    }
    Ok(bytes)
}

enum AccessMode {
    ObjectData,
    ObjectExists,
}

fn do_gitoxide_in_parallel<C>(
    hashes: &[ObjectId],
    repo: &Repository,
    new_cache: impl Fn() -> C + Sync + Send,
    mode: AccessMode,
) -> anyhow::Result<u64>
where
    C: odb::pack::cache::DecodeEntry,
{
    use rayon::prelude::*;
    let bytes = std::sync::atomic::AtomicU64::default();
    hashes.par_iter().try_for_each_init::<_, _, _, anyhow::Result<_>>(
        || (Vec::new(), new_cache()),
        |(buf, cache), hash| {
            match mode {
                AccessMode::ObjectData => {
                    let obj = repo.odb.find_existing(hash, buf, cache)?;
                    bytes.fetch_add(obj.data.len() as u64, std::sync::atomic::Ordering::Relaxed);
                }
                AccessMode::ObjectExists => {
                    assert!(repo.odb.contains(hash), "each traversed object exists");
                }
            }
            Ok(())
        },
    )?;

    Ok(bytes.load(std::sync::atomic::Ordering::Acquire))
}
