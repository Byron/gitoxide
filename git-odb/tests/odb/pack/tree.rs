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

struct TreeItem<D> {
    _offset: u64,
    _data: D,
    _children: Vec<usize>,
}

#[test]
fn using_option_as_data_does_not_increase_size_in_memory() {
    struct Entry {
        pub _id: Option<git_hash::ObjectId>,
        pub _crc32: u32,
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
        480_000_000,
        "it should be as small as possible"
    );
}

#[test]
fn size_of_pack_verify_data_structure() {
    use git_odb::pack;
    pub struct EntryWithDefault {
        _index_entry: pack::index::Entry,
        _kind: git_object::Kind,
        _object_size: u64,
        _decompressed_size: u64,
        _compressed_size: u64,
        _header_size: u16,
        _level: u16,
    }

    assert_eq!(
        std::mem::size_of::<[TreeItem<EntryWithDefault>; 7_500_000]>(),
        780_000_000
    );
}
