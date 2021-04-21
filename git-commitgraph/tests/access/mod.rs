use crate::{check_common, inspect_refs, make_readonly_repo};
use git_commitgraph::Graph;

#[test]
fn single_parent() -> crate::Result {
    let repo_dir = make_readonly_repo("single_parent.sh");
    let refs = inspect_refs(&repo_dir, &["parent", "child"]);
    let cg = Graph::from_info_dir(repo_dir.join(".git").join("objects").join("info"))?;
    check_common(&cg, &refs);

    assert_eq!(cg.commit_at(refs["parent"].pos()).generation(), 1);
    assert_eq!(cg.commit_at(refs["child"].pos()).generation(), 2);

    Ok(())
}

#[test]
fn octupus_merges() -> crate::Result {
    let repo_dir = make_readonly_repo("octopus_merges.sh");
    let refs = inspect_refs(
        &repo_dir,
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
    let cg = Graph::from_info_dir(repo_dir.join(".git").join("objects").join("info"))?;
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

#[test]
fn single_commit() -> crate::Result {
    let repo_dir = make_readonly_repo("single_commit.sh");
    let refs = inspect_refs(&repo_dir, &["commit"]);
    let cg = Graph::from_info_dir(repo_dir.join(".git").join("objects").join("info"))?;
    check_common(&cg, &refs);

    assert_eq!(cg.commit_at(refs["commit"].pos()).generation(), 1);

    Ok(())
}

#[test]
fn two_parents() -> crate::Result {
    let repo_dir = make_readonly_repo("two_parents.sh");
    let refs = inspect_refs(&repo_dir, &["parent1", "parent2", "child"]);
    let cg = Graph::from_info_dir(repo_dir.join(".git").join("objects").join("info"))?;
    check_common(&cg, &refs);

    assert_eq!(cg.commit_at(refs["parent1"].pos()).generation(), 1);
    assert_eq!(cg.commit_at(refs["parent2"].pos()).generation(), 1);
    assert_eq!(cg.commit_at(refs["child"].pos()).generation(), 2);

    Ok(())
}
