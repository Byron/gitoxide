use crate::{extension, extension::Signature, util::from_be_u32};

pub fn header(data: &[u8]) -> (Signature, u32, &[u8]) {
    let (signature, data) = data.split_at(4);
    let (size, data) = data.split_at(4);
    (signature.try_into().unwrap(), from_be_u32(size), data)
}

pub fn all(maybe_beginning_of_extensions: &[u8], object_hash: git_hash::Kind) -> (Outcome, &[u8]) {
    extension::Iter::new_without_checksum(maybe_beginning_of_extensions, object_hash)
        .map(|mut ext_iter| {
            let mut ext = Outcome::default();
            for (signature, ext_data) in ext_iter.by_ref() {
                match signature {
                    extension::tree::SIGNATURE => {
                        ext.cache_tree = extension::tree::decode(ext_data, object_hash);
                    }
                    extension::end_of_index_entry::SIGNATURE => {} // skip already done
                    extension::index_entry_offset_table::SIGNATURE => {} // not relevant/obtained already
                    mandatory if mandatory[0].is_ascii_lowercase() => match mandatory {
                        _unknown => todo!("error on mandatory extension that we can't handle"),
                    },
                    _unknown => {} // skip unknown extensions, too
                }
            }
            (ext, &maybe_beginning_of_extensions[ext_iter.consumed..])
        })
        .unwrap_or_else(|| (Outcome::default(), maybe_beginning_of_extensions))
}

#[derive(Default)]
pub struct Outcome {
    pub cache_tree: Option<extension::Tree>,
}
