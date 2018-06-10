extern crate git_odb as odb;
extern crate hex;

mod utils;

mod index {
    use odb::pack::index::{File, Kind};
    use utils::fixture;

    const INDEX_V1: &'static str = "packs/pack-c0438c19fb16422b6bbcce24387b3264416d485b.idx";
    const INDEX_V2: &'static str = "packs/pack-11fdfa9e156ab73caae3b6da867192221f2089c2.idx";

    #[test]
    fn index_iter_v1() {
        for (path, kind, len, version) in
            &[(INDEX_V2, Kind::V2, 30, 2), (INDEX_V1, Kind::V1, 67, 1)]
        {
            let idx = File::at(&fixture(path)).unwrap();
            assert_eq!(idx.kind(), *kind);
            assert_eq!(idx.version(), *version);
            assert_eq!(idx.size(), *len);
        }
    }
}
