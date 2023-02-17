use crate::upwards::repo_path;

#[cfg(target_os = "macos")]
#[test]
fn verify_on_exfat() -> crate::Result<()> {
    use std::process::Command;

    use gix_discover::repository::Kind;

    let fixtures = gix_testtools::scripted_fixture_read_only("make_exfat_repo_darwin.sh")?;
    let mount_point = tempfile::tempdir()?;

    let _cleanup = {
        // Mount dmg file
        Command::new("hdiutil")
            .args(["attach", "-nobrowse", "-mountpoint"])
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

    let is_git = gix_discover::is_git(mount_point.path().join(".git"));

    assert!(
        matches!(is_git, Ok(Kind::WorkTree { linked_git_dir: None })),
        "repo on exFAT is recognized as a valid worktree repo"
    );
    Ok(())
}

#[test]
fn missing_configuration_file_is_not_a_dealbreaker_in_bare_repo() -> crate::Result {
    for name in ["bare-no-config-after-init.git", "bare-no-config.git"] {
        let repo = repo_path()?.join(name);
        let kind = gix_discover::is_git(repo)?;
        assert_eq!(kind, gix_discover::repository::Kind::Bare);
    }
    Ok(())
}

#[test]
fn missing_configuration_file_is_not_a_dealbreaker_in_nonbare_repo() -> crate::Result {
    for name in ["worktree-no-config-after-init/.git", "worktree-no-config/.git"] {
        let repo = repo_path()?.join(name);
        let kind = gix_discover::is_git(repo)?;
        assert_eq!(kind, gix_discover::repository::Kind::WorkTree { linked_git_dir: None });
    }
    Ok(())
}
