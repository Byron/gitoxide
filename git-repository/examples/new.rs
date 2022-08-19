// cargo run -p git-repository --example new

use git_repository as git;
use std::fs;
use std::path::Path;

const NEW_REPO_PATH: &str = "example-new-repo";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(NEW_REPO_PATH);
    if path.exists() {
        fs::remove_dir_all(path)?;
    }
    let repo = git::init(path)?.apply_environment();
    println!(
        "Repo: {}",
        repo.work_dir().as_deref().unwrap_or(repo.git_dir()).display()
    );
    let empty_tree_id = repo.write_object(&git::objs::Tree::empty()).unwrap().detach();
    let author = git::actor::Signature {
        name: "Maria Sanchez".into(),
        email: "maria@example.com".into(),
        time: git_date::Time::now_local_or_utc(),
    };
    let id = repo.commit(
        "HEAD",
        author.to_ref(),
        author.to_ref(),
        "initial commit",
        empty_tree_id,
        git::commit::NO_PARENT_IDS,
    )?;
    println!("new object id: {:?}", id);
    Ok(())
}
