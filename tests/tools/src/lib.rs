pub use bstr;
use once_cell::sync::Lazy;
use std::{collections::BTreeMap, path::Path, path::PathBuf, sync::Mutex};

pub use tempfile;

static SCRIPT_IDENTITY: Lazy<Mutex<BTreeMap<PathBuf, u32>>> = Lazy::new(|| Mutex::new(BTreeMap::new()));

pub fn fixture_path(path: impl AsRef<str>) -> PathBuf {
    PathBuf::from("tests").join("fixtures").join(path.as_ref())
}

/// Returns the directory at which the data is present
pub fn scripted_fixture_repo_read_only(script_name: &str) -> std::result::Result<PathBuf, Box<dyn std::error::Error>> {
    use bstr::ByteSlice;
    let script_path = fixture_path(script_name);

    // keep this lock to assure we don't return unfinished directories for threaded callers
    let mut map = SCRIPT_IDENTITY.lock().unwrap();
    let script_identity = map
        .entry(script_path.clone())
        .or_insert_with(|| crc::crc32::checksum_ieee(&std::fs::read(&script_path).expect("file can be read entirely")))
        .to_owned();
    let script_result_directory = fixture_path(
        Path::new("generated")
            .join(format!("{}", script_identity))
            .to_string_lossy(),
    );
    if !script_result_directory.is_dir() {
        std::fs::create_dir_all(&script_result_directory)?;
        let script_absolute_path = std::env::current_dir()?.join(script_path);
        let output = std::process::Command::new("bash")
            .arg(script_absolute_path)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .current_dir(&script_result_directory)
            .env_remove("GIT_DIR")
            .env("GIT_AUTHOR_DATE", "2000-01-01 00:00:00 +0000")
            .env("GIT_AUTHOR_EMAIL", "author@example.com")
            .env("GIT_AUTHOR_NAME", "author")
            .env("GIT_COMMITTER_DATE", "2000-01-02 00:00:00 +0000")
            .env("GIT_COMMITTER_EMAIL", "committer@example.com")
            .env("GIT_COMMITTER_NAME", "committer")
            .output()?;
        assert!(
            output.status.success(),
            "repo script failed: stdout: {}\nstderr: {}",
            output.stdout.as_bstr(),
            output.stderr.as_bstr()
        );
    }
    Ok(script_result_directory)
}
