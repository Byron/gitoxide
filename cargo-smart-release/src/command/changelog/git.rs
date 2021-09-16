use std::path::PathBuf;

use cargo_metadata::Metadata;
use git_repository as git;
use git_repository::{
    bstr::{BStr, ByteSlice},
    prelude::ReferenceAccessExt,
};
use std::collections::BTreeMap;

use crate::utils::{is_tag_name, is_tag_version, package_by_name, tag_prefix};
use std::iter::FromIterator;
use std::time::Instant;

/// A head reference will all commits that are 'governed' by it, that is are in its exclusive ancestry.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub struct Segment {
    head: git::refs::Reference,
    commits: Vec<git::hash::ObjectId>,
}

/// Return the head reference followed by all tags affecting `crate_name` as per our tag name rules, ordered by ancestry.
pub fn crate_references_descending(
    crate_name: &str,
    meta: &Metadata,
    repo: &git::Easy,
) -> anyhow::Result<Vec<Segment>> {
    let package = package_by_name(meta, crate_name)?;
    let tag_prefix = tag_prefix(package, repo);
    let start = Instant::now();
    let tags_by_commit = {
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
    Ok(vec![])
}

fn strip_tag_path(fullname: &BStr) -> &BStr {
    fullname
        .strip_prefix(b"refs/tags/")
        .expect("prefix iteration works")
        .as_bstr()
}
