use gix_testtools::TestResult;

fn fixture(scenario: &str) -> gix_testtools::Result<std::path::PathBuf> {
    gix_testtools::scripted_fixture_read_only_with_args("make_symlink_preflight.sh", [scenario])
}

#[test]
fn supported_links_preserve_script_arguments_and_do_not_leak_to_child_shells() -> TestResult {
    let dir = fixture("supported")?;
    if !gix_testtools::fixture_has_symlinks(&dir)? {
        return Ok(());
    }
    assert_eq!(
        std::fs::read_to_string(dir.join("nested/scenario"))?,
        "supported",
        "sourcing the script preserves its positional arguments and script name"
    );
    assert!(
        dir.join("nested/link").symlink_metadata()?.file_type().is_symlink(),
        "a successful preflight allows real fixture links to be created"
    );
    assert_eq!(
        std::fs::read_dir(&dir)?.count(),
        2,
        "only the result marker and fixture contents remain after the probe"
    );
    Ok(())
}

#[test]
fn unsupported_links_stop_successfully_before_fixture_generation() -> TestResult {
    let dir = fixture("unsupported")?;
    assert!(
        !gix_testtools::fixture_has_symlinks(&dir)?,
        "an unavailable preflight is an explicit reason to skip"
    );
    assert!(
        !dir.join("nested/scenario").exists(),
        "fixture generation must stop at the unavailable preflight"
    );
    assert_eq!(
        std::fs::read_dir(&dir)?.count(),
        2,
        "a failed link probe also cleans its temporary files"
    );
    Ok(())
}

#[test]
fn copied_links_and_later_failures_are_not_skips() -> TestResult {
    assert!(
        fixture("copy").is_err(),
        "ln succeeding without creating a real symlink is a fixture error"
    );
    for scenario in ["crash", "unexpected"] {
        let err = fixture(scenario).expect_err("unexpected ln exits must remain fixture errors");
        assert!(
            err.to_string().contains("failed with exit code: 1")
                || err.to_string().contains("failed with exit status: 1"),
            "the failure includes the script's exit status: {err}"
        );
    }
    let dir = fixture("supported")?;
    if !gix_testtools::fixture_has_symlinks(&dir)? {
        return Ok(());
    }
    assert!(
        fixture("failure").is_err(),
        "an unrelated error after a successful preflight must fail fixture generation"
    );
    Ok(())
}

#[test]
fn missing_and_malformed_results_are_errors() -> TestResult {
    let dir = gix_testtools::tempfile::tempdir()?;
    let err =
        gix_testtools::fixture_has_symlinks(dir.path()).expect_err("a missing marker is not an unsupported preflight");
    assert_eq!(
        err.kind(),
        std::io::ErrorKind::NotFound,
        "fixtures must explicitly run the preflight"
    );
    let diagnostic = err.to_string();
    assert!(
        diagnostic.contains("requires the Bash fixture script to call gix_testtools_require_symlinks"),
        "the error explains the missing Bash-side preflight: {diagnostic}"
    );
    assert!(
        diagnostic.contains(&dir.path().join("__gix_testtools_symlinks__").display().to_string()),
        "the error identifies the expected marker location: {diagnostic}"
    );
    assert!(
        diagnostic.contains("Pass the fixture root, not a repository subdirectory"),
        "the error also explains how to correct a misplaced Rust-side query: {diagnostic}"
    );
    std::fs::write(dir.path().join("__gix_testtools_symlinks__"), b"invalid\n")?;
    assert_eq!(
        gix_testtools::fixture_has_symlinks(dir.path())
            .expect_err("malformed markers are not reasons to skip")
            .kind(),
        std::io::ErrorKind::InvalidData,
        "only recognized preflight outcomes are accepted"
    );
    Ok(())
}
