fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo = git_repository::discover(".")?;
    println!(
        "Repo: {}",
        repo.work_tree.as_deref().unwrap_or(repo.git_dir()).display()
    );
    let repo = repo.to_easy();
    let commit_ids = repo
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
    println!("{}", last_commit_id.object()?.to_commit().message);

    Ok(())
}
