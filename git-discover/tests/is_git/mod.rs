#[cfg(target_os = "macos")]
#[test]
fn verify_on_exfat() -> crate::Result<()> {
    use std::process::Command;

    use git_discover::repository::Kind;

    let fixtures = git_testtools::scripted_fixture_repo_read_only("make_exfat_repo_darwin.sh")?;
    let mount_point = tempfile::tempdir()?;

    let _cleanup = {
        // Mount dmg file
        Command::new("hdiutil")
            .args(&["attach", "-nobrowse", "-mountpoint"])
            .arg(mount_point.path())
            .arg(fixtures.as_path().join("exfat_repo.dmg"))
            .status()?;

        // Ensure that the mount point is always cleaned up
        defer::defer({
            let mount_point = mount_point.path().to_owned();
            move || {
                Command::new("hdiutil")
                    .arg("detach")
                    .arg(&mount_point)
                    .status()
                    .expect("detach temporary test dmg filesystem successfully");
            }
        })
    };

    let is_git = git_discover::is_git(mount_point.path().join(".git"));

    assert!(
        matches!(is_git, Ok(Kind::WorkTree { linked_git_dir: None })),
        "repo on exFAT is recognized as a valid worktree repo"
    );
    Ok(())
}
