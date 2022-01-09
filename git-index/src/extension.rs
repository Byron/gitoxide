use crate::{util::read_u32, Version};

const MIN_SIZE: usize = 4 /* signature */ + 4 /* size */;

fn decode_header(data: &[u8]) -> ([u8; 4], u32, &[u8]) {
    let (signature, data) = data.split_at(4);
    let (size, data) = data.split_at(4);
    (signature.try_into().unwrap(), read_u32(size), data)
}

mod end_of_index_entry {
    use crate::{extension, extension::EndOfIndexEntry, file::header, util::read_u32};

    impl EndOfIndexEntry {
        pub fn from_bytes(data: &[u8], object_hash: git_hash::Kind) -> Option<Self> {
            let hash_len = object_hash.len_in_bytes();
            if data.len() < EndOfIndexEntry::SIZE_WITH_HEADER + hash_len {
                return None;
            }

            let start_of_eoie = data.len() - EndOfIndexEntry::SIZE_WITH_HEADER - hash_len;
            let data = &data[start_of_eoie..][..hash_len];

            let (signature, ext_size, data) = extension::decode_header(data);
            if signature != EndOfIndexEntry::SIGNATURE || ext_size as usize != EndOfIndexEntry::SIZE {
                return None;
            }

            let (offset, hash) = data.split_at(4);
            let offset = read_u32(offset) as usize;
            if offset < header::SIZE || offset > start_of_eoie {
                return None;
            }
            todo!("eoie")
        }
    }
}

pub struct EndOfIndexEntry {
    /// The offset the the beginning of all extensions, or the end of all entries.
    offset_to_extensions: u32,
    /// The SHA1 checksum over the signature and size of all extensions.
    checksum: git_hash::ObjectId,
}

impl EndOfIndexEntry {
    pub const SIGNATURE: &'static [u8] = b"EOIE";
    pub const SIZE: usize = 4 /* offset to extensions */ + git_hash::Kind::Sha1.len_in_bytes();
    pub const SIZE_WITH_HEADER: usize = crate::extension::MIN_SIZE + Self::SIZE;
}
