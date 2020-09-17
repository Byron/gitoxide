use crate::{check_common, create_repo, inspect_refs};
use git_commitgraph::Graph;

#[test]
fn test() -> Result<(), Box<dyn std::error::Error>> {
    let repo_dir = create_repo("octopus_merges.sh");
    let refs = inspect_refs(
        repo_dir.path(),
        &[
            "root",
            "parent1",
            "parent2",
            "parent3",
            "parent4",
            "three_parents",
            "four_parents",
        ],
    );
    let cg = Graph::from_info_dir(repo_dir.path().join(".git").join("objects").join("info"))?;
    check_common(&cg, &refs);

    assert_eq!(cg.commit_at(refs["root"].pos()).generation(), 1);
    assert_eq!(cg.commit_at(refs["parent1"].pos()).generation(), 2);
    assert_eq!(cg.commit_at(refs["parent2"].pos()).generation(), 2);
    assert_eq!(cg.commit_at(refs["parent3"].pos()).generation(), 2);
    assert_eq!(cg.commit_at(refs["parent4"].pos()).generation(), 2);
    assert_eq!(cg.commit_at(refs["three_parents"].pos()).generation(), 3);
    assert_eq!(cg.commit_at(refs["four_parents"].pos()).generation(), 3);

    Ok(())
}
