use crate::{fixture_path, hex_to_id};
use git_commitgraph::{Graph, GraphPosition};

const PARENT1: &[u8] = b"7506a95b8062f0bee6ccdee717021a0c710a1959";
const PARENT1_INDEX: GraphPosition = GraphPosition(2);
const PARENT2: &[u8] = b"12ae061ac737392973e59d5df2103a72f1088e45";
const PARENT2_INDEX: GraphPosition = GraphPosition(0);
const MERGE: &[u8] = b"6a23d4bb5ef62fff1b57b898240bf5a9d5813800";
const MERGE_INDEX: GraphPosition = GraphPosition(1);

#[test]
fn v1() -> Result<(), Box<dyn std::error::Error>> {
    let parent1_id = hex_to_id(PARENT1);
    let parent2_id = hex_to_id(PARENT2);
    let merge_id = hex_to_id(MERGE);

    let cg = Graph::from_object_dir(fixture_path("v1/two_parents"))?;
    assert_eq!(cg.num_commits(), 3);
    assert_eq!(cg.id_at(PARENT1_INDEX), parent1_id.to_borrowed());
    assert_eq!(cg.id_at(PARENT2_INDEX), parent2_id.to_borrowed());
    assert_eq!(cg.id_at(MERGE_INDEX), merge_id.to_borrowed());
    assert_eq!(
        cg.iter_ids().collect::<Vec<_>>(),
        vec![
            parent2_id.to_borrowed(),
            merge_id.to_borrowed(),
            parent1_id.to_borrowed()
        ]
    );

    let parent1 = cg.commit_at(PARENT1_INDEX);
    assert_eq!(parent1.generation(), 1);
    assert_eq!(parent1.id(), parent1_id.to_borrowed());
    assert_eq!(parent1.parent1()?, None);
    assert_eq!(
        parent1.root_tree_id(),
        hex_to_id(b"4b825dc642cb6eb9a060e54bf8d69288fbee4904").to_borrowed(),
    );
    assert_eq!(parent1.iter_parent_indices().collect::<Result<Vec<_>, _>>()?, vec![]);

    let parent2 = cg.commit_at(PARENT2_INDEX);
    assert_eq!(parent2.generation(), 1);
    assert_eq!(parent2.id(), parent2_id.to_borrowed());
    assert_eq!(parent2.parent1()?, None);
    assert_eq!(
        parent2.root_tree_id(),
        hex_to_id(b"4b825dc642cb6eb9a060e54bf8d69288fbee4904").to_borrowed(),
    );
    assert_eq!(parent2.iter_parent_indices().collect::<Result<Vec<_>, _>>()?, vec![]);

    let merge = cg.commit_at(MERGE_INDEX);
    assert_eq!(merge.generation(), 2);
    assert_eq!(merge.id(), merge_id.to_borrowed());
    assert_eq!(merge.parent1()?, Some(PARENT1_INDEX));
    assert_eq!(
        merge.root_tree_id(),
        hex_to_id(b"4b825dc642cb6eb9a060e54bf8d69288fbee4904").to_borrowed(),
    );
    assert_eq!(
        merge.iter_parent_indices().collect::<Result<Vec<_>, _>>()?,
        vec![PARENT1_INDEX, PARENT2_INDEX]
    );

    Ok(())
}
