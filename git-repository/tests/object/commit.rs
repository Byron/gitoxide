use std::cmp::Ordering;

use git_hash::hex_to_id;

use crate::basic_repo;

#[test]
fn short_id() -> crate::Result {
    let repo = basic_repo()?;
    let commit = repo.head_commit()?;
    assert_eq!(commit.short_id()?.cmp_oid(&commit.id), Ordering::Equal);
    Ok(())
}

#[test]
fn tree() -> crate::Result {
    let repo = basic_repo()?;
    let commit = repo.head_commit()?;

    assert_eq!(commit.tree()?.id, commit.tree_id().expect("id present"));
    assert_eq!(
        commit.tree_id().ok().map(|id| id.detach()),
        Some(hex_to_id("21d3ba9a26b790a4858d67754ae05d04dfce4d0c"))
    );
    Ok(())
}

#[test]
fn decode() -> crate::Result {
    let repo = basic_repo()?;
    let commit = repo.head_commit()?;
    assert_eq!(commit.decode()?.message, commit.message_raw()?);
    assert_eq!(commit.decode()?.message(), commit.message()?);
    assert_eq!(commit.decode()?.message, "c2\n");
    Ok(())
}
