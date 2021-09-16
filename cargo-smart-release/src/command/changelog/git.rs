#![allow(unused)]
use std::{collections::BTreeMap, iter::FromIterator, path::PathBuf, time::Instant};

use anyhow::bail;
use cargo_metadata::Metadata;
use git_repository as git;
use git_repository::{
    bstr::{BStr, ByteSlice},
    easy::head,
    prelude::ReferenceAccessExt,
};

use crate::utils::{is_tag_name, is_tag_version, package_by_name, tag_prefix};
use git_repository::prelude::{ObjectAccessExt, ReferenceExt, RepositoryAccessExt};

/// A head reference will all commits that are 'governed' by it, that is are in its exclusive ancestry.
pub struct Segment<'a> {
    head: git::refs::Reference,
    /// only relevant history items, that is those that change code in the respective crate.
    history: Vec<&'a HistoryItem>,
}

pub struct History {
    head: git::refs::Reference,
    items: Vec<HistoryItem>,
}

pub struct HistoryItem {
    id: git::hash::ObjectId,
    message: git::bstr::BString,
    tree_data: Vec<u8>,
}

pub fn commit_history(repo: &git::Easy) -> anyhow::Result<Option<History>> {
    let start = Instant::now();
    let prev = repo.object_cache(64 * 1024)?;
    let reference = match repo.head()?.peeled()?.kind {
        head::Kind::Detached { .. } => bail!("Refusing to operate on a detached head."),
        head::Kind::Unborn { .. } => return Ok(None),
        head::Kind::Symbolic(r) => r.attach(repo),
    };

    let mut items = Vec::new();
    for commit_id in reference.id().ancestors()?.all() {
        let commit_id = commit_id?;
        let (message, tree_id) = {
            let object = commit_id.object()?;
            let commit = object.commit()?;
            (commit.message.to_owned(), commit.tree())
        };

        items.push(HistoryItem {
            id: commit_id.detach(),
            message,
            tree_data: repo.find_object(tree_id)?.data.to_owned(),
        });
    }
    repo.object_cache(None)?;

    let elapsed = start.elapsed();
    log::trace!(
        "Cached commit history of {} commits and trees in {}s ({:.0} items/s)",
        items.len(),
        elapsed.as_secs_f32(),
        items.len() as f32 / elapsed.as_secs_f32()
    );
    Ok(Some(History {
        head: reference.detach(),
        items,
    }))
}

/// Return the head reference followed by all tags affecting `crate_name` as per our tag name rules, ordered by ancestry.
pub fn crate_references_descending<'h>(
    crate_name: &str,
    meta: &Metadata,
    repo: &git::Easy,
    history: &'h History,
) -> anyhow::Result<Vec<Segment<'h>>> {
    let package = package_by_name(meta, crate_name)?;
    let tag_prefix = tag_prefix(package, repo);
    let start = Instant::now();
    let mut tags_by_commit = {
        let refs = repo.references()?;
        match tag_prefix {
            Some(prefix) => BTreeMap::from_iter(
                refs.prefixed(PathBuf::from(format!("refs/tags/{}-", prefix)))?
                    .peeled()
                    .filter_map(|r| r.ok().map(|r| r.detach()))
                    .filter(|r| is_tag_name(prefix, strip_tag_path(r.name.as_bstr())))
                    .map(|r| {
                        let t = r.peeled.expect("already peeled");
                        (t, r)
                    }),
            ),
            None => BTreeMap::from_iter(
                refs.prefixed("refs/tags")?
                    .peeled()
                    .filter_map(|r| r.ok().map(|r| r.detach()))
                    .filter(|r| is_tag_version(strip_tag_path(r.name.as_bstr())))
                    .map(|r| {
                        let t = r.peeled.expect("already peeled");
                        (t, r)
                    }),
            ),
        }
    };

    let elapsed = start.elapsed();
    log::trace!(
        "Mapped {} tags in {}s ({:.0} refs/s)",
        tags_by_commit.len(),
        elapsed.as_secs_f32(),
        tags_by_commit.len() as f32 / elapsed.as_secs_f32()
    );

    let start = Instant::now();
    let mut segments = Vec::new();
    let mut segment = Segment {
        head: history.head.to_owned(),
        history: vec![],
    };
    for item in &history.items {
        match tags_by_commit.remove(&item.id) {
            None => {
                // TODO: filter by touched panifest path, fast or slow path
                segment.history.push(item)
            }
            Some(next_ref) => segments.push(std::mem::replace(
                &mut segment,
                Segment {
                    head: next_ref,
                    history: vec![item],
                },
            )),
        }
    }
    segments.push(segment);

    if !tags_by_commit.is_empty() {
        log::warn!(
            "The following tags were on branches which are ignored during traversal: {}",
            tags_by_commit
                .into_values()
                .map(|v| v.name.as_bstr().to_str_lossy().into_owned())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }

    let elapsed = start.elapsed();
    let num_commits = segments.iter().map(|s| s.history.len()).sum::<usize>();
    log::trace!(
        "Found {} relevant commits out of {} in {} segments {}s ({:.0} commits/s)",
        num_commits,
        history.items.len(),
        segments.len(),
        elapsed.as_secs_f32(),
        num_commits as f32 / elapsed.as_secs_f32()
    );

    Ok(segments)
}

fn strip_tag_path(fullname: &BStr) -> &BStr {
    fullname
        .strip_prefix(b"refs/tags/")
        .expect("prefix iteration works")
        .as_bstr()
}
