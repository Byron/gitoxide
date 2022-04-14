use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

pub use bstr;
use bstr::{BStr, ByteSlice};
use io_close::Close;
use nom::error::VerboseError;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
pub use tempfile;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

static SCRIPT_IDENTITY: Lazy<Mutex<BTreeMap<PathBuf, u32>>> = Lazy::new(|| Mutex::new(BTreeMap::new()));

pub fn run_git(working_dir: &Path, args: &[&str]) -> std::io::Result<std::process::ExitStatus> {
    std::process::Command::new("git")
        .current_dir(working_dir)
        .args(args)
        .status()
}

pub fn hex_to_id(hex: &str) -> git_hash::ObjectId {
    git_hash::ObjectId::from_hex(hex.as_bytes()).expect("40 bytes hex")
}

pub fn fixture_path(path: impl AsRef<Path>) -> PathBuf {
    PathBuf::from("tests").join("fixtures").join(path.as_ref())
}
pub fn fixture_bytes(path: impl AsRef<Path>) -> Vec<u8> {
    match std::fs::read(fixture_path(path.as_ref())) {
        Ok(res) => res,
        Err(_) => panic!("File at '{}' not found", path.as_ref().display()),
    }
}
pub fn scripted_fixture_repo_read_only(script_name: impl AsRef<Path>) -> Result<PathBuf> {
    scripted_fixture_repo_read_only_with_args(script_name, None)
}

pub fn scripted_fixture_repo_writable(script_name: &str) -> Result<tempfile::TempDir> {
    scripted_fixture_repo_writable_with_args(script_name, None)
}

pub fn scripted_fixture_repo_writable_with_args(
    script_name: &str,
    args: impl IntoIterator<Item = &'static str>,
) -> Result<tempfile::TempDir> {
    let ro_dir = scripted_fixture_repo_read_only_with_args(script_name, args)?;
    let dst = tempfile::TempDir::new()?;
    copy_recursively_into_existing_dir(&ro_dir, dst.path())?;
    Ok(dst)
}

pub fn copy_recursively_into_existing_dir(src_dir: impl AsRef<Path>, dst_dir: impl AsRef<Path>) -> std::io::Result<()> {
    fs_extra::copy_items(
        &std::fs::read_dir(src_dir)?
            .map(|e| e.map(|e| e.path()))
            .collect::<std::result::Result<Vec<_>, _>>()?,
        dst_dir,
        &fs_extra::dir::CopyOptions {
            overwrite: false,
            skip_exist: false,
            copy_inside: false,
            content_only: false,
            ..Default::default()
        },
    )
    .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
    Ok(())
}

/// Returns the directory at which the data is present
pub fn scripted_fixture_repo_read_only_with_args(
    script_name: impl AsRef<Path>,
    args: impl IntoIterator<Item = &'static str>,
) -> Result<PathBuf> {
    let script_name = script_name.as_ref();
    let script_path = fixture_path(script_name);

    // keep this lock to assure we don't return unfinished directories for threaded callers
    let args: Vec<String> = args.into_iter().map(Into::into).collect();
    let mut map = SCRIPT_IDENTITY.lock();
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

    let script_basename = script_name.file_stem().unwrap_or(script_name.as_os_str());
    let archive_file_path = fixture_path(
        Path::new("generated-archives").join(format!("{}.tar.xz", script_basename.to_str().expect("valid UTF-8"))),
    );
    let script_result_directory = fixture_path(
        Path::new("generated-do-not-edit")
            .join(script_basename)
            .join(format!("{}", script_identity)),
    );

    if !script_result_directory.is_dir() {
        match extract_archive(&archive_file_path, &script_result_directory, script_identity) {
            Ok(_) => {}
            Err(err) => {
                if err.kind() != std::io::ErrorKind::NotFound {
                    eprintln!("{}", err);
                }
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
                    .env("GIT_CONFIG_COUNT", "1")
                    .env("GIT_CONFIG_KEY_0", "commit.gpgsign")
                    .env("GIT_CONFIG_VALUE_0", "false")
                    .output()?;
                assert!(
                    output.status.success(),
                    "repo script failed: stdout: {}\nstderr: {}",
                    output.stdout.as_bstr(),
                    output.stderr.as_bstr()
                );
                create_archive_if_not_on_ci(&script_result_directory, &archive_file_path, script_identity)?;
            }
        }
    }
    Ok(script_result_directory)
}

/// The `script_identity` will be baked into the soon to be created `archive` as it identitifies the script
/// that created the contents of `source_dir`.
fn create_archive_if_not_on_ci(source_dir: &Path, archive: &Path, script_identity: u32) -> std::io::Result<()> {
    if is_ci::cached() {
        return Ok(());
    }
    std::fs::create_dir_all(archive.parent().expect("archive is a file"))?;

    let meta_dir = populate_meta_dir(&source_dir, script_identity)?;
    let res = (move || {
        let mut buf = Vec::<u8>::new();
        {
            let mut ar = tar::Builder::new(&mut buf);
            ar.mode(tar::HeaderMode::Deterministic);
            ar.append_dir_all(".", source_dir)?;
            ar.finish()?;
        }
        let archive = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .append(false)
            .open(archive)?;
        let mut xz_write = xz2::write::XzEncoder::new(archive, 3);
        std::io::copy(&mut &*buf, &mut xz_write)?;
        xz_write.finish()?.close()
    })();
    std::fs::remove_dir_all(meta_dir)?;
    res
}

const META_NAME: &str = "__gitoxide_meta__";
const META_IDENTITY: &str = "identity";
const META_GIT_VERSION: &str = "git-version";

fn populate_meta_dir(destination_dir: &Path, script_identity: u32) -> std::io::Result<PathBuf> {
    let meta_dir = destination_dir.join(META_NAME);
    std::fs::create_dir_all(&meta_dir)?;
    std::fs::write(meta_dir.join(META_IDENTITY), format!("{}", script_identity).as_bytes())?;
    std::fs::write(
        meta_dir.join(META_GIT_VERSION),
        &std::process::Command::new("git").arg("--version").output()?.stdout,
    )?;
    Ok(meta_dir)
}

/// `required_script_identity` is the identity of the script that generated the state that is contained in `archive`.
/// If this is not the case, the arvhive will be ignored.
fn extract_archive(_archive: &Path, destination_dir: &Path, _required_script_identity: u32) -> std::io::Result<()> {
    std::fs::create_dir_all(destination_dir)?;
    // TODO
    Err(std::io::ErrorKind::NotFound.into())
}

pub fn to_bstr_err(err: nom::Err<VerboseError<&[u8]>>) -> VerboseError<&BStr> {
    let err = match err {
        nom::Err::Error(err) | nom::Err::Failure(err) => err,
        nom::Err::Incomplete(_) => unreachable!("not a streaming parser"),
    };
    VerboseError {
        errors: err.errors.into_iter().map(|(i, v)| (i.as_bstr(), v)).collect(),
    }
}
