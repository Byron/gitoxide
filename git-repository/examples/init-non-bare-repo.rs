// cargo run -p git-repository --example new

use anyhow::Context;
use git_repository as git;

fn main() -> anyhow::Result<()> {
    let work_dir = std::env::args_os()
        .nth(1)
        .context("First argument needs to be the directory to initialize the repository in")?;
    let repo = git::init(work_dir)?;

    println!(
        "Repo: {:?}",
        repo.work_dir().expect("non-bare repositories have a work-dir")
    );

    let empty_tree_id = repo.write_object(git::objs::Tree::empty())?;
    let author = git::actor::SignatureRef {
        name: "Maria Sanchez".into(),
        email: "maria@example.com".into(),
        time: git_date::Time::now_local_or_utc(),
    };
    let id = repo.commit(
        "HEAD",
        author,
        author,
        "initial commit",
        empty_tree_id,
        git::commit::NO_PARENT_IDS,
    )?;
    println!("new commit id with empty tree: {:?}", id);
    Ok(())
}
