use std::{path::Path, sync::Arc, time::Instant};

use anyhow::anyhow;
use git_repository::{hash::ObjectId, odb, threading::OwnShared, ThreadSafeRepository};

use crate::odb::Cache;

const GITOXIDE_STATIC_CACHE_SIZE: usize = 64;
const GITOXIDE_CACHED_OBJECT_DATA_PER_THREAD_IN_BYTES: usize = 60_000_000;

fn main() -> anyhow::Result<()> {
    let repo_git_dir = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("First argument is the .git directory to work in"))?;
    let repo = git_repository::discover(&repo_git_dir)?.into_sync();

    let hashes = {
        let start = Instant::now();
        let repo = git_repository::discover(repo_git_dir)?;
        let hashes = repo.objects.iter()?.collect::<Result<Vec<_>, _>>()?;
        let elapsed = start.elapsed();
        println!("gitoxide: {} objects (collected in {:?}", hashes.len(), elapsed);
        hashes
    };

    let objs_per_sec = |elapsed: std::time::Duration| hashes.len() as f32 / elapsed.as_secs_f32();

    let start = Instant::now();
    do_gitoxide_in_parallel_sync(&hashes, &repo, || odb::pack::cache::Never, AccessMode::ObjectExists)?;
    let elapsed = start.elapsed();
    println!(
        "parallel gitoxide : confirmed {} objects exists in {:?} ({:0.0} objects/s)",
        hashes.len(),
        elapsed,
        objs_per_sec(elapsed)
    );

    let start = Instant::now();
    let (object_store, _) = do_new_gitoxide_store_in_parallel(
        &hashes,
        repo.objects_dir(),
        || odb::pack::cache::Never,
        AccessMode::ObjectExists,
        None,
    )?;
    let elapsed = start.elapsed();
    println!(
        "parallel gitoxide (new store): confirmed {} objects exists in {:?} ({:0.0} objects/s)",
        hashes.len(),
        elapsed,
        objs_per_sec(elapsed)
    );

    let start = Instant::now();
    do_new_gitoxide_store_in_parallel(
        &hashes,
        repo.objects_dir(),
        || odb::pack::cache::Never,
        AccessMode::ObjectExists,
        Some(object_store),
    )?;
    let elapsed = start.elapsed();
    println!(
        "parallel gitoxide (new store, warm): confirmed {} objects exists in {:?} ({:0.0} objects/s)",
        hashes.len(),
        elapsed,
        objs_per_sec(elapsed)
    );

    let start = Instant::now();
    do_gitoxide_in_parallel_through_arc(
        &hashes,
        repo.objects_dir(),
        odb::pack::cache::Never::default,
        AccessMode::ObjectExists,
    )?;
    let elapsed = start.elapsed();
    println!(
        "parallel gitoxide (Arc): confirmed {} objects exists in {:?} ({:0.0} objects/s)",
        hashes.len(),
        elapsed,
        objs_per_sec(elapsed)
    );

    let start = Instant::now();
    do_parallel_git2(hashes.as_slice(), repo.git_dir(), AccessMode::ObjectExists)?;
    let elapsed = start.elapsed();
    println!(
        "parallel libgit2: confirmed {} exist in {:?} ({:0.0} objects/s)",
        hashes.len(),
        elapsed,
        objs_per_sec(elapsed)
    );

    let start = Instant::now();
    do_git2(hashes.as_slice(), repo.git_dir(), AccessMode::ObjectExists)?;
    let elapsed = start.elapsed();
    println!(
        "libgit2:  confirmed {} exist in {:?} ({:0.0} objects/s)",
        hashes.len(),
        elapsed,
        objs_per_sec(elapsed)
    );

    let start = Instant::now();
    let (object_store, bytes) = do_new_gitoxide_store_in_parallel(
        &hashes,
        repo.objects_dir(),
        || odb::pack::cache::Never,
        AccessMode::ObjectData,
        None,
    )?;
    let elapsed = start.elapsed();
    println!(
        "parallel gitoxide (new store, uncached): confirmed {} bytes exists in {:?} ({:0.0} objects/s)",
        bytes,
        elapsed,
        objs_per_sec(elapsed)
    );

    let start = Instant::now();
    let (object_store, bytes) = do_new_gitoxide_store_in_parallel(
        &hashes,
        repo.objects_dir(),
        || odb::pack::cache::Never,
        AccessMode::ObjectData,
        Some(object_store),
    )?;
    let elapsed = start.elapsed();
    println!(
        "parallel gitoxide (new store, uncached, warm): confirmed {} bytes exists in {:?} ({:0.0} objects/s)",
        bytes,
        elapsed,
        objs_per_sec(elapsed)
    );

    let start = Instant::now();
    let (_, bytes) = do_new_gitoxide_store_in_parallel(
        &hashes,
        repo.objects_dir(),
        || odb::pack::cache::lru::MemoryCappedHashmap::new(GITOXIDE_CACHED_OBJECT_DATA_PER_THREAD_IN_BYTES),
        AccessMode::ObjectData,
        Some(object_store),
    )?;
    let elapsed = start.elapsed();
    println!(
        "parallel gitoxide (new store, cache = {:.0}MB), warm): confirmed {} bytes exists in {:?} ({:0.0} objects/s)",
        GITOXIDE_CACHED_OBJECT_DATA_PER_THREAD_IN_BYTES as f32 / (1024 * 1024) as f32,
        bytes,
        elapsed,
        objs_per_sec(elapsed)
    );

    let start = Instant::now();
    let bytes = do_gitoxide_in_parallel_sync(&hashes, &repo, odb::pack::cache::Never::default, AccessMode::ObjectData)?;
    let elapsed = start.elapsed();
    println!(
        "parallel gitoxide (uncached): confirmed {} bytes in {:?} ({:0.0} objects/s)",
        bytes,
        elapsed,
        objs_per_sec(elapsed)
    );

    let start = Instant::now();
    let bytes = do_gitoxide_in_parallel_through_arc(
        &hashes,
        repo.objects_dir(),
        odb::pack::cache::Never::default,
        AccessMode::ObjectData,
    )?;
    let elapsed = start.elapsed();
    println!(
        "parallel gitoxide (uncached, Arc): confirmed {} bytes in {:?} ({:0.0} objects/s)",
        bytes,
        elapsed,
        objs_per_sec(elapsed)
    );

    let start = Instant::now();
    let bytes = do_gitoxide_in_parallel_sync(
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
    let bytes = do_gitoxide_in_parallel_sync(
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
    let bytes = do_git2(hashes.as_slice(), repo.git_dir(), AccessMode::ObjectData)?;
    let elapsed = start.elapsed();
    println!(
        "libgit2:  confirmed {} bytes in {:?} ({:0.0} objects/s)",
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
    let bytes = do_parallel_git2(hashes.as_slice(), repo.git_dir(), AccessMode::ObjectData)?;
    let elapsed = start.elapsed();
    println!(
        "parallel libgit2:  confirmed {} bytes in {:?} ({:0.0} objects/s)",
        bytes,
        elapsed,
        objs_per_sec(elapsed)
    );

    Ok(())
}

fn do_git2(hashes: &[ObjectId], git_dir: &Path, mode: AccessMode) -> anyhow::Result<u64> {
    git2::opts::strict_hash_verification(false);
    let repo = git2::Repository::open(git_dir)?;
    let odb = repo.odb()?;
    let mut bytes = 0u64;
    for hash in hashes {
        let hash = git2::Oid::from_bytes(hash.as_slice())?;
        match mode {
            AccessMode::ObjectData => {
                let obj = odb.read(hash)?;
                bytes += obj.len() as u64;
            }
            AccessMode::ObjectExists => {
                assert!(odb.exists(hash));
            }
        }
    }
    Ok(bytes)
}

fn do_parallel_git2(hashes: &[ObjectId], git_dir: &Path, mode: AccessMode) -> anyhow::Result<u64> {
    use rayon::prelude::*;
    git2::opts::strict_hash_verification(false);
    let bytes = std::sync::atomic::AtomicU64::default();
    hashes.par_iter().try_for_each_init::<_, _, _, anyhow::Result<_>>(
        || git2::Repository::open(git_dir).expect("git directory is valid"),
        |repo, hash| {
            let odb = repo.odb()?;
            let hash = git2::Oid::from_bytes(hash.as_slice())?;
            match mode {
                AccessMode::ObjectData => {
                    let obj = odb.read(hash)?;
                    bytes.fetch_add(obj.len() as u64, std::sync::atomic::Ordering::Relaxed);
                }
                AccessMode::ObjectExists => {
                    assert!(odb.exists(hash));
                }
            }
            Ok(())
        },
    )?;

    Ok(bytes.load(std::sync::atomic::Ordering::Acquire))
}

fn do_gitoxide<C>(
    hashes: &[ObjectId],
    repo: &ThreadSafeRepository,
    new_cache: impl Fn() -> C + Send + Sync + 'static,
) -> anyhow::Result<u64>
where
    C: odb::pack::cache::DecodeEntry + Send + 'static,
{
    use git_repository::prelude::FindExt;
    let mut buf = Vec::new();
    let mut bytes = 0u64;
    let handle = repo.objects.to_cache().with_pack_cache(move || Box::new(new_cache()));
    for hash in hashes {
        let obj = handle.find(hash, &mut buf)?;
        bytes += obj.data.len() as u64;
    }
    Ok(bytes)
}

enum AccessMode {
    ObjectData,
    ObjectExists,
}

fn do_gitoxide_in_parallel_sync<C>(
    hashes: &[ObjectId],
    repo: &ThreadSafeRepository,
    new_cache: impl Fn() -> C + Send + Clone + Sync + 'static,
    mode: AccessMode,
) -> anyhow::Result<u64>
where
    C: odb::pack::cache::DecodeEntry + Send + 'static,
{
    use git_repository::prelude::FindExt;
    use rayon::prelude::*;
    let bytes = std::sync::atomic::AtomicU64::default();
    hashes.par_iter().try_for_each_init::<_, _, _, anyhow::Result<_>>(
        move || {
            (
                Vec::new(),
                repo.objects.to_cache().with_pack_cache({
                    let new_cache = new_cache.clone();
                    move || Box::new(new_cache())
                }),
            )
        },
        |(buf, objects), hash| {
            match mode {
                AccessMode::ObjectData => {
                    let obj = objects.find(hash, buf)?;
                    bytes.fetch_add(obj.data.len() as u64, std::sync::atomic::Ordering::Relaxed);
                }
                AccessMode::ObjectExists => {
                    use git_repository::prelude::Find;
                    assert!(objects.contains(hash), "each traversed object exists");
                }
            }
            Ok(())
        },
    )?;

    Ok(bytes.load(std::sync::atomic::Ordering::Acquire))
}

fn do_new_gitoxide_store_in_parallel<C>(
    hashes: &[ObjectId],
    objects_dir: &Path,
    new_cache: impl Fn() -> C + Send + Sync + 'static,
    mode: AccessMode,
    store: Option<OwnShared<odb::Store>>,
) -> anyhow::Result<(std::sync::Arc<odb::Store>, u64)>
where
    C: odb::pack::cache::DecodeEntry + Send + 'static,
{
    use git_repository::prelude::FindExt;
    let bytes = std::sync::atomic::AtomicU64::default();
    let slots = std::env::args()
        .nth(2)
        .and_then(|num| num.parse().ok().map(odb::store::init::Slots::Given))
        .unwrap_or_default();

    let store = match store {
        Some(store) => store,
        None => OwnShared::new(odb::Store::at_opts(
            objects_dir,
            git_repository::odb::store::init::Options {
                slots,
                ..Default::default()
            },
        )?),
    };
    let handle = Cache::from(store.to_handle()).with_pack_cache(move || Box::new(new_cache()));

    git_repository::parallel::in_parallel(
        hashes.chunks(1000),
        None,
        move |_| (Vec::new(), handle.clone()),
        |hashes, (buf, cache)| {
            for hash in hashes {
                match mode {
                    AccessMode::ObjectData => {
                        let obj = cache.find(hash, buf)?;
                        bytes.fetch_add(obj.data.len() as u64, std::sync::atomic::Ordering::Relaxed);
                    }
                    AccessMode::ObjectExists => {
                        use git_repository::prelude::Find;
                        assert!(cache.contains(hash), "each traversed object exists");
                    }
                }
            }
            Ok(())
        },
        git_repository::parallel::reduce::IdentityWithResult::<(), anyhow::Error>::default(),
    )?;
    Ok((store, bytes.load(std::sync::atomic::Ordering::Acquire)))
}

fn do_gitoxide_in_parallel_through_arc<C>(
    hashes: &[ObjectId],
    objects_dir: &Path,
    new_cache: impl Fn() -> C + Send + Sync + Clone + 'static,
    mode: AccessMode,
) -> anyhow::Result<u64>
where
    C: odb::pack::cache::DecodeEntry + Send + 'static,
{
    use git_repository::prelude::FindExt;
    let bytes = std::sync::atomic::AtomicU64::default();
    #[allow(deprecated)]
    let odb = Arc::new(odb::linked::Store::at(objects_dir)?);

    git_repository::parallel::in_parallel(
        hashes.chunks(1000),
        None,
        move |_| {
            (
                Vec::new(),
                odb.to_cache_arc().with_pack_cache({
                    let new_cache = new_cache.clone();
                    move || Box::new(new_cache())
                }),
            )
        },
        |hashes, (buf, odb)| {
            for hash in hashes {
                match mode {
                    AccessMode::ObjectData => {
                        let obj = odb.find(hash, buf)?;
                        bytes.fetch_add(obj.data.len() as u64, std::sync::atomic::Ordering::Relaxed);
                    }
                    AccessMode::ObjectExists => {
                        use git_repository::prelude::Find;
                        assert!(odb.contains(hash), "each traversed object exists");
                    }
                }
            }
            Ok(())
        },
        git_repository::parallel::reduce::IdentityWithResult::<(), anyhow::Error>::default(),
    )?;
    Ok(bytes.load(std::sync::atomic::Ordering::Acquire))
}
