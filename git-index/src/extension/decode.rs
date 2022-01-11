use crate::extension;
use crate::extension::Signature;
use crate::util::from_be_u32;

pub fn header(data: &[u8]) -> (Signature, u32, &[u8]) {
    let (signature, data) = data.split_at(4);
    let (size, data) = data.split_at(4);
    (signature.try_into().unwrap(), from_be_u32(size), data)
}

pub fn all(beginning_of_extensions: &[u8], object_hash: git_hash::Kind) -> (Outcome, &[u8]) {
    extension::Iter::new_without_checksum(beginning_of_extensions, object_hash)
        .map(|mut ext_iter| {
            let mut ext = Outcome::default();
            for (signature, ext_data) in ext_iter.by_ref() {
                match signature {
                    extension::tree::SIGNATURE => {
                        ext.cache_tree = extension::tree::decode(ext_data, object_hash);
                    }
                    extension::end_of_index_entry::SIGNATURE => {} // skip already done
                    _unknown => {}                                 // skip unknown extensions, too
                }
            }
            (ext, &beginning_of_extensions[ext_iter.consumed..])
        })
        .unwrap_or_else(|| (Outcome::default(), beginning_of_extensions))
}

#[derive(Default)]
pub struct Outcome {
    pub cache_tree: Option<extension::Tree>,
}
