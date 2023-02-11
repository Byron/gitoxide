use std::iter::FromIterator;

use crate::{named_repo, remote, remote::cow_str, Result};

#[test]
fn remote_and_branch_names() {
    let repo = remote::repo("base");
    assert_eq!(repo.remote_names().len(), 0, "there are no remotes");
    assert_eq!(repo.branch_names().len(), 0, "there are no configured branches");
    assert_eq!(repo.remote_default_name(gix::remote::Direction::Fetch), None);
    assert_eq!(repo.remote_default_name(gix::remote::Direction::Push), None);

    let repo = remote::repo("clone");
    assert_eq!(
        Vec::from_iter(repo.remote_names().into_iter()),
        vec!["myself", "origin"]
    );
    assert_eq!(
        repo.remote_default_name(gix::remote::Direction::Fetch),
        Some(cow_str("origin"))
    );
    assert_eq!(
        repo.remote_default_name(gix::remote::Direction::Push),
        Some(cow_str("origin"))
    );
    assert_eq!(Vec::from_iter(repo.branch_names()), vec!["main"]);
}

#[test]
fn remote_default_name() {
    let repo = remote::repo("push-default");

    assert_eq!(
        repo.remote_default_name(gix::remote::Direction::Push),
        Some(cow_str("myself")),
        "overridden via remote.pushDefault"
    );

    assert_eq!(
        repo.remote_default_name(gix::remote::Direction::Fetch),
        None,
        "none if name origin, and there are multiple"
    );
}

#[test]
fn branch_remote() -> Result {
    let repo = named_repo("make_remote_repo.sh")?;

    assert_eq!(
        repo.branch_remote_ref("main")
            .expect("Remote Merge ref exists")
            .expect("Remote Merge ref is valid")
            .shorten(),
        "main"
    );
    assert_eq!(
        repo.branch_remote_name("main").expect("Remote name exists").as_ref(),
        "remote_repo"
    );

    assert!(repo
        .branch_remote_ref("broken")
        .expect("Remote Merge ref exists")
        .is_err());
    assert!(repo.branch_remote_ref("missing").is_none());
    assert!(repo.branch_remote_name("broken").is_none());

    Ok(())
}
