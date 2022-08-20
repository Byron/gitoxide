// cargo run -p git-repository --example new

use git_repository as git;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tmp = tempfile::TempDir::new()?;
    let work_dir = tmp.path().join("repo-non-bare");
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

    let _persisted = tmp.into_path();
    Ok(())
}
