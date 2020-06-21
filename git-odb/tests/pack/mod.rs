const SMALL_PACK_INDEX: &str = "packs/pack-a2bf8e71d8c18879e499335762dd95119d93d9f1.idx";
const SMALL_PACK: &str = "packs/pack-a2bf8e71d8c18879e499335762dd95119d93d9f1.pack";

mod pack {}
mod index {
    use crate::{
        fixture, hex_to_id,
        pack::{SMALL_PACK, SMALL_PACK_INDEX},
    };
    use git_odb::pack::{self, index};
    use pretty_assertions::assert_eq;

    const INDEX_V1: &str = "packs/pack-c0438c19fb16422b6bbcce24387b3264416d485b.idx";
    const PACK_FOR_INDEX_V1: &str = "packs/pack-c0438c19fb16422b6bbcce24387b3264416d485b.pack";

    const INDEX_V2: &str = "packs/pack-11fdfa9e156ab73caae3b6da867192221f2089c2.idx";
    const PACK_FOR_INDEX_V2: &str = "packs/pack-11fdfa9e156ab73caae3b6da867192221f2089c2.pack";

    #[test]
    fn pack_lookup() {
        for (index_path, pack_path) in &[
            (INDEX_V2, PACK_FOR_INDEX_V2),
            (INDEX_V1, PACK_FOR_INDEX_V1),
            (SMALL_PACK_INDEX, SMALL_PACK),
        ] {
            let idx = index::File::at(&fixture(index_path)).unwrap();
            let pack = pack::File::at(&fixture(pack_path)).unwrap();

            assert_eq!(pack.kind(), pack::Kind::V2);
            assert_eq!(pack.num_objects(), idx.num_objects());
            for idx_entry in idx.iter() {
                let pack_entry = pack.entry(idx_entry.offset);
                assert_ne!(pack_entry.offset, idx_entry.offset);
            }
        }
    }

    #[test]
    fn iter() {
        for (path, kind, num_objects, version, index_checksum, pack_checksum) in &[
            (
                INDEX_V1,
                index::Kind::V1,
                67,
                1,
                "5a2b20ef73ffe911178532df86232b64830cb536",
                "7ebaef998897d903e6e6b6763d3a6ec4dc5b845b",
            ),
            (
                INDEX_V2,
                index::Kind::V2,
                30,
                2,
                "560eba66e6b391eb83efc3ec9fc8a3087788911c",
                "f1cd3cc7bc63a4a2b357a475a58ad49b40355470",
            ),
            (
                SMALL_PACK_INDEX,
                index::Kind::V2,
                42,
                2,
                "544a7204a55f6e9cacccf8f6e191ea8f83575de3",
                "0f3ea84cd1bba10c2a03d736a460635082833e59",
            ),
        ] {
            let idx = index::File::at(&fixture(path)).unwrap();
            assert_eq!(idx.kind(), *kind);
            assert_eq!(idx.version(), *version);
            assert_eq!(idx.num_objects(), *num_objects);
            assert_eq!(idx.checksum_of_index(), hex_to_id(index_checksum));
            assert_eq!(idx.checksum_of_pack(), hex_to_id(pack_checksum));
            assert_eq!(idx.iter().count(), *num_objects as usize);
        }
    }
}
