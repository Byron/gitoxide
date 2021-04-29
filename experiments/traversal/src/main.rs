use anyhow::anyhow;
use git_hash::{bstr::ByteSlice, oid, ObjectId};
use git_object::immutable;
use git_odb::Locate;
use std::{
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
    let count = do_gitoxide_graph_traveral(commit_id, &db, || {
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
    let count = do_gitoxide_graph_traveral(
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
    let count = do_gitoxide_graph_traveral(commit_id, &db, || git_odb::pack::cache::Never)?;
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
    let num_diffs = all_commits.len() + 1;
    let elapsed = start.elapsed();
    println!(
        "gitoxide (uncached): collect all {} commits in {:?} ({:0.0} commits/s)",
        all_commits.len(),
        elapsed,
        all_commits.len() as f32 / elapsed.as_secs_f32()
    );

    let start = Instant::now();
    let num_deltas = do_gitoxide_tree_diff(&all_commits, || {
        |oid, buf| db.locate(oid, buf, &mut git_odb::pack::cache::Never).ok().flatten()
    })?;
    let elapsed = start.elapsed();

    println!(
        "gitoxide-deltas (uncached|uncached): collect {} tree deltas of {} trees-diffs in {:?} ({:0.0} deltas/s, {:0.0} tree-diffs/s)",
        num_deltas,
        num_diffs,
        elapsed,
        num_deltas as f32 / elapsed.as_secs_f32(),
        num_diffs as f32 / elapsed.as_secs_f32()
    );

    Ok(())
}

fn do_gitoxide_tree_diff<C, L>(commits: &[ObjectId], make_locate: C) -> anyhow::Result<usize>
where
    C: Fn() -> L,
    L: for<'b> FnMut(&oid, &'b mut Vec<u8>) -> Option<git_odb::data::Object<'b>>,
{
    let empty_tree = ObjectId::empty_tree();
    let mut buf: Vec<u8> = Vec::new();
    let last = [commits.last().expect("at least one commit").to_owned(), empty_tree];
    let iter = commits.windows(2).chain(std::iter::once(&last[..]));
    let mut find = make_locate();

    fn find_tree<'b, L>(id: &oid, buf: &'b mut Vec<u8>, mut find: L) -> Option<immutable::TreeIter<'b>>
    where
        L: FnMut(&oid, &'b mut Vec<u8>) -> Option<git_odb::data::Object<'b>>,
    {
        find(id, buf).and_then(|o| o.into_tree_iter())
    }
    // let mut find_tree = |tid, buf: &mut Vec<u8>| find(tid, buf).and_then(|o| o.into_tree_iter());
    // let tree_iter_by_commit = |cid| { let tid = find(cid, &mut buf)
    //         .expect("commit present")
    //         .into_commit_iter()
    //         .expect("a commit")
    //         .next()
    //         .expect("tree token")
    //         .expect("tree decodable")
    //         .id()
    //         .expect("first token is a tree id");
    //     find_tree(tid, &mut buf).expect("tree available")
    // };
    // let mut
    for pair in iter {
        let (ca, cb) = (pair[0], pair[1]);
        // let (ta, tb) = (tree_iter_by_commit(&ca), tree_iter_by_commit(&cb));
        // git_diff::visit::Changes::from(ca).needed_to_obtain()
    }
    todo!("tree diff")
}

fn do_gitoxide_graph_traveral<C>(
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
