#[test]
fn init_into_empty_directory_creates_a_dot_git_dir() -> crate::Result {
    let tmp = tempfile::tempdir()?;
    let repo = git_repository::init(tmp.path())?;
    assert_eq!(
        repo.work_tree.as_deref(),
        Some(tmp.path()),
        "there is a work tree by default"
    );
    assert_eq!(
        repo.git_dir(),
        tmp.path().join(".git"),
        "there is a work tree by default"
    );
    Ok(())
}

#[test]
fn init_into_non_empty_directory_is_allowed() -> crate::Result {
    let tmp = tempfile::tempdir()?;
    std::fs::write(tmp.path().join("existing.txt"), b"I was here before you")?;

    git_repository::init(tmp.path())?;
    Ok(())
}
