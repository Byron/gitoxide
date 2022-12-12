const SMALL_PACK_INDEX: &str = "objects/pack/pack-a2bf8e71d8c18879e499335762dd95119d93d9f1.idx";
const SMALL_PACK: &str = "objects/pack/pack-a2bf8e71d8c18879e499335762dd95119d93d9f1.pack";

const INDEX_V1: &str = "objects/pack/pack-c0438c19fb16422b6bbcce24387b3264416d485b.idx";
const PACK_FOR_INDEX_V1: &str = "objects/pack/pack-c0438c19fb16422b6bbcce24387b3264416d485b.pack";

const INDEX_V2: &str = "objects/pack/pack-11fdfa9e156ab73caae3b6da867192221f2089c2.idx";
const PACK_FOR_INDEX_V2: &str = "objects/pack/pack-11fdfa9e156ab73caae3b6da867192221f2089c2.pack";

const PACKS_AND_INDICES: &[(&'static str, &'static str)] =
    &[(SMALL_PACK_INDEX, SMALL_PACK), (INDEX_V1, PACK_FOR_INDEX_V1)];

const V2_PACKS_AND_INDICES: &[(&'static str, &'static str)] =
    &[(SMALL_PACK_INDEX, SMALL_PACK), (INDEX_V2, PACK_FOR_INDEX_V2)];

pub use git_testtools::{fixture_path, hex_to_id, scripted_fixture_read_only};

pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

#[cfg(not(windows))]
pub fn fixup(v: Vec<u8>) -> Vec<u8> {
    v
}

#[cfg(windows)]
pub fn fixup(v: Vec<u8>) -> Vec<u8> {
    // Git checks out text files with line ending conversions, git itself will of course not put '\r\n' anywhere,
    // so that wouldn't be expected in an object and doesn't have to be parsed.
    use bstr::ByteSlice;
    v.replace(b"\r\n", "\n")
}

mod bundle;
mod data;
mod index;
mod iter;
mod multi_index;
