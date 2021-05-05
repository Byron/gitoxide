use anyhow::anyhow;
use dashmap::DashSet;
use git_hash::{bstr::BStr, bstr::ByteSlice, ObjectId};
use git_object::immutable::tree::Entry;
use git_odb::{find::FindExt, Find};
use git_traverse::{commit, tree, tree::visit::Action};
use std::{
    collections::HashSet,
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
    let all_commits = commit::Ancestors::new(Some(commit_id), commit::ancestors::State::default(), |oid, buf| {
        db.find(oid, buf, &mut git_odb::pack::cache::Never)
            .ok()
            .flatten()
            .and_then(|o| o.into_commit_iter())
    })
    .collect::<Result<Vec<_>, _>>()?;
    let elapsed = start.elapsed();
    println!(
        "gitoxide (uncached): collect all {} commits in {:?} ({:0.0} commits/s)",
        all_commits.len(),
        elapsed,
        all_commits.len() as f32 / elapsed.as_secs_f32()
    );

    let start = Instant::now();
    let (unique, entries) = do_gitoxide_tree_dag_traversal(
        &all_commits,
        &db,
        git_odb::pack::cache::lru::StaticLinkedList::<64>::default,
        Computation::MultiThreaded,
    )?;
    let elapsed = start.elapsed();
    println!(
        "gitoxide PARALLEL (cache = 64 entries: confirmed {} entries ({} unique objects) in {} trees in {:?} ({:0.0} entries/s, {:0.0} trees/s)",
        entries,
        unique,
        all_commits.len(),
        elapsed,
        entries as f32 / elapsed.as_secs_f32(),
        all_commits.len() as f32 / elapsed.as_secs_f32()
    );

    let start = Instant::now();
    let (unique, entries) = do_gitoxide_tree_dag_traversal(
        &all_commits,
        &db,
        git_odb::pack::cache::lru::StaticLinkedList::<64>::default,
        Computation::SingleThreaded,
    )?;
    let elapsed = start.elapsed();
    println!(
        "gitoxide (cache = 64 entries: confirmed {} entries ({} unique objects) in {} trees in {:?} ({:0.0} entries/s, {:0.0} trees/s)",
        entries,
        unique,
        all_commits.len(),
        elapsed,
        entries as f32 / elapsed.as_secs_f32(),
        all_commits.len() as f32 / elapsed.as_secs_f32()
    );

    let repo = git2::Repository::open(&repo_git_dir)?;
    let start = Instant::now();
    let (unique, entries) = do_libgit2_tree_dag_traversal(&all_commits, &repo)?;
    let elapsed = start.elapsed();
    println!(
        "libgit2: confirmed {} entries ({} unique objects) in {} trees in {:?} ({:0.0} entries/s, {:0.0} trees/s))",
        entries,
        unique,
        all_commits.len(),
        elapsed,
        entries as f32 / elapsed.as_secs_f32(),
        all_commits.len() as f32 / elapsed.as_secs_f32()
    );

    let start = Instant::now();
    let count = do_gitoxide_commit_graph_traversal(commit_id, &db, || {
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
    let count = do_gitoxide_commit_graph_traversal(
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
    let count = do_gitoxide_commit_graph_traversal(commit_id, &db, || git_odb::pack::cache::Never)?;
    let elapsed = start.elapsed();
    let objs_per_sec = |elapsed: Duration| count as f32 / elapsed.as_secs_f32();
    println!(
        "gitoxide (uncached): confirmed {} commits in {:?} ({:0.0} commits/s)",
        count,
        elapsed,
        objs_per_sec(elapsed)
    );

    let start = Instant::now();
    let count = do_libgit2_commit_graph_traversal(commit_id, &repo)?;
    let elapsed = start.elapsed();
    let objs_per_sec = |elapsed: Duration| count as f32 / elapsed.as_secs_f32();
    println!(
        "libgit2: confirmed {} commits in {:?} ({:0.0} commits/s)",
        count,
        elapsed,
        objs_per_sec(elapsed)
    );

    Ok(())
}

fn do_gitoxide_commit_graph_traversal<C>(
    tip: ObjectId,
    db: &git_odb::linked::Db,
    new_cache: impl FnOnce() -> C,
) -> anyhow::Result<usize>
where
    C: git_odb::pack::cache::DecodeEntry,
{
    let mut cache = new_cache();
    let ancestors = commit::Ancestors::new(Some(tip), commit::ancestors::State::default(), |oid, buf| {
        db.find(oid, buf, &mut cache)
            .ok()
            .flatten()
            .and_then(|o| o.into_commit_iter())
    });
    let mut commits = 0;
    for commit_id in ancestors {
        let _ = commit_id?;
        commits += 1;
    }
    Ok(commits)
}

enum Computation {
    SingleThreaded,
    MultiThreaded,
}

fn do_gitoxide_tree_dag_traversal<C>(
    commits: &[ObjectId],
    db: &git_odb::linked::Db,
    new_cache: impl Fn() -> C + Sync + Send,
    mode: Computation,
) -> anyhow::Result<(usize, u64)>
where
    C: git_odb::pack::cache::DecodeEntry,
{
    match mode {
        Computation::SingleThreaded => {
            #[derive(Default)]
            struct Count {
                entries: usize,
                seen: HashSet<ObjectId>,
            }

            impl tree::visit::Visit for Count {
                type PathId = ();
                fn set_current_path(&mut self, _id: Self::PathId) {}
                fn push_tracked_path_component(&mut self, _component: &BStr) -> Self::PathId {}
                fn push_path_component(&mut self, _component: &BStr) {}
                fn pop_path_component(&mut self) {}
                fn visit_tree(&mut self, entry: &Entry<'_>) -> Action {
                    self.entries += 1;
                    let inserted = self.seen.insert(entry.oid.to_owned());
                    if !inserted {
                        tree::visit::Action::Skip
                    } else {
                        tree::visit::Action::Continue
                    }
                }
                fn visit_nontree(&mut self, entry: &Entry<'_>) -> Action {
                    self.entries += 1;
                    self.seen.insert(entry.oid.to_owned());
                    tree::visit::Action::Continue
                }
            }

            let mut cache = new_cache();
            let mut buf = Vec::new();
            let mut state = tree::breadthfirst::State::default();
            let mut seen = HashSet::new();
            let mut entries = 0;

            for commit in commits {
                let tid = db
                    .find(commit, &mut buf, &mut cache)?
                    .and_then(|o| o.into_commit_iter().and_then(|mut c| c.tree_id()))
                    .expect("commit as starting point");

                let mut count = Count { entries: 0, seen };
                tree::breadthfirst::traverse(
                    tid,
                    &mut state,
                    |oid, buf| {
                        db.find_existing(oid, buf, &mut cache)
                            .ok()
                            .and_then(|o| o.into_tree_iter())
                    },
                    &mut count,
                )?;
                entries += count.entries as u64;
                seen = count.seen;
            }
            Ok((seen.len(), entries))
        }
        Computation::MultiThreaded => {
            struct Count<'a> {
                entries: usize,
                seen: &'a DashSet<ObjectId>,
            }

            impl<'a> tree::visit::Visit for Count<'a> {
                type PathId = ();
                fn set_current_path(&mut self, _id: Self::PathId) {}
                fn push_tracked_path_component(&mut self, _component: &BStr) -> Self::PathId {}
                fn push_path_component(&mut self, _component: &BStr) {}
                fn pop_path_component(&mut self) {}
                fn visit_tree(&mut self, entry: &Entry<'_>) -> Action {
                    self.entries += 1;
                    let inserted = self.seen.insert(entry.oid.to_owned());
                    if !inserted {
                        tree::visit::Action::Skip
                    } else {
                        tree::visit::Action::Continue
                    }
                }
                fn visit_nontree(&mut self, entry: &Entry<'_>) -> Action {
                    self.entries += 1;
                    self.seen.insert(entry.oid.to_owned());
                    tree::visit::Action::Continue
                }
            }
            use rayon::prelude::*;
            let seen = DashSet::new();
            let entries = std::sync::atomic::AtomicU64::new(0);

            commits
                .into_par_iter()
                .try_for_each_init::<_, _, _, anyhow::Result<_>>(
                    {
                        let new_cache = &new_cache;
                        let seen = &seen;
                        move || {
                            (
                                Count {
                                    entries: 0,
                                    seen: &seen,
                                },
                                Vec::<u8>::new(),
                                new_cache(),
                                tree::breadthfirst::State::default(),
                            )
                        }
                    },
                    |(count, buf, cache, state), commit| {
                        let tid = db
                            .find_existing(commit, buf, cache)?
                            .into_commit_iter()
                            .and_then(|mut c| c.tree_id())
                            .expect("commit as starting point");
                        tree::breadthfirst::traverse(
                            tid,
                            state,
                            |oid, buf| db.find_existing(oid, buf, cache).ok().and_then(|o| o.into_tree_iter()),
                            count,
                        )?;
                        entries.fetch_add(count.entries as u64, std::sync::atomic::Ordering::Relaxed);
                        Ok(())
                    },
                )?;
            Ok((seen.len(), entries.load(std::sync::atomic::Ordering::Acquire)))
        }
    }
}

fn do_libgit2_tree_dag_traversal(commits: &[ObjectId], db: &git2::Repository) -> anyhow::Result<(usize, u64)> {
    let mut entries = 0;
    let mut seen = HashSet::new();
    for commit in commits {
        let commit = db.find_commit(git2::Oid::from_bytes(commit.as_bytes())?)?;
        commit.tree()?.walk(git2::TreeWalkMode::PreOrder, |_path, entry| {
            entries += 1;
            let was_inserted = seen.insert(entry.id());
            if was_inserted {
                git2::TreeWalkResult::Ok
            } else {
                git2::TreeWalkResult::Skip
            }
        })?;
    }
    Ok((seen.len(), entries))
}

fn do_libgit2_commit_graph_traversal(tip: ObjectId, db: &git2::Repository) -> anyhow::Result<usize> {
    let mut commits = 0;
    let mut walk = db.revwalk()?;
    walk.push(git2::Oid::from_bytes(tip.as_bytes())?)?;

    for commit_id in walk {
        let _ = commit_id?;
        commits += 1;
    }
    Ok(commits)
}
