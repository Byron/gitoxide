extern crate git_odb as odb;
extern crate hex;

mod utils;

mod index {
    use odb::pack::index::{File, Kind};
    use utils::fixture;
    use utils::bin;

    const INDEX_V1: &'static str = "packs/pack-c0438c19fb16422b6bbcce24387b3264416d485b.idx";
    const INDEX_V1_CHECKSUM: &'static str = "5a2b20ef73ffe911178532df86232b64830cb536";
    const PACK_V1_CHECKSUM: &'static str = "7ebaef998897d903e6e6b6763d3a6ec4dc5b845b";
    const INDEX_V2: &'static str = "packs/pack-11fdfa9e156ab73caae3b6da867192221f2089c2.idx";
    const INDEX_V2_CHECKSUM: &'static str = "560eba66e6b391eb83efc3ec9fc8a3087788911c";
    const PACK_V2_CHECKSUM: &'static str = "f1cd3cc7bc63a4a2b357a475a58ad49b40355470";

    #[test]
    fn index_iter() {
        for (path, kind, len, version, index_checksum, pack_checksum) in &[
            (INDEX_V1, Kind::V1, 67, 1, INDEX_V1_CHECKSUM, PACK_V1_CHECKSUM),
            (INDEX_V2, Kind::V2, 30, 2, INDEX_V2_CHECKSUM, PACK_V2_CHECKSUM),
        ] {
            let idx = File::at(&fixture(path)).unwrap();
            assert_eq!(idx.kind(), *kind);
            assert_eq!(idx.version(), *version);
            assert_eq!(idx.size(), *len);
            assert_eq!(idx.checksum_of_index(), bin(index_checksum));
            assert_eq!(idx.checksum_of_pack(), bin(pack_checksum));
        }
    }
}
