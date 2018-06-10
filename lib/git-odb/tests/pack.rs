extern crate git_odb as odb;
extern crate hex;

mod utils;

use odb::pack::{self, index};
use utils::fixture;
use utils::bin;
use std::mem;

const INDEX_V1: &'static str = "packs/pack-c0438c19fb16422b6bbcce24387b3264416d485b.idx";
const PACK_FOR_INDEX_V1: &'static str = "packs/pack-c0438c19fb16422b6bbcce24387b3264416d485b.pack";
const INDEX_V1_CHECKSUM: &'static str = "5a2b20ef73ffe911178532df86232b64830cb536";
const PACK_V1_CHECKSUM: &'static str = "7ebaef998897d903e6e6b6763d3a6ec4dc5b845b";

const INDEX_V2: &'static str = "packs/pack-11fdfa9e156ab73caae3b6da867192221f2089c2.idx";
const PACK_FOR_INDEX_V2: &'static str = "packs/pack-11fdfa9e156ab73caae3b6da867192221f2089c2.pack";
const INDEX_V2_CHECKSUM: &'static str = "560eba66e6b391eb83efc3ec9fc8a3087788911c";
const PACK_V2_CHECKSUM: &'static str = "f1cd3cc7bc63a4a2b357a475a58ad49b40355470";

#[test]
fn pack_lookup() {
    for (index_path, pack_path) in &[(INDEX_V1, PACK_FOR_INDEX_V1), (INDEX_V2, PACK_FOR_INDEX_V2)] {
        let idx = index::File::at(&fixture(index_path)).unwrap();
        let pack = pack::File::at(&fixture(pack_path)).unwrap();

        assert_eq!(pack.kind(), pack::Kind::V2);
        assert_eq!(pack.size(), idx.size());
        println!("{}", pack_path);
        for entry in idx.iter() {
            mem::drop(pack.entry(entry.offset));
        }
    }
}

#[test]
fn index_iter() {
    for (path, kind, num_objects, version, index_checksum, pack_checksum) in &[
        (
            INDEX_V1,
            index::Kind::V1,
            67,
            1,
            INDEX_V1_CHECKSUM,
            PACK_V1_CHECKSUM,
        ),
        (
            INDEX_V2,
            index::Kind::V2,
            30,
            2,
            INDEX_V2_CHECKSUM,
            PACK_V2_CHECKSUM,
        ),
    ] {
        let idx = index::File::at(&fixture(path)).unwrap();
        assert_eq!(idx.kind(), *kind);
        assert_eq!(idx.version(), *version);
        assert_eq!(idx.size(), *num_objects);
        assert_eq!(idx.checksum_of_index(), bin(index_checksum));
        assert_eq!(idx.checksum_of_pack(), bin(pack_checksum));
        assert_eq!(idx.iter().count(), *num_objects as usize);
    }
}
