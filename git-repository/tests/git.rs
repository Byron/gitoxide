use git_repository::{open, permission, permissions, Permissions, Repository, ThreadSafeRepository};

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

fn repo(name: &str) -> Result<ThreadSafeRepository> {
    let repo_path = git_testtools::scripted_fixture_repo_read_only(name)?;
    Ok(ThreadSafeRepository::open_opts(repo_path, restricted())?)
}

fn named_repo(name: &str) -> Result<Repository> {
    let repo_path = git_testtools::scripted_fixture_repo_read_only(name)?;
    Ok(ThreadSafeRepository::open_opts(repo_path, restricted())?.to_thread_local())
}

fn restricted() -> open::Options {
    open::Options::default().permissions(Permissions {
        config: permissions::Config {
            system: false,
            git: false,
            user: false,
            env: false,
            includes: false,
        },
        env: {
            let deny = permission::env_var::Resource::resource(git_sec::Permission::Deny);
            permissions::Environment {
                xdg_config_home: deny.clone(),
                home: deny.clone(),
                git_prefix: deny,
            }
        },
        ..Permissions::default()
    })
}

fn repo_rw(name: &str) -> Result<(Repository, tempfile::TempDir)> {
    let repo_path = git_testtools::scripted_fixture_repo_writable(name)?;
    Ok((
        ThreadSafeRepository::discover_opts(
            repo_path.path(),
            Default::default(),
            git_sec::trust::Mapping {
                full: restricted(),
                reduced: restricted(),
            },
        )?
        .to_thread_local(),
        repo_path,
    ))
}

fn basic_repo() -> Result<Repository> {
    repo("make_basic_repo.sh").map(|r| r.to_thread_local())
}

fn basic_rw_repo() -> Result<(Repository, tempfile::TempDir)> {
    repo_rw("make_basic_repo.sh")
}

mod commit;
mod id;
mod init;
mod object;
mod reference;
mod repository;
