use crate::{fixture_path, hex_to_id};
use git_commitgraph::{Graph, GraphPosition};

const COMMIT: &[u8] = b"771fa42310bb3d221f4cf82613a491b0957d2003";

#[test]
fn v1() -> Result<(), Box<dyn std::error::Error>> {
    let commit_id = hex_to_id(COMMIT);

    let cg = Graph::from_info_dir(fixture_path("v1/single_commit/info"))?;
    assert_eq!(cg.num_commits(), 1);
    assert_eq!(cg.id_at(GraphPosition(0)), commit_id.to_borrowed());
    assert_eq!(cg.iter_ids().collect::<Vec<_>>(), vec![commit_id.to_borrowed()]);

    let commit = cg.commit_at(GraphPosition(0));
    assert_eq!(commit.generation(), 1);
    assert_eq!(commit.id(), commit_id.to_borrowed());
    assert_eq!(commit.parent1()?, None);
    assert_eq!(
        commit.root_tree_id(),
        hex_to_id(b"4b825dc642cb6eb9a060e54bf8d69288fbee4904").to_borrowed(),
    );
    assert_eq!(commit.iter_parent_indices().collect::<Result<Vec<_>, _>>()?, vec![]);

    Ok(())
}
