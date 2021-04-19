use std::path::PathBuf;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

#[cfg(not(windows))]
fn fixup(v: Vec<u8>) -> Vec<u8> {
    v
}

#[cfg(windows)]
fn fixup(v: Vec<u8>) -> Vec<u8> {
    // Git checks out text files with line ending conversions, git itself will of course not put '\r\n' anywhere,
    // so that wouldn't be expected in an object and doesn't have to be parsed.
    use bstr::ByteSlice;
    v.replace(b"\r\n", "\n")
}

pub fn hex_to_id(hex: &str) -> git_hash::ObjectId {
    git_hash::ObjectId::from_hex(hex.as_bytes()).expect("40 bytes hex")
}

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

mod alternate;
mod compound;
mod linked;
mod loose;
mod pack;
mod sink;
mod traversal;
