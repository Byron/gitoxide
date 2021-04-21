// Re-exports for convenience
use std::path::PathBuf;

pub use bstr;
pub use tempdir;

pub fn fixture_path(path: &str) -> PathBuf {
    PathBuf::from("tests").join("fixtures").join(path)
}

/// Returns the directory at which the data is present
pub fn assure_fixture_repo_present(
    script_name: &str,
) -> std::result::Result<tempdir::TempDir, Box<dyn std::error::Error>> {
    use bstr::ByteSlice;
    let dir = tempdir::TempDir::new(script_name)?;
    let output = std::process::Command::new("bash")
        .arg(std::env::current_dir()?.join(fixture_path(script_name)))
        .arg(dir.path())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .current_dir(dir.path())
        .env_remove("GIT_DIR")
        .output()?;
    assert!(
        output.status.success(),
        "repo script failed: stdout: {}\nstderr: {}",
        output.stdout.as_bstr(),
        output.stderr.as_bstr()
    );
    Ok(dir)
}
