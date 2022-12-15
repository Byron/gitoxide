use git_repository::{open, Repository, ThreadSafeRepository};
use git_testtools::tempfile;

pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn freeze_time() -> git_testtools::Env<'static> {
    let frozen_time = "1979-02-26 18:30:00";
    git_testtools::Env::new()
        .unset("GIT_AUTHOR_NAME")
        .unset("GIT_AUTHOR_EMAIL")
        .set("GIT_AUTHOR_DATE", frozen_time)
        .unset("GIT_COMMITTER_NAME")
        .unset("GIT_COMMITTER_EMAIL")
        .set("GIT_COMMITTER_DATE", frozen_time)
}
pub fn repo(name: &str) -> Result<ThreadSafeRepository> {
    let repo_path = git_testtools::scripted_fixture_read_only(name)?;
    Ok(ThreadSafeRepository::open_opts(repo_path, restricted())?)
}

pub fn named_repo(name: &str) -> Result<Repository> {
    let repo_path = git_testtools::scripted_fixture_read_only(name)?;
    Ok(ThreadSafeRepository::open_opts(repo_path, restricted())?.to_thread_local())
}

pub fn named_subrepo_opts(fixture: &str, name: &str, opts: open::Options) -> Result<Repository> {
    let repo_path = git_testtools::scripted_fixture_read_only(fixture)?.join(name);
    Ok(ThreadSafeRepository::open_opts(repo_path, opts)?.to_thread_local())
}

pub fn restricted() -> open::Options {
    open::Options::isolated()
}

pub fn restricted_and_git() -> open::Options {
    let mut opts = open::Options::isolated();
    opts.permissions.env.git_prefix = git_sec::Permission::Allow;
    opts.permissions.env.identity = git_sec::Permission::Allow;
    opts
}

pub fn repo_rw(name: &str) -> Result<(Repository, tempfile::TempDir)> {
    repo_rw_opts(name, restricted())
}

pub fn repo_rw_opts(name: &str, opts: git_repository::open::Options) -> Result<(Repository, tempfile::TempDir)> {
    let repo_path = git_testtools::scripted_fixture_writable(name)?;
    Ok((
        ThreadSafeRepository::discover_opts(
            repo_path.path(),
            Default::default(),
            git_sec::trust::Mapping {
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
