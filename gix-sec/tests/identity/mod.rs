#[test]
fn is_path_owned_by_current_user() -> crate::Result {
    let dir = tempfile::tempdir()?;
    let file = dir.path().join("file");
    std::fs::write(&file, [])?;
    assert!(gix_sec::identity::is_path_owned_by_current_user(&file)?);
    assert!(gix_sec::identity::is_path_owned_by_current_user(dir.path())?);
    Ok(())
}

#[test]
#[cfg(windows)]
fn windows_home() -> crate::Result {
    let home = gix_path::env::home_dir().expect("home dir is available");
    assert!(gix_sec::identity::is_path_owned_by_current_user(&home)?);
    Ok(())
}
