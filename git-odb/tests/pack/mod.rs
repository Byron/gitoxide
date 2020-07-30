const SMALL_PACK_INDEX: &str = "packs/pack-a2bf8e71d8c18879e499335762dd95119d93d9f1.idx";
const SMALL_PACK: &str = "packs/pack-a2bf8e71d8c18879e499335762dd95119d93d9f1.pack";

const INDEX_V1: &str = "packs/pack-c0438c19fb16422b6bbcce24387b3264416d485b.idx";
const PACK_FOR_INDEX_V1: &str = "packs/pack-c0438c19fb16422b6bbcce24387b3264416d485b.pack";

const INDEX_V2: &str = "packs/pack-11fdfa9e156ab73caae3b6da867192221f2089c2.idx";
const PACK_FOR_INDEX_V2: &str = "packs/pack-11fdfa9e156ab73caae3b6da867192221f2089c2.pack";

const PACKS_AND_INDICES: &[(&'static str, &'static str)] =
    &[(SMALL_PACK_INDEX, SMALL_PACK), (INDEX_V1, PACK_FOR_INDEX_V1)];

const V2_PACKS_AND_INDICES: &[(&'static str, &'static str)] =
    &[(SMALL_PACK_INDEX, SMALL_PACK), (INDEX_V2, PACK_FOR_INDEX_V2)];

mod bundle;
mod file;
mod graph;
mod index;
mod iter;
