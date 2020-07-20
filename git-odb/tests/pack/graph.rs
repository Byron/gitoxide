mod from_sorted_offsets {
    use crate::{
        fixture_path,
        pack::{SMALL_PACK, SMALL_PACK_INDEX},
    };
    use git_odb::pack;
    use std::{fs, io::BufReader};

    #[test]
    fn pack_v2() {
        let read = BufReader::new(fs::File::open(fixture_path(SMALL_PACK)).unwrap());
        let idx = pack::index::File::at(fixture_path(SMALL_PACK_INDEX)).unwrap();
        let ofs = idx.sorted_offsets();
        pack::graph::DeltaTree::from_sorted_offsets(ofs.into_iter(), read, git_features::progress::Discard).unwrap();
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
