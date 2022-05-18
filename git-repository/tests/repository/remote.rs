use crate::{named_repo, Result};

#[test]
fn simple() -> Result {
    let repo = named_repo("make_remote_repo.sh")?;

    assert_eq!(
        repo.remote_ref("main")
            .expect("Remote Merge ref exists")
            .expect("Remote Merge ref is valid")
            .shorten(),
        "main"
    );
    assert_eq!(
        repo.branch_remote_name("main").expect("Remote name exists").as_ref(),
        "remote_repo"
    );

    assert!(repo.remote_ref("broken").expect("Remote Merge ref exists").is_err());
    assert!(repo.remote_ref("missing").is_none());
    assert!(repo.branch_remote_name("broken").is_none());

    Ok(())
}
