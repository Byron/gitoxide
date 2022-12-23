// Clone a repository from any URL or Path to a given taget directory

use anyhow::Context;
use git_repository as git;

fn main() -> anyhow::Result<()> {
    let repo_url = std::env::args_os()
        .nth(1)
        .context("First argument needs to be the repository URL")?;

    let dst = std::env::args_os()
        .nth(2)
        .context("Second argument needs to be the directory to clone the repository in")?;

    std::fs::create_dir_all(&dst)?;

    let url = git_url::parse(repo_url.to_str().unwrap().into())?;

    println!("Url: {:?}", url.to_bstring());

    let mut prepare = git::prepare_clone(url, &dst)?;

    println!("Cloning {repo_url:?} into {dst:?}...");

    let (mut checkout, _) =
        prepare.fetch_then_checkout(git::progress::Discard, &std::sync::atomic::AtomicBool::default())?;

    println!(
        "Checking out into {:?} ...",
        checkout.repo().work_dir().expect("should be there")
    );

    let (repo, _) = checkout.main_worktree(git::progress::Discard, &std::sync::atomic::AtomicBool::default())?;

    println!("Repo cloned into {:?}", repo.work_dir().expect("Should be there"));

    let remote = repo
        .find_default_remote(git::remote::Direction::Fetch)
        .expect("Should be there")?;

    println!(
        "Default remote: {} -> {}",
        remote.name().expect("should be origin").as_bstr(),
        remote
            .url(git::remote::Direction::Fetch)
            .expect("should be the remote URL")
            .to_bstring(),
    );

    Ok(())
}
