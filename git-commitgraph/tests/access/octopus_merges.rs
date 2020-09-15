use crate::{fixture_path, hex_to_id};
use git_commitgraph::{Graph, GraphPosition};

const ROOT: &[u8] = b"09e3185550fdb352031df65f0e81e74df3b9a556";
const ROOT_INDEX: GraphPosition = GraphPosition(0);
const COMMIT1: &[u8] = b"33316871b4ab988ea87ba60aaf0138ddb4fb0d21";
const COMMIT1_INDEX: GraphPosition = GraphPosition(2);
const COMMIT2: &[u8] = b"cbfe147d1826a0d2701f7c80dbe0e2cfba385b15";
const COMMIT2_INDEX: GraphPosition = GraphPosition(6);
const COMMIT3: &[u8] = b"9dab5d610702ceca4bd889a5ef2662cacfd53178";
const COMMIT3_INDEX: GraphPosition = GraphPosition(5);
const COMMIT4: &[u8] = b"2ec748404455c0b0451b186851a815da13918ebb";
const COMMIT4_INDEX: GraphPosition = GraphPosition(1);
const THREE_PARENTS: &[u8] = b"7070cd0b59cd46a0f0fa0c50ca9148390ae1e7c7";
const THREE_PARENTS_INDEX: GraphPosition = GraphPosition(4);
const FOUR_PARENTS: &[u8] = b"5ca46a72d91817add3a7218f3eaf8b2b6c1caa13";
const FOUR_PARENTS_INDEX: GraphPosition = GraphPosition(3);

#[test]
fn v1() -> Result<(), Box<dyn std::error::Error>> {
    let root_id = hex_to_id(ROOT);
    let commit1_id = hex_to_id(COMMIT1);
    let commit2_id = hex_to_id(COMMIT2);
    let commit3_id = hex_to_id(COMMIT3);
    let commit4_id = hex_to_id(COMMIT4);
    let three_parents_id = hex_to_id(THREE_PARENTS);
    let four_parents_id = hex_to_id(FOUR_PARENTS);

    let cg = Graph::from_object_dir(fixture_path("v1/octopus_merges"))?;
    assert_eq!(cg.num_commits(), 7);
    assert_eq!(cg.id_at(ROOT_INDEX), root_id.to_borrowed());
    assert_eq!(cg.id_at(COMMIT1_INDEX), commit1_id.to_borrowed());
    assert_eq!(cg.id_at(COMMIT2_INDEX), commit2_id.to_borrowed());
    assert_eq!(cg.id_at(COMMIT3_INDEX), commit3_id.to_borrowed());
    assert_eq!(cg.id_at(COMMIT4_INDEX), commit4_id.to_borrowed());
    assert_eq!(cg.id_at(THREE_PARENTS_INDEX), three_parents_id.to_borrowed());
    assert_eq!(cg.id_at(FOUR_PARENTS_INDEX), four_parents_id.to_borrowed());
    assert_eq!(
        cg.iter_ids().collect::<Vec<_>>(),
        vec![
            root_id.to_borrowed(),
            commit4_id.to_borrowed(),
            commit1_id.to_borrowed(),
            four_parents_id.to_borrowed(),
            three_parents_id.to_borrowed(),
            commit3_id.to_borrowed(),
            commit2_id.to_borrowed(),
        ]
    );

    let root = cg.commit_at(ROOT_INDEX);
    assert_eq!(root.generation(), 1);
    assert_eq!(root.id(), root_id.to_borrowed());
    assert_eq!(root.parent1()?, None);
    assert_eq!(
        root.root_tree_id(),
        hex_to_id(b"4b825dc642cb6eb9a060e54bf8d69288fbee4904").to_borrowed(),
    );
    assert_eq!(root.iter_parent_indices().collect::<Result<Vec<_>, _>>()?, vec![]);

    let commit1 = cg.commit_at(COMMIT1_INDEX);
    assert_eq!(commit1.generation(), 2);
    assert_eq!(commit1.id(), commit1_id.to_borrowed());
    assert_eq!(commit1.parent1()?, Some(ROOT_INDEX));
    assert_eq!(
        commit1.root_tree_id(),
        hex_to_id(b"4b825dc642cb6eb9a060e54bf8d69288fbee4904").to_borrowed(),
    );
    assert_eq!(
        commit1.iter_parent_indices().collect::<Result<Vec<_>, _>>()?,
        vec![ROOT_INDEX],
    );

    let commit2 = cg.commit_at(COMMIT2_INDEX);
    assert_eq!(commit2.generation(), 2);
    assert_eq!(commit2.id(), commit2_id.to_borrowed());
    assert_eq!(commit2.parent1()?, Some(ROOT_INDEX));
    assert_eq!(
        commit2.root_tree_id(),
        hex_to_id(b"4b825dc642cb6eb9a060e54bf8d69288fbee4904").to_borrowed(),
    );
    assert_eq!(
        commit2.iter_parent_indices().collect::<Result<Vec<_>, _>>()?,
        vec![ROOT_INDEX],
    );

    let commit3 = cg.commit_at(COMMIT3_INDEX);
    assert_eq!(commit3.generation(), 2);
    assert_eq!(commit3.id(), commit3_id.to_borrowed());
    assert_eq!(commit3.parent1()?, Some(ROOT_INDEX));
    assert_eq!(
        commit3.root_tree_id(),
        hex_to_id(b"4b825dc642cb6eb9a060e54bf8d69288fbee4904").to_borrowed(),
    );
    assert_eq!(
        commit3.iter_parent_indices().collect::<Result<Vec<_>, _>>()?,
        vec![ROOT_INDEX],
    );

    let commit4 = cg.commit_at(COMMIT4_INDEX);
    assert_eq!(commit4.generation(), 2);
    assert_eq!(commit4.id(), commit4_id.to_borrowed());
    assert_eq!(commit4.parent1()?, Some(ROOT_INDEX));
    assert_eq!(
        commit4.root_tree_id(),
        hex_to_id(b"4b825dc642cb6eb9a060e54bf8d69288fbee4904").to_borrowed(),
    );
    assert_eq!(
        commit4.iter_parent_indices().collect::<Result<Vec<_>, _>>()?,
        vec![ROOT_INDEX],
    );

    let three_parents = cg.commit_at(THREE_PARENTS_INDEX);
    assert_eq!(three_parents.generation(), 3);
    assert_eq!(three_parents.id(), three_parents_id.to_borrowed());
    assert_eq!(three_parents.parent1()?, Some(COMMIT1_INDEX));
    assert_eq!(
        three_parents.root_tree_id(),
        hex_to_id(b"4b825dc642cb6eb9a060e54bf8d69288fbee4904").to_borrowed(),
    );
    assert_eq!(
        three_parents.iter_parent_indices().collect::<Result<Vec<_>, _>>()?,
        vec![COMMIT1_INDEX, COMMIT2_INDEX, COMMIT3_INDEX],
    );

    let four_parents = cg.commit_at(FOUR_PARENTS_INDEX);
    assert_eq!(four_parents.generation(), 3);
    assert_eq!(four_parents.id(), four_parents_id.to_borrowed());
    assert_eq!(four_parents.parent1()?, Some(COMMIT2_INDEX));
    assert_eq!(
        four_parents.root_tree_id(),
        hex_to_id(b"4b825dc642cb6eb9a060e54bf8d69288fbee4904").to_borrowed(),
    );
    assert_eq!(
        four_parents.iter_parent_indices().collect::<Result<Vec<_>, _>>()?,
        vec![COMMIT2_INDEX, COMMIT1_INDEX, COMMIT3_INDEX, COMMIT4_INDEX],
    );

    Ok(())
}
