use crate::remote;
use git_repository as git;

#[test]
fn persist() -> crate::Result {
    let tmp = git_testtools::tempfile::TempDir::new()?;
    let repo = git::prepare_clone_bare(remote::repo("base").path(), tmp.path())?.persist();
    assert_eq!(repo.is_bare(), true, "repo is now ours and remains");
    Ok(())
}

#[test]
fn clone_bare_into_empty_directory() -> crate::Result {
    let tmp = git_testtools::tempfile::TempDir::new()?;
    // this breaks isolation, but shouldn't be affecting the test. If so, use isolation options for opening the repo.
    let prep = git::prepare_clone_bare(remote::repo("base").path(), tmp.path())?;
    let head = tmp.path().join("HEAD");
    assert!(head.is_file(), "now a bare basic repo is present");
    drop(prep);

    assert!(!head.is_file(), "we cleanup if the clone isn't followed through");
    Ok(())
}

#[test]
fn clone_into_empty_directory() -> crate::Result {
    let tmp = git_testtools::tempfile::TempDir::new()?;
    // this breaks isolation, but shouldn't be affecting the test. If so, use isolation options for opening the repo.
    let prep = git::prepare_clone(remote::repo("base").path(), tmp.path())?;
    let head = tmp.path().join(".git").join("HEAD");
    assert!(head.is_file(), "now a basic repo is present");
    drop(prep);

    assert!(!head.is_file(), "we cleanup if the clone isn't followed through");
    Ok(())
}
