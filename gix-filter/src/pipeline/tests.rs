mod buffers {
    use bstr::ByteSlice;

    use crate::pipeline::util::Buffers;

    #[test]
    fn usage() {
        let mut backing = Buffers::default();
        let mut bufs = backing.with_src(b"a");

        {
            let (src, dest) = bufs.src_and_dest();
            assert_eq!(src.as_bstr(), "a");
            assert!(dest.is_empty());
            dest.push(b'b');
        }
        assert!(bufs.ro_src.is_some(), "read-only source remains until swap");
        bufs.swap();
        assert!(
            bufs.ro_src.is_none(),
            "after swap it's not used anymore as recent data is in owned buffers"
        );

        let (src, dest) = bufs.src_and_dest();
        assert_eq!(src.as_bstr(), "b", "buffers were swapped");
        assert_eq!(dest.as_bstr(), "", "destination is new and cleared");
        dest.push(b'c');
        bufs.swap();

        let (src, dest) = bufs.src_and_dest();
        assert_eq!(src.as_bstr(), "c");
        assert_eq!(dest.as_bstr(), "", "destination is cleared");

        let mut bufs = backing.with_src(b"z");
        let (src, dest) = bufs.src_and_dest();
        assert_eq!(src.as_bstr(), "z");
        assert_eq!(dest.as_bstr(), "", "output buffer was cleared by `with_src()`")
    }
}
