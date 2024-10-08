#![allow(clippy::result_large_err)]
use gix::{open, Repository, ThreadSafeRepository};
use gix_testtools::tempfile;
pub use gix_testtools::Result;

/// Convert a hexadecimal hash into its corresponding `ObjectId` or _panic_.
pub fn hex_to_id(hex: &str) -> gix_hash::ObjectId {
    gix_hash::ObjectId::from_hex(hex.as_bytes()).expect("40 bytes hex")
}

pub fn freeze_time() -> gix_testtools::Env<'static> {
    let frozen_time = "1979-02-26 18:30:00";
    gix_testtools::Env::new()
        .unset("GIT_AUTHOR_NAME")
        .unset("GIT_AUTHOR_EMAIL")
        .set("GIT_AUTHOR_DATE", frozen_time)
        .unset("GIT_COMMITTER_NAME")
        .unset("GIT_COMMITTER_EMAIL")
        .set("GIT_COMMITTER_DATE", frozen_time)
}
pub fn repo(name: &str) -> Result<ThreadSafeRepository> {
    let repo_path = gix_testtools::scripted_fixture_read_only(name)?;
    Ok(ThreadSafeRepository::open_opts(repo_path, restricted())?)
}

pub fn repo_opts(name: &str, opts: open::Options) -> std::result::Result<ThreadSafeRepository, open::Error> {
    let repo_path = gix_testtools::scripted_fixture_read_only(name).unwrap();
    ThreadSafeRepository::open_opts(repo_path, opts)
}

pub fn named_repo(name: &str) -> Result<Repository> {
    let repo_path = gix_testtools::scripted_fixture_read_only(name)?;
    Ok(ThreadSafeRepository::open_opts(repo_path, restricted())?.to_thread_local())
}

pub fn named_subrepo_opts(
    fixture: &str,
    name: &str,
    opts: open::Options,
) -> std::result::Result<Repository, gix::open::Error> {
    let repo_path = gix_testtools::scripted_fixture_read_only(fixture).unwrap().join(name);
    Ok(ThreadSafeRepository::open_opts(repo_path, opts)?.to_thread_local())
}

pub fn restricted() -> open::Options {
    open::Options::isolated().config_overrides(["user.name=gitoxide", "user.email=gitoxide@localhost"])
}

pub fn restricted_and_git() -> open::Options {
    let mut opts = restricted();
    opts.permissions.env.git_prefix = gix_sec::Permission::Allow;
    opts.permissions.env.identity = gix_sec::Permission::Allow;
    opts
}

pub fn repo_rw(name: &str) -> Result<(Repository, tempfile::TempDir)> {
    repo_rw_opts(name, restricted())
}

pub fn repo_rw_opts(name: &str, opts: gix::open::Options) -> Result<(Repository, tempfile::TempDir)> {
    let repo_path = gix_testtools::scripted_fixture_writable(name)?;
    Ok((
        ThreadSafeRepository::discover_opts(
            repo_path.path(),
            Default::default(),
            gix_sec::trust::Mapping {
                full: opts.clone(),
                reduced: opts,
            },
        )?
        .to_thread_local(),
        repo_path,
    ))
}

pub fn basic_repo() -> Result<Repository> {
    repo("make_basic_repo.sh").map(|r| r.to_thread_local())
}

pub fn basic_rw_repo() -> Result<(Repository, tempfile::TempDir)> {
    repo_rw("make_basic_repo.sh")
}
