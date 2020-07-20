#[test]
fn size() {
    assert_eq!(
        std::mem::size_of::<petgraph::graph::Node<u64, u32>>(),
        16,
        "Graph Nodes must remain small as these trees are up to 10mio objects"
    )
}
