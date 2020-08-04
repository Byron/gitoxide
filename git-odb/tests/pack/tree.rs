#[test]
fn using_option_as_data_does_not_increase_size_in_memory() {
    enum Cache {
        _Unset,
        _Decompressed(Vec<u8>),
    }
    enum ObjectKind {
        _Base(git_object::Kind),
        _OfsDelta,
    }
    struct Entry {
        pub _id: Option<git_object::owned::Id>,
        pub _pack_offset: u64,
        pub _entry_len: usize,
        pub _kind: ObjectKind,
        pub _crc32: u32,
        pub _cache: Cache,
    }

    struct TreeItem<D> {
        _offset: u64,
        _data: D,
        _children: Vec<usize>,
    }
    struct TreeItemOption<D> {
        _offset: u64,
        _data: Option<D>,
        _children: Vec<usize>,
    }
    assert_eq!(
        std::mem::size_of::<TreeItem<Entry>>(),
        std::mem::size_of::<TreeItemOption<Entry>>(),
        "we hope niche filling optimizations kick in for our data structures to not pay for the Option at all"
    );
    assert_eq!(
        std::mem::size_of::<[TreeItemOption<Entry>; 7_500_000]>(),
        780000000,
        "it should be as small as possible"
    );
}
