use std::io::{BufWriter, Write};

use anyhow::Context;
use gix::{objs::Kind, ObjectId};

pub fn connectivity(mut repo: gix::Repository, spec: Option<String>, out: impl std::io::Write) -> anyhow::Result<()> {
    let mut out = BufWriter::with_capacity(64 * 1024, out);
    let spec = spec.unwrap_or("HEAD".into());

    repo.object_cache_size_if_unset(4 * 1024 * 1024);
    // We expect to be finding a bunch of non-existent objects here - never refresh the ODB
    repo.objects.refresh_never();

    let id = repo
        .rev_parse_single(spec.as_str())
        .context("Only single revisions are supported")?;
    let commits: gix::revision::Walk<'_> = id
        .object()?
        .peel_to_kind(gix::object::Kind::Commit)
        .context("Need commitish as starting point")?
        .id()
        .ancestors()
        .all()?;

    let missing_cb = |oid: &ObjectId, kind: Kind| {
        writeln!(out, "{oid}: {kind}").expect("failed to write output");
    };
    let mut conn = gix_fsck::ConnectivityCheck::new(&repo.objects, missing_cb);

    // Walk all commits, checking each one for connectivity
    for commit in commits {
        let commit = commit?;
        conn.check_commit(&commit.id);
        for parent in commit.parent_ids {
            conn.check_commit(&parent);
        }
    }

    Ok(())
}
