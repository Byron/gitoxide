mod method {
    mod delta_tree {
        use crate::{
            fixture_path,
            pack::{INDEX_V1, PACK_FOR_INDEX_V1, SMALL_PACK, SMALL_PACK_INDEX},
        };
        use git_odb::pack;

        #[test]
        fn v1() {
            delta_tree(INDEX_V1, PACK_FOR_INDEX_V1);
        }
        #[test]
        fn v2() {
            delta_tree(SMALL_PACK_INDEX, SMALL_PACK);
        }

        fn delta_tree(index_path: &str, pack_path: &str) {
            let idx = pack::index::File::at(fixture_path(index_path)).unwrap();
            pack::graph::DeltaTree::from_sorted_offsets(
                idx.sorted_offsets().into_iter(),
                fixture_path(pack_path),
                git_features::progress::Discard,
            )
            .unwrap();
        }
    }
}
#[test]
fn size() {
    assert_eq!(
        std::mem::size_of::<petgraph::graph::Node<u64, u32>>(),
        16,
        "Graph Nodes must remain small as these trees are up to 10mio objects"
    )
}
