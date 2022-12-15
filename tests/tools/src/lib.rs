//! Utilities for testing `gitoxide` crates, many of which might be useful for testing programs that use `git` in general.
#![deny(missing_docs)]

use std::{
    collections::BTreeMap,
    convert::Infallible,
    ffi::OsString,
    io::Read,
    path::{Path, PathBuf},
    str::FromStr,
    time::Duration,
};

pub use bstr;
use bstr::{BStr, ByteSlice};
use io_close::Close;
pub use is_ci;
use nom::error::VerboseError;
pub use once_cell;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
pub use tempfile;

/// A result type to allow using the try operator `?` in unit tests.
///
/// Use it like so:
///
/// ```no_run
/// use git_testtools::Result;
///
/// #[test]
/// fn this() -> Result {
///     let x: usize = "42".parse()?;    
///     Ok(())
///
/// }
/// ```
pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

/// A wrapper for a running git-daemon process which is killed automatically on drop.
///
/// Note that we will swallow any errors, assuming that the test would have failed if the daemon crashed.
pub struct GitDaemon {
    child: std::process::Child,
    /// The base url under which all repositories are hosted, typically `git://127.0.0.1:port`.
    pub url: String,
}

impl Drop for GitDaemon {
    fn drop(&mut self) {
        self.child.kill().ok();
    }
}

static SCRIPT_IDENTITY: Lazy<Mutex<BTreeMap<PathBuf, u32>>> = Lazy::new(|| Mutex::new(BTreeMap::new()));
static EXCLUDE_LUT: Lazy<Mutex<Option<git_worktree::fs::Cache<'static>>>> = Lazy::new(|| {
    let cache = (|| {
        let (repo_path, _) = git_discover::upwards(Path::new(".")).ok()?;
        let (git_dir, work_tree) = repo_path.into_repository_and_work_tree_directories();
        let work_tree = work_tree?.canonicalize().ok()?;

        let mut buf = Vec::with_capacity(512);
        let case = git_worktree::fs::Capabilities::probe(&work_tree)
            .ignore_case
            .then(|| git_attributes::glob::pattern::Case::Fold)
            .unwrap_or_default();
        let state = git_worktree::fs::cache::State::IgnoreStack(git_worktree::fs::cache::state::Ignore::new(
            Default::default(),
            git_attributes::MatchGroup::<git_attributes::Ignore>::from_git_dir(git_dir, None, &mut buf).ok()?,
            None,
            case,
        ));
        Some(git_worktree::fs::Cache::new(
            work_tree,
            state,
            case,
            buf,
            Default::default(),
        ))
    })();
    Mutex::new(cache)
});
/// The major, minor and patch level of the git version on the system.
pub static GIT_VERSION: Lazy<(u8, u8, u8)> = Lazy::new(|| parse_git_version().expect("git version to be parsable"));

/// Define how [scripted_fixture_writable_with_args()] uses produces the writable copy.
pub enum Creation {
    /// Run the script once and copy the data from its output to the writable location.
    /// This is fast but won't work if absolute paths are produced by the script.
    CopyFromReadOnly,
    /// Run the script in the writable location. That way, absolute paths match the location.
    ExecuteScript,
}

/// Returns true if the given `major`, `minor` and `patch` is smaller than the actual git version on the system
/// to facilitate skipping a test on the caller.
/// Will never return true on CI which is expected to have a recent enough git version.
///
/// # Panics
///
/// If `git` cannot be executed or if its version output cannot be parsed.
pub fn should_skip_as_git_version_is_smaller_than(major: u8, minor: u8, patch: u8) -> bool {
    if is_ci::cached() {
        return false; // CI should be made to use a recent git version, it should run there.
    }
    *GIT_VERSION < (major, minor, patch)
}

fn parse_git_version() -> Result<(u8, u8, u8)> {
    let git_program = cfg!(windows).then(|| "git.exe").unwrap_or("git");
    let output = std::process::Command::new(git_program).arg("--version").output()?;

    git_version_from_bytes(&output.stdout)
}

fn git_version_from_bytes(bytes: &[u8]) -> Result<(u8, u8, u8)> {
    let mut numbers = bytes
        .split(|b| *b == b' ' || *b == b'\n')
        .nth(2)
        .expect("git version <version>")
        .split(|b| *b == b'.')
        .take(3)
        .map(|n| std::str::from_utf8(n).expect("valid utf8 in version number"))
        .map(u8::from_str);

    Ok((|| -> Result<_> {
        Ok((
            numbers.next().expect("major")?,
            numbers.next().expect("minor")?,
            numbers.next().expect("patch")?,
        ))
    })()
    .map_err(|err| {
        format!(
            "Could not parse version from output of 'git --version' ({:?}) with error: {}",
            bytes.to_str_lossy(),
            err
        )
    })?)
}

/// Run `git` in `working_dir` with all provided `args`.
pub fn run_git(working_dir: &Path, args: &[&str]) -> std::io::Result<std::process::ExitStatus> {
    std::process::Command::new("git")
        .current_dir(working_dir)
        .args(args)
        .status()
}

/// Spawn a git daemon process to host all repository at or below `working_dir`.
pub fn spawn_git_daemon(working_dir: impl AsRef<Path>) -> std::io::Result<GitDaemon> {
    static EXEC_PATH: Lazy<PathBuf> = Lazy::new(|| {
        let path = std::process::Command::new("git")
            .arg("--exec-path")
            .stderr(std::process::Stdio::null())
            .output()
            .expect("can execute `git --exec-path`")
            .stdout;
        String::from_utf8(path.trim().into())
            .expect("no invalid UTF8 in exec-path")
            .into()
    });
    let mut ports: Vec<_> = (9419u16..9419 + 100).collect();
    fastrand::shuffle(&mut ports);
    let addr_at = |port| std::net::SocketAddr::from(([127, 0, 0, 1], port));
    let free_port = {
        let listener = std::net::TcpListener::bind(ports.into_iter().map(addr_at).collect::<Vec<_>>().as_slice())?;
        listener.local_addr().expect("listener address is available").port()
    };

    let child = std::process::Command::new(EXEC_PATH.join(if cfg!(windows) { "git-daemon.exe" } else { "git-daemon" }))
        .current_dir(working_dir)
        .args(["--verbose", "--base-path=.", "--export-all", "--user-path"])
        .arg(format!("--port={free_port}"))
        .spawn()?;

    let server_addr = addr_at(free_port);
    for time in git_lock::backoff::Exponential::default_with_random() {
        std::thread::sleep(time);
        if std::net::TcpStream::connect(server_addr).is_ok() {
            break;
        }
    }
    Ok(GitDaemon {
        child,
        url: format!("git://{}", server_addr),
    })
}

/// Convert a hexadecimal hash into its corresponding `ObjectId` or _panic_.
pub fn hex_to_id(hex: &str) -> git_hash::ObjectId {
    git_hash::ObjectId::from_hex(hex.as_bytes()).expect("40 bytes hex")
}

/// Return the path to the `<crate-root>/tests/fixtures/<path>` directory.
pub fn fixture_path(path: impl AsRef<Path>) -> PathBuf {
    PathBuf::from("tests").join("fixtures").join(path.as_ref())
}

/// Load the fixture from `<crate-root>/tests/fixtures/<path>` and return its data, or _panic_.
pub fn fixture_bytes(path: impl AsRef<Path>) -> Vec<u8> {
    match std::fs::read(fixture_path(path.as_ref())) {
        Ok(res) => res,
        Err(_) => panic!("File at '{}' not found", path.as_ref().display()),
    }
}

/// Run the executable at `script_name`, like `make_repo.sh` or `my_setup.py` to produce a read-only directory to which
/// the path is returned.
///
/// Note that it persists and the script at `script_name` will only be executed once if it ran without error.
///
/// ### Automatic Archive Creation
///
/// In order to speed up CI and even local runs should the cache get purged, the result of each script run
/// is automatically placed into a compressed _tar_ archive.
/// If a script result doesn't exist, these will be checked first and extracted if present, which they are by default.
/// This behaviour can be prohibited by setting the `GITOXIDE_TEST_IGNORE_ARCHIVES` to any value.
///
/// To speed CI up, one can add these archives to the repository. It's absolutely recommended to use `git-lfs` for that to
/// not bloat the repository size.
///
/// #### Disable Archive Creation
///
/// If archives aren't useful, they can be disabled by using `.gitignore` specifications.
/// That way it's trivial to prevent creation of all archives with `generated-archives/*.tar.xz` in the root
/// or more specific `.gitignore` configurations in lower levels of the work tree.
///
/// The latter is useful if the the script's output is platform specific.
pub fn scripted_fixture_read_only(script_name: impl AsRef<Path>) -> Result<PathBuf> {
    scripted_fixture_read_only_with_args(script_name, None::<String>)
}

/// Run the executable at `script_name`, like `make_repo.sh` to produce a writable directory to which
/// the tempdir is returned. It will be removed automatically, courtesy of [`tempfile::TempDir`].
///
/// Note that `script_name` is only executed once, so the data can be copied from its read-only location.
pub fn scripted_fixture_writable(script_name: &str) -> Result<tempfile::TempDir> {
    scripted_fixture_writable_with_args(script_name, None::<String>, Creation::CopyFromReadOnly)
}

/// Like [`scripted_fixture_writable()`], but passes `args` to `script_name` while providing control over
/// the way files are created with `mode`.
pub fn scripted_fixture_writable_with_args(
    script_name: &str,
    args: impl IntoIterator<Item = impl Into<String>>,
    mode: Creation,
) -> Result<tempfile::TempDir> {
    let dst = tempfile::TempDir::new()?;
    Ok(match mode {
        Creation::CopyFromReadOnly => {
            let ro_dir = scripted_fixture_read_only_with_args_inner(script_name, args, None)?;
            copy_recursively_into_existing_dir(ro_dir, dst.path())?;
            dst
        }
        Creation::ExecuteScript => {
            scripted_fixture_read_only_with_args_inner(script_name, args, dst.path().into())?;
            dst
        }
    })
}

/// A utility to copy the entire contents of `src_dir` into `dst_dir`.
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

/// Like `scripted_fixture_read_only()`], but passes `args` to `script_name`.
pub fn scripted_fixture_read_only_with_args(
    script_name: impl AsRef<Path>,
    args: impl IntoIterator<Item = impl Into<String>>,
) -> Result<PathBuf> {
    scripted_fixture_read_only_with_args_inner(script_name, args, None)
}

fn scripted_fixture_read_only_with_args_inner(
    script_name: impl AsRef<Path>,
    args: impl IntoIterator<Item = impl Into<String>>,
    destination_dir: Option<&Path>,
) -> Result<PathBuf> {
    // Assure tempfiles get removed when aborting the test.
    git_lock::tempfile::setup(
        git_lock::tempfile::SignalHandlerMode::DeleteTempfilesOnTerminationAndRestoreDefaultBehaviour,
    );

    let script_location = script_name.as_ref();
    let script_path = fixture_path(script_location);

    // keep this lock to assure we don't return unfinished directories for threaded callers
    let args: Vec<String> = args.into_iter().map(Into::into).collect();
    let script_identity = {
        let mut map = SCRIPT_IDENTITY.lock();
        map.entry(args.iter().fold(script_path.clone(), |p, a| p.join(a)))
            .or_insert_with(|| {
                let crc_value = crc::Crc::<u32>::new(&crc::CRC_32_CKSUM);
                let mut crc_digest = crc_value.digest();
                crc_digest.update(&std::fs::read(&script_path).unwrap_or_else(|err| {
                    panic!(
                        "file {script_path:?} in CWD {:?} could not be read: {err}",
                        std::env::current_dir().expect("valid cwd"),
                    )
                }));
                for arg in args.iter() {
                    crc_digest.update(arg.as_bytes());
                }
                crc_digest.finalize()
            })
            .to_owned()
    };

    let script_basename = script_location.file_stem().unwrap_or(script_location.as_os_str());
    let archive_file_path = fixture_path(
        Path::new("generated-archives").join(format!("{}.tar.xz", script_basename.to_str().expect("valid UTF-8"))),
    );
    let (force_run, script_result_directory) = destination_dir.map(|d| (true, d.to_owned())).unwrap_or_else(|| {
        let dir = fixture_path(Path::new("generated-do-not-edit").join(script_basename).join(format!(
            "{}-{}",
            script_identity,
            family_name()
        )));
        (false, dir)
    });

    let _marker = git_lock::Marker::acquire_to_hold_resource(
        script_basename,
        git_lock::acquire::Fail::AfterDurationWithBackoff(Duration::from_secs(3 * 60)),
        None,
    )?;
    let failure_marker = script_result_directory.join("_invalid_state_due_to_script_failure_");
    if force_run || !script_result_directory.is_dir() || failure_marker.is_file() {
        if failure_marker.is_file() {
            std::fs::remove_dir_all(&script_result_directory)?;
        }
        std::fs::create_dir_all(&script_result_directory)?;
        match extract_archive(&archive_file_path, &script_result_directory, script_identity) {
            Ok((archive_id, platform)) => {
                eprintln!(
                    "Extracted fixture from archive '{}' ({}, {:?})",
                    archive_file_path.display(),
                    archive_id,
                    platform
                )
            }
            Err(err) => {
                if err.kind() != std::io::ErrorKind::NotFound {
                    eprintln!("failed to extract '{}': {}", archive_file_path.display(), err);
                } else if !is_excluded(&archive_file_path) {
                    eprintln!(
                        "Archive at '{}' not found, creating fixture using script '{}'",
                        archive_file_path.display(),
                        script_location.display()
                    );
                }
                let script_absolute_path = std::env::current_dir()?.join(script_path);
                let mut cmd = std::process::Command::new(&script_absolute_path);
                let output = match configure_command(&mut cmd, &args, &script_result_directory).output() {
                    Ok(out) => out,
                    Err(err)
                        if err.kind() == std::io::ErrorKind::PermissionDenied || err.raw_os_error() == Some(193) /* windows */ =>
                    {
                        cmd = std::process::Command::new("bash");
                        configure_command(cmd.arg(script_absolute_path), &args, &script_result_directory).output()?
                    }
                    Err(err) => return Err(err.into()),
                };
                if !output.status.success() {
                    write_failure_marker(&failure_marker);
                }
                assert!(
                    output.status.success(),
                    "fixture script failed: stdout: {}\nstderr: {}",
                    output.stdout.as_bstr(),
                    output.stderr.as_bstr()
                );
                create_archive_if_not_on_ci(&script_result_directory, &archive_file_path, script_identity).map_err(
                    |err| {
                        write_failure_marker(&failure_marker);
                        err
                    },
                )?;
            }
        }
    }
    Ok(script_result_directory)
}

fn configure_command<'a>(
    cmd: &'a mut std::process::Command,
    args: &[String],
    script_result_directory: &Path,
) -> &'a mut std::process::Command {
    cmd.args(args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .current_dir(script_result_directory)
        .env_remove("GIT_DIR")
        .env_remove("GIT_ASKPASS")
        .env_remove("SSH_ASKPASS")
        .env("GIT_TERMINAL_PROMPT", "false")
        .env("GIT_AUTHOR_DATE", "2000-01-01 00:00:00 +0000")
        .env("GIT_AUTHOR_EMAIL", "author@example.com")
        .env("GIT_AUTHOR_NAME", "author")
        .env("GIT_COMMITTER_DATE", "2000-01-02 00:00:00 +0000")
        .env("GIT_COMMITTER_EMAIL", "committer@example.com")
        .env("GIT_COMMITTER_NAME", "committer")
        .env("GIT_CONFIG_COUNT", "4")
        .env("GIT_CONFIG_KEY_0", "commit.gpgsign")
        .env("GIT_CONFIG_VALUE_0", "false")
        .env("GIT_CONFIG_KEY_1", "tag.gpgsign")
        .env("GIT_CONFIG_VALUE_1", "false")
        .env("GIT_CONFIG_KEY_2", "init.defaultBranch")
        .env("GIT_CONFIG_VALUE_2", "main")
        .env("GIT_CONFIG_KEY_3", "protocol.file.allow")
        .env("GIT_CONFIG_VALUE_3", "always")
}

fn write_failure_marker(failure_marker: &Path) {
    std::fs::write(failure_marker, []).ok();
}

fn is_lfs_pointer_file(path: &Path) -> bool {
    const PREFIX: &[u8] = b"version https://git-lfs";
    let mut buf = [0_u8; PREFIX.len()];
    std::fs::OpenOptions::new()
        .read(true)
        .open(path)
        .and_then(|mut f| f.read_exact(&mut buf))
        .map_or(false, |_| buf.starts_with(PREFIX))
}

/// The `script_identity` will be baked into the soon to be created `archive` as it identitifies the script
/// that created the contents of `source_dir`.
fn create_archive_if_not_on_ci(source_dir: &Path, archive: &Path, script_identity: u32) -> std::io::Result<()> {
    if is_ci::cached() {
        return Ok(());
    }
    if is_excluded(archive) {
        return Ok(());
    }
    if is_lfs_pointer_file(archive) {
        eprintln!(
            "Refusing to overwrite `git-lfs` pointer file at \"{}\" - git lfs might not be properly installed.",
            archive.display()
        );
        return Ok(());
    }
    std::fs::create_dir_all(archive.parent().expect("archive is a file"))?;

    let meta_dir = populate_meta_dir(source_dir, script_identity)?;
    let res = (move || {
        let mut buf = Vec::<u8>::new();
        {
            let mut ar = tar::Builder::new(&mut buf);
            ar.mode(tar::HeaderMode::Deterministic);
            ar.follow_symlinks(false);
            ar.append_dir_all(".", source_dir)?;
            ar.finish()?;
        }
        let archive = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(archive)?;
        let mut xz_write = xz2::write::XzEncoder::new(archive, 3);
        std::io::copy(&mut &*buf, &mut xz_write)?;
        xz_write.finish()?.close()
    })();
    #[cfg(not(windows))]
    std::fs::remove_dir_all(meta_dir)?;
    #[cfg(windows)]
    std::fs::remove_dir_all(meta_dir).ok(); // it really can't delete these directories for some reason (even after 10 seconds)
    res
}

fn is_excluded(archive: &Path) -> bool {
    let mut lut = EXCLUDE_LUT.lock();
    lut.as_mut()
        .and_then(|cache| {
            let archive = std::env::current_dir().ok()?.join(archive);
            let relative_path = archive.strip_prefix(cache.base()).ok()?;
            cache
                .at_path(
                    relative_path,
                    Some(false),
                    |_oid, _buf| -> std::result::Result<_, Infallible> { unreachable!("") },
                )
                .ok()?
                .is_excluded()
                .into()
        })
        .unwrap_or(false)
}

const META_DIR_NAME: &str = "__gitoxide_meta__";
const META_IDENTITY: &str = "identity";
const META_GIT_VERSION: &str = "git-version";

fn populate_meta_dir(destination_dir: &Path, script_identity: u32) -> std::io::Result<PathBuf> {
    let meta_dir = destination_dir.join(META_DIR_NAME);
    std::fs::create_dir_all(&meta_dir)?;
    std::fs::write(
        meta_dir.join(META_IDENTITY),
        format!("{}-{}", script_identity, family_name()).as_bytes(),
    )?;
    std::fs::write(
        meta_dir.join(META_GIT_VERSION),
        std::process::Command::new("git").arg("--version").output()?.stdout,
    )?;
    Ok(meta_dir)
}

/// `required_script_identity` is the identity of the script that generated the state that is contained in `archive`.
/// If this is not the case, the arvhive will be ignored.
fn extract_archive(
    archive: &Path,
    destination_dir: &Path,
    required_script_identity: u32,
) -> std::io::Result<(u32, Option<String>)> {
    let archive_buf: Vec<u8> = {
        let mut buf = Vec::new();
        let input_archive = std::fs::File::open(archive)?;
        if std::env::var_os("GITOXIDE_TEST_IGNORE_ARCHIVES").is_some() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Ignoring archive at '{}' as GITOXIDE_TEST_IGNORE_ARCHIVES is set.",
                    archive.display()
                ),
            ));
        }
        let mut decoder = xz2::bufread::XzDecoder::new(std::io::BufReader::new(input_archive));
        std::io::copy(&mut decoder, &mut buf)?;
        buf
    };

    let mut entry_buf = Vec::<u8>::new();
    let (archive_identity, platform): (u32, _) = tar::Archive::new(std::io::Cursor::new(&mut &*archive_buf))
        .entries_with_seek()?
        .filter_map(|e| e.ok())
        .find_map(|mut e: tar::Entry<'_, _>| {
            let path = e.path().ok()?;
            if path.parent()?.file_name()? == META_DIR_NAME && path.file_name()? == META_IDENTITY {
                entry_buf.clear();
                e.read_to_end(&mut entry_buf).ok()?;
                let mut tokens = entry_buf.to_str().ok()?.trim().splitn(2, '-');
                match (tokens.next(), tokens.next()) {
                    (Some(id), platform) => Some((id.parse().ok()?, platform.map(ToOwned::to_owned))),
                    _ => None,
                }
            } else {
                None
            }
        })
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "BUG: Could not find meta directory in our own archive",
            )
        })?;
    if archive_identity != required_script_identity {
        return Err(std::io::ErrorKind::NotFound.into());
    }

    for entry in tar::Archive::new(&mut &*archive_buf).entries()? {
        let mut entry = entry?;
        let path = entry.path()?;
        if path.to_str() == Some(META_DIR_NAME) || path.parent().and_then(|p| p.to_str()) == Some(META_DIR_NAME) {
            continue;
        }
        entry.unpack_in(destination_dir)?;
    }
    Ok((archive_identity, platform))
}

/// Transform a verbose bom errors from raw bytes into a `BStr` to make printing/debugging human-readable.
pub fn to_bstr_err(err: nom::Err<VerboseError<&[u8]>>) -> VerboseError<&BStr> {
    let err = match err {
        nom::Err::Error(err) | nom::Err::Failure(err) => err,
        nom::Err::Incomplete(_) => unreachable!("not a streaming parser"),
    };
    VerboseError {
        errors: err.errors.into_iter().map(|(i, v)| (i.as_bstr(), v)).collect(),
    }
}

fn family_name() -> &'static str {
    if cfg!(windows) {
        "windows"
    } else {
        "unix"
    }
}

/// A utility to set environment variables, while unsetting them (or resetting them to their previous value) on drop.
#[derive(Default)]
pub struct Env<'a> {
    altered_vars: Vec<(&'a str, Option<OsString>)>,
}

impl<'a> Env<'a> {
    /// Create a new instance.
    pub fn new() -> Self {
        Env {
            altered_vars: Vec::new(),
        }
    }

    /// Set `var` to `value`.
    pub fn set(mut self, var: &'a str, value: impl Into<String>) -> Self {
        let prev = std::env::var_os(var);
        std::env::set_var(var, value.into());
        self.altered_vars.push((var, prev));
        self
    }

    /// Set `var` to `value`.
    pub fn unset(mut self, var: &'a str) -> Self {
        let prev = std::env::var_os(var);
        std::env::remove_var(var);
        self.altered_vars.push((var, prev));
        self
    }
}

impl<'a> Drop for Env<'a> {
    fn drop(&mut self) {
        for (var, prev_value) in &self.altered_vars {
            match prev_value {
                Some(value) => std::env::set_var(var, value),
                None => std::env::remove_var(var),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_version() {
        assert_eq!(git_version_from_bytes(b"git version 2.37.2").unwrap(), (2, 37, 2));
        assert_eq!(
            git_version_from_bytes(b"git version 2.32.1 (Apple Git-133)").unwrap(),
            (2, 32, 1)
        );
    }

    #[test]
    fn parse_version_with_trailing_newline() {
        assert_eq!(git_version_from_bytes(b"git version 2.37.2\n").unwrap(), (2, 37, 2));
    }
}
