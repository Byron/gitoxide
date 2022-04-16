#[test]
fn is_path_owned_by_current_user() -> crate::Result {
    let dir = tempfile::tempdir()?;
    let file = dir.path().join("file");
    std::fs::write(&file, &[])?;
    assert!(git_sec::identity::is_path_owned_by_current_user(file)?);
    assert!(git_sec::identity::is_path_owned_by_current_user(dir.path())?);
    Ok(())
}
