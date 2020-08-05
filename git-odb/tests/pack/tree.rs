mod method {
    mod from_offsets_in_pack {
        use crate::{
            fixture_path,
            pack::{INDEX_V1, PACK_FOR_INDEX_V1, SMALL_PACK, SMALL_PACK_INDEX},
        };
        use git_odb::pack;

        #[test]
        fn v1() -> Result<(), Box<dyn std::error::Error>> {
            tree(INDEX_V1, PACK_FOR_INDEX_V1)
        }

        #[test]
        fn v2() -> Result<(), Box<dyn std::error::Error>> {
            tree(SMALL_PACK_INDEX, SMALL_PACK)
        }

        fn tree(index_path: &str, pack_path: &str) -> Result<(), Box<dyn std::error::Error>> {
            let idx = pack::index::File::at(fixture_path(index_path))?;
            pack::tree::Tree::from_offsets_in_pack(
                idx.sorted_offsets().into_iter(),
                |ofs| *ofs,
                fixture_path(pack_path),
                git_features::progress::Discard,
                |id| idx.lookup(id).map(|index| idx.pack_offset_at_index(index)),
            )?;
            Ok(())
        }
    }
}

#[test]
fn using_option_as_data_does_not_increase_size_in_memory() {
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
        600_000_000,
        "it should be as small as possible"
    );
}
