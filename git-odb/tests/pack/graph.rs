mod delta_tree {
    mod from_offsets_in_pack {
        use crate::{
            fixture_path,
            pack::{INDEX_V1, PACK_FOR_INDEX_V1, SMALL_PACK, SMALL_PACK_INDEX},
        };
        use git_odb::pack;

        #[test]
        fn v1() -> Result<(), Box<dyn std::error::Error>> {
            delta_tree(INDEX_V1, PACK_FOR_INDEX_V1)
        }

        #[test]
        fn v2() -> Result<(), Box<dyn std::error::Error>> {
            delta_tree(SMALL_PACK_INDEX, SMALL_PACK)
        }

        fn delta_tree(index_path: &str, pack_path: &str) -> Result<(), Box<dyn std::error::Error>> {
            let idx = pack::index::File::at(fixture_path(index_path))?;
            pack::graph::DeltaTree::from_offsets_in_pack(
                idx.sorted_offsets().into_iter(),
                fixture_path(pack_path),
                git_features::progress::Discard,
                |_| None,
            )?;
            Ok(())
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
