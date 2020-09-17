use crate::{fixture_path, hex_to_id};
use git_commitgraph::{Graph, GraphPosition};

const PARENT: &[u8] = b"ddbbe1220676705c5b7a6bb7047e0d63b58cfc28";
const CHILD: &[u8] = b"91bfe17bfdfef01c4c38e34e577b01a14a1ef2d4";

#[test]
fn v1() -> Result<(), Box<dyn std::error::Error>> {
    let parent_id = hex_to_id(PARENT);
    let child_id = hex_to_id(CHILD);

    let cg = Graph::from_info_dir(fixture_path("v1/single_parent/info"))?;
    assert_eq!(cg.num_commits(), 2);
    assert_eq!(cg.id_at(GraphPosition(0)), child_id.to_borrowed());
    assert_eq!(
        cg.iter_ids().collect::<Vec<_>>(),
        vec![child_id.to_borrowed(), parent_id.to_borrowed()]
    );

    let parent = cg.commit_at(GraphPosition(1));
    assert_eq!(parent.generation(), 1);
    assert_eq!(parent.id(), parent_id.to_borrowed());
    assert_eq!(parent.parent1()?, None);
    assert_eq!(
        parent.root_tree_id(),
        hex_to_id(b"4b825dc642cb6eb9a060e54bf8d69288fbee4904").to_borrowed(),
    );
    assert_eq!(parent.iter_parent_indices().collect::<Result<Vec<_>, _>>()?, vec![]);

    let child = cg.commit_at(GraphPosition(0));
    assert_eq!(child.generation(), 2);
    assert_eq!(child.id(), child_id.to_borrowed());
    assert_eq!(child.parent1()?, Some(GraphPosition(1)));
    assert_eq!(
        child.root_tree_id(),
        hex_to_id(b"4b825dc642cb6eb9a060e54bf8d69288fbee4904").to_borrowed(),
    );
    assert_eq!(
        child.iter_parent_indices().collect::<Result<Vec<_>, _>>()?,
        vec![GraphPosition(1)]
    );

    Ok(())
}
