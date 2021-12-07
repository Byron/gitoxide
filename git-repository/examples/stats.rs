use git_repository as git;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo = git_repository::discover(".")?;
    println!(
        "Repo: {}",
        repo.work_tree.as_deref().unwrap_or(repo.git_dir()).display()
    );
    let handle = repo.to_easy();
    let commit_ids = handle
        .head()?
        .into_fully_peeled_id()
        .ok_or_else(|| "There are no commits - nothing to do here.")??
        .ancestors()
        .all()
        .collect::<Result<Vec<_>, _>>()?;
    println!("Num Commits: {}", commit_ids.len());
    assert!(!commit_ids.is_empty(), "checked that before");

    let last_commit_id = &commit_ids[0];
    println!("Most recent commit message");

    let object_ref = last_commit_id.object()?;
    let handle2 = handle.clone();
    let commit = object_ref.to_commit();
    println!("{}", commit.message);
    let tree_ref = handle2.find_object(commit.tree())?.into_tree();
    println!("{:?}", git::objs::TreeRefIter::from_bytes(&tree_ref.data).nth(1));

    Ok(())
}
