use anyhow::anyhow;
use git_diff::visit::record::{Action, Change};
use git_hash::{
    bstr::{BStr, ByteSlice},
    oid, ObjectId,
};
use git_object::immutable;
use git_odb::Locate;
use rayon::prelude::*;
use std::{
    collections::{btree_map::Entry, BTreeMap},
    path::PathBuf,
    time::{Duration, Instant},
};

const GITOXIDE_STATIC_CACHE_SIZE: usize = 64;
const GITOXIDE_CACHED_OBJECT_DATA_PER_THREAD_IN_BYTES: usize = 60_000_000;

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args();
    let repo_git_dir = args
        .nth(1)
        .ok_or_else(|| anyhow!("First argument is the .git directory to work in"))
        .and_then(|p| {
            let p = PathBuf::from(p).canonicalize()?;
            if p.extension().unwrap_or_default() == "git"
                || p.file_name().unwrap_or_default() == ".git"
                || p.join("HEAD").is_file()
            {
                Ok(p)
            } else {
                Err(anyhow!("Path '{}' needs to be a .git directory", p.display()))
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

    let start = Instant::now();
    let count = do_gitoxide_graph_traversal(commit_id, &db, || {
        git_odb::pack::cache::lru::MemoryCappedHashmap::new(GITOXIDE_CACHED_OBJECT_DATA_PER_THREAD_IN_BYTES)
    })?;
    let elapsed = start.elapsed();
    let objs_per_sec = |elapsed: Duration| count as f32 / elapsed.as_secs_f32();
    println!(
        "gitoxide (cache = {:.0}MB): confirmed {} commits in {:?} ({:0.0} commits/s)",
        GITOXIDE_CACHED_OBJECT_DATA_PER_THREAD_IN_BYTES as f32 / (1024 * 1024) as f32,
        count,
        elapsed,
        objs_per_sec(elapsed)
    );

    let start = Instant::now();
    let count = do_gitoxide_graph_traversal(
        commit_id,
        &db,
        git_odb::pack::cache::lru::StaticLinkedList::<GITOXIDE_STATIC_CACHE_SIZE>::default,
    )?;
    let elapsed = start.elapsed();
    let objs_per_sec = |elapsed: Duration| count as f32 / elapsed.as_secs_f32();
    println!(
        "gitoxide (static cache = {:.0} entries): confirmed {} commits in {:?} ({:0.0} commits/s)",
        GITOXIDE_STATIC_CACHE_SIZE,
        count,
        elapsed,
        objs_per_sec(elapsed)
    );

    let start = Instant::now();
    let count = do_gitoxide_graph_traversal(commit_id, &db, || git_odb::pack::cache::Never)?;
    let elapsed = start.elapsed();
    let objs_per_sec = |elapsed: Duration| count as f32 / elapsed.as_secs_f32();
    println!(
        "gitoxide (uncached): confirmed {} commits in {:?} ({:0.0} commits/s)",
        count,
        elapsed,
        objs_per_sec(elapsed)
    );

    let repo = git2::Repository::open(&repo_git_dir)?;
    let start = Instant::now();
    let count = do_libgit2_graph_traversal(commit_id, &repo)?;
    let elapsed = start.elapsed();
    let objs_per_sec = |elapsed: Duration| count as f32 / elapsed.as_secs_f32();
    println!(
        "libgit2: confirmed {} commits in {:?} ({:0.0} commits/s)",
        count,
        elapsed,
        objs_per_sec(elapsed)
    );

    let start = Instant::now();
    let all_commits = git_odb::traverse::Ancestors::new(&db, Some(commit_id), &mut git_odb::pack::cache::Never)
        .collect::<Result<Vec<_>, _>>()?;
    let num_diffs = all_commits.len();
    let elapsed = start.elapsed();
    println!(
        "gitoxide (uncached): collect all {} commits in {:?} ({:0.0} commits/s)",
        all_commits.len(),
        elapsed,
        all_commits.len() as f32 / elapsed.as_secs_f32()
    );

    let start = Instant::now();
    fn find_with_obj_cache<'b>(
        oid: &oid,
        buf: &'b mut Vec<u8>,
        obj_cache: &mut BTreeMap<ObjectId, (git_object::Kind, Vec<u8>)>,
        db: &git_odb::linked::Db,
        pack_cache: &mut impl git_odb::pack::cache::DecodeEntry,
    ) -> Option<git_odb::data::Object<'b>> {
        match obj_cache.entry(oid.to_owned()) {
            Entry::Vacant(e) => {
                let obj = db.locate(oid, buf, pack_cache).ok().flatten();
                if let Some(ref obj) = obj {
                    e.insert((obj.kind, obj.data.to_owned()));
                }
                obj
            }
            Entry::Occupied(e) => {
                let (k, d) = e.get();
                buf.resize(d.len(), 0);
                buf.copy_from_slice(d);
                Some(git_odb::data::Object::new(*k, buf))
            }
        }
    }
    let num_deltas = do_gitoxide_tree_diff(
        &all_commits,
        || {
            let mut pack_cache =
                git_odb::pack::cache::lru::MemoryCappedHashmap::new(GITOXIDE_CACHED_OBJECT_DATA_PER_THREAD_IN_BYTES);
            let db = &db;
            let mut obj_cache = BTreeMap::new();
            move |oid, buf: &mut Vec<u8>| find_with_obj_cache(oid, buf, &mut obj_cache, db, &mut pack_cache)
        },
        Computation::SingleThreaded,
    )?;
    let elapsed = start.elapsed();
    println!(
        "gitoxide-deltas (cache = memory obj map -> {:.0}MB pack): collect {} tree deltas of {} trees-diffs in {:?} ({:0.0} deltas/s, {:0.0} tree-diffs/s)",
        GITOXIDE_CACHED_OBJECT_DATA_PER_THREAD_IN_BYTES as f32 / (1024 * 1024) as f32,
        num_deltas,
        num_diffs,
        elapsed,
        num_deltas as f32 / elapsed.as_secs_f32(),
        num_diffs as f32 / elapsed.as_secs_f32()
    );

    let start = Instant::now();
    let num_deltas = do_gitoxide_tree_diff(
        &all_commits,
        || {
            let mut pack_cache =
                git_odb::pack::cache::lru::MemoryCappedHashmap::new(GITOXIDE_CACHED_OBJECT_DATA_PER_THREAD_IN_BYTES);
            let db = &db;
            let mut obj_cache = BTreeMap::new();
            move |oid, buf: &mut Vec<u8>| find_with_obj_cache(oid, buf, &mut obj_cache, db, &mut pack_cache)
        },
        Computation::MultiThreaded,
    )?;
    let elapsed = start.elapsed();
    println!(
        "gitoxide-deltas PARALLEL (cache = memory obj map -> {:.0}MB pack): collect {} tree deltas of {} trees-diffs in {:?} ({:0.0} deltas/s, {:0.0} tree-diffs/s)",
        GITOXIDE_CACHED_OBJECT_DATA_PER_THREAD_IN_BYTES as f32 / (1024 * 1024) as f32,
        num_deltas,
        num_diffs,
        elapsed,
        num_deltas as f32 / elapsed.as_secs_f32(),
        num_diffs as f32 / elapsed.as_secs_f32()
    );

    let start = Instant::now();
    let num_deltas = do_gitoxide_tree_diff(
        &all_commits,
        || {
            let mut pack_cache =
                git_odb::pack::cache::lru::MemoryCappedHashmap::new(GITOXIDE_CACHED_OBJECT_DATA_PER_THREAD_IN_BYTES);
            let db = &db;
            struct ObjectInfo {
                kind: git_object::Kind,
                data: Vec<u8>,
            }
            impl memory_lru::ResidentSize for ObjectInfo {
                fn resident_size(&self) -> usize {
                    self.data.len()
                }
            }
            let mut obj_cache = memory_lru::MemoryLruCache::new(GITOXIDE_CACHED_OBJECT_DATA_PER_THREAD_IN_BYTES);
            move |oid, buf: &mut Vec<u8>| {
                let oid = oid.to_owned();
                match obj_cache.get(&oid) {
                    Some(ObjectInfo { kind, data }) => {
                        buf.resize(data.len(), 0);
                        buf.copy_from_slice(data);
                        Some(git_odb::data::Object::new(*kind, buf))
                    }
                    None => {
                        let obj = db.locate(oid, buf, &mut pack_cache).ok().flatten();
                        if let Some(ref obj) = obj {
                            obj_cache.insert(
                                oid,
                                ObjectInfo {
                                    kind: obj.kind,
                                    data: obj.data.to_owned(),
                                },
                            );
                        }
                        obj
                    }
                }
            }
        },
        Computation::MultiThreaded,
    )?;
    let elapsed = start.elapsed();
    println!(
        "gitoxide-deltas PARALLEL (cache = memory-lrup -> {:.0}MB pack): collect {} tree deltas of {} trees-diffs in {:?} ({:0.0} deltas/s, {:0.0} tree-diffs/s)",
        GITOXIDE_CACHED_OBJECT_DATA_PER_THREAD_IN_BYTES as f32 / (1024 * 1024) as f32,
        num_deltas,
        num_diffs,
        elapsed,
        num_deltas as f32 / elapsed.as_secs_f32(),
        num_diffs as f32 / elapsed.as_secs_f32()
    );

    Ok(())
}

enum Computation {
    SingleThreaded,
    MultiThreaded,
}

fn do_gitoxide_tree_diff<C, L>(commits: &[ObjectId], make_locate: C, mode: Computation) -> anyhow::Result<usize>
where
    C: Fn() -> L + Sync,
    L: for<'b> FnMut(&oid, &'b mut Vec<u8>) -> Option<git_odb::data::Object<'b>>,
{
    let changes: usize = match mode {
        Computation::MultiThreaded => {
            let changes = std::sync::atomic::AtomicUsize::new(0);
            commits.par_windows(2).try_for_each_init::<_, _, _, anyhow::Result<_>>(
                || {
                    (
                        git_diff::visit::State::<()>::default(),
                        Vec::<u8>::new(),
                        Vec::<u8>::new(),
                        make_locate(),
                    )
                },
                |(state, buf1, buf2, find), pair| {
                    let (ca, cb) = (pair[0], pair[1]);
                    let (ta, tb) = (
                        tree_iter_by_commit(&ca, buf1, &mut *find),
                        tree_iter_by_commit(&cb, buf2, &mut *find),
                    );
                    let mut count = Count::default();
                    git_diff::visit::Changes::from(ta).needed_to_obtain(
                        tb,
                        state,
                        |id, buf| find_tree_iter(id, buf, &mut *find),
                        &mut count,
                    )?;
                    changes.fetch_add(count.0, std::sync::atomic::Ordering::Relaxed);
                    Ok(())
                },
            )?;
            changes.load(std::sync::atomic::Ordering::Acquire)
        }
        Computation::SingleThreaded => {
            let mut state = git_diff::visit::State::default();
            let mut find = make_locate();
            let mut buf: Vec<u8> = Vec::new();
            let mut buf2: Vec<u8> = Vec::new();
            let mut changes = 0;

            for pair in commits.windows(2) {
                let (ca, cb) = (pair[0], pair[1]);
                let (ta, tb) = (
                    tree_iter_by_commit(&ca, &mut buf, &mut find),
                    tree_iter_by_commit(&cb, &mut buf2, &mut find),
                );
                let mut count = Count::default();
                git_diff::visit::Changes::from(ta).needed_to_obtain(
                    tb,
                    &mut state,
                    |id, buf| find_tree_iter(id, buf, &mut find),
                    &mut count,
                )?;
                changes += count.0;
            }
            changes
        }
    };

    fn find_tree_iter<'b, L>(id: &oid, buf: &'b mut Vec<u8>, mut find: L) -> Option<immutable::TreeIter<'b>>
    where
        L: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Option<git_odb::data::Object<'a>>,
    {
        find(id, buf).and_then(|o| o.into_tree_iter())
    }

    fn tree_iter_by_commit<'b, L>(id: &oid, buf: &'b mut Vec<u8>, mut find: L) -> immutable::TreeIter<'b>
    where
        L: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Option<git_odb::data::Object<'a>>,
    {
        let tid = find(id, buf)
            .expect("commit present")
            .into_commit_iter()
            .expect("a commit")
            .tree_id()
            .expect("tree id present and decodable");
        find_tree_iter(&tid, buf, find).expect("tree available")
    }

    #[derive(Default)]
    struct Count(usize);

    impl git_diff::visit::Record for Count {
        type PathId = ();

        fn set_current_path(&mut self, _path: Self::PathId) {}

        fn push_tracked_path_component(&mut self, _component: &BStr) -> Self::PathId {}

        fn push_path_component(&mut self, _component: &BStr) {}

        fn pop_path_component(&mut self) {}

        fn record(&mut self, _change: Change) -> Action {
            self.0 += 1;
            Action::Continue
        }
    }
    Ok(changes)
}

fn do_gitoxide_graph_traversal<C>(
    tip: ObjectId,
    db: &git_odb::linked::Db,
    new_cache: impl FnOnce() -> C,
) -> anyhow::Result<usize>
where
    C: git_odb::pack::cache::DecodeEntry,
{
    let mut cache = new_cache();
    let ancestors = git_odb::traverse::Ancestors::new(db, Some(tip), &mut cache);
    let mut commits = 0;
    for commit_id in ancestors {
        let _ = commit_id?;
        commits += 1;
    }
    Ok(commits)
}

fn do_libgit2_graph_traversal(tip: ObjectId, db: &git2::Repository) -> anyhow::Result<usize> {
    let mut commits = 0;
    let mut walk = db.revwalk()?;
    walk.push(git2::Oid::from_bytes(tip.as_bytes())?)?;

    for commit_id in walk {
        let _ = commit_id?;
        commits += 1;
    }
    Ok(commits)
}
