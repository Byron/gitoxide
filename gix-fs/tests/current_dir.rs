#[test]
fn precompose_unicode() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let tmp = tempfile::TempDir::new()?;

    let decomposed = "a\u{308}";
    // Note that even on MacOS decomposition doesn't happen reliably, so we assure it's decomposed
    // which then should work everywhere.
    let cwd = tmp.path().join(decomposed);

    std::fs::create_dir(&cwd)?;
    std::env::set_current_dir(&cwd)?;

    let keep_as_is = false;
    let dirname = gix_fs::current_dir(keep_as_is)?
        .file_name()
        .expect("present")
        .to_str()
        .expect("no illformed unicode")
        .to_owned();

    assert_eq!(dirname.chars().count(), decomposed.chars().count());

    let precomposed = "ä";
    let precompose_unicode = true;
    let dirname = gix_fs::current_dir(precompose_unicode)?
        .file_name()
        .expect("present")
        .to_str()
        .expect("no illformed unicode")
        .to_owned();
    assert_eq!(dirname.chars().count(), precomposed.chars().count());
    Ok(())
}
