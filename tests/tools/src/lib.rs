pub use bstr;
use once_cell::sync::Lazy;
use std::{collections::BTreeMap, path::Path, path::PathBuf, sync::Mutex};

pub use tempfile;

static SCRIPT_IDENTITY: Lazy<Mutex<BTreeMap<PathBuf, u32>>> = Lazy::new(|| Mutex::new(BTreeMap::new()));

pub fn hex_to_id(hex: &str) -> git_hash::ObjectId {
    git_hash::ObjectId::from_hex(hex.as_bytes()).expect("40 bytes hex")
}

pub fn fixture_path(path: impl AsRef<str>) -> PathBuf {
    PathBuf::from("tests").join("fixtures").join(path.as_ref())
}
pub fn scripted_fixture_repo_read_only(script_name: &str) -> std::result::Result<PathBuf, Box<dyn std::error::Error>> {
    scripted_fixture_repo_read_only_with_args(script_name, None)
}

pub fn scripted_fixture_repo_writable(
    script_name: &str,
) -> std::result::Result<tempfile::TempDir, Box<dyn std::error::Error>> {
    scripted_fixture_repo_writable_with_args(script_name, None)
}

pub fn scripted_fixture_repo_writable_with_args(
    script_name: &str,
    args: impl IntoIterator<Item = &'static str>,
) -> std::result::Result<tempfile::TempDir, Box<dyn std::error::Error>> {
    let ro_dir = scripted_fixture_repo_read_only_with_args(script_name, args)?;
    let dst = tempfile::TempDir::new()?;
    fs_extra::copy_items(
        &[ro_dir],
        dst.path(),
        &fs_extra::dir::CopyOptions {
            overwrite: false,
            skip_exist: false,
            copy_inside: false,
            content_only: false,
            ..Default::default()
        },
    )
    .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
    Ok(dst)
}

/// Returns the directory at which the data is present
pub fn scripted_fixture_repo_read_only_with_args(
    script_name: &str,
    args: impl IntoIterator<Item = &'static str>,
) -> std::result::Result<PathBuf, Box<dyn std::error::Error>> {
    use bstr::ByteSlice;
    let script_path = fixture_path(script_name);

    // keep this lock to assure we don't return unfinished directories for threaded callers
    let args: Vec<String> = args.into_iter().map(Into::into).collect();
    let mut map = SCRIPT_IDENTITY.lock().unwrap();
    let script_identity = map
        .entry(args.iter().fold(script_path.clone(), |p, a| p.join(a)))
        .or_insert_with(|| {
            let crc_value = crc::Crc::<u32>::new(&crc::CRC_32_CKSUM);
            let mut crc_digest = crc_value.digest();
            crc_digest.update(&std::fs::read(&script_path).expect("file can be read entirely"));
            for arg in args.iter() {
                crc_digest.update(arg.as_bytes());
            }
            crc_digest.finalize()
        })
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
            .args(args)
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
