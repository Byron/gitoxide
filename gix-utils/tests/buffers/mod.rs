use gix_utils::Buffers;

#[test]
fn lifecycle() {
    let mut bufs = Buffers::default();
    let mut bufs = bufs.use_foreign_src(b"a");

    assert_eq!(bufs.ro_src.unwrap(), b"a");

    let (src, dest) = bufs.src_and_dest();

    assert_eq!(src, b"a");
    assert_eq!(dest.len(), 0);
    dest.push(b'b');

    bufs.swap();

    assert_eq!(bufs.src, b"b");
    assert!(bufs.ro_src.is_none());

    let (src, dest) = bufs.src_and_dest();
    assert_eq!(src, b"b");
    assert_eq!(
        dest.len(),
        0,
        "the original previously empty source buffer was swapped in "
    );
    dest.push(b'c');
    bufs.swap();
    let (src, dest) = bufs.src_and_dest();
    assert_eq!(src, b"c");
    assert_eq!(dest.len(), 0, "dest always starting  out empty");
}
