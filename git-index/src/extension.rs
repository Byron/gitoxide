use crate::{util::read_u32, Version};

const MIN_SIZE: usize = 4 /* signature */ + 4 /* size */;

pub type Signature = [u8; 4];

fn decode_header(data: &[u8]) -> (Signature, u32, &[u8]) {
    let (signature, data) = data.split_at(4);
    let (size, data) = data.split_at(4);
    (signature.try_into().unwrap(), read_u32(size), data)
}

pub(crate) mod end_of_index_entry {
    use crate::{extension, extension::Signature, file::header, util::read_u32};

    pub const SIGNATURE: Signature = *b"EOIE";
    pub const SIZE: usize = 4 /* offset to extensions */ + git_hash::Kind::Sha1.len_in_bytes();
    pub const SIZE_WITH_HEADER: usize = crate::extension::MIN_SIZE + SIZE;

    pub fn decode(data: &[u8], object_hash: git_hash::Kind) -> Option<usize> {
        let hash_len = object_hash.len_in_bytes();
        if data.len() < SIZE_WITH_HEADER + hash_len {
            return None;
        }

        let start_of_eoie = data.len() - SIZE_WITH_HEADER - hash_len;
        let ext_data = &data[start_of_eoie..data.len() - hash_len];

        let (signature, ext_size, ext_data) = extension::decode_header(ext_data);
        if signature != SIGNATURE || ext_size as usize != SIZE {
            return None;
        }

        let (offset, checksum) = ext_data.split_at(4);
        let offset = read_u32(offset) as usize;
        if offset < header::SIZE || offset > start_of_eoie || checksum.len() != git_hash::Kind::Sha1.len_in_bytes() {
            return None;
        }

        let mut hasher = git_features::hash::hasher(git_hash::Kind::Sha1);
        let mut last_chunk = None;
        for (signature, chunk) in extension::Iter::new(&data[offset..data.len() - SIZE_WITH_HEADER - hash_len]) {
            hasher.update(&signature);
            hasher.update(&(chunk.len() as u32).to_be_bytes());
            last_chunk = Some(chunk);
        }

        if hasher.digest() != checksum {
            return None;
        }
        // The last-to-this chunk ends where ours starts
        if last_chunk
            .map(|s| s.as_ptr_range().end != (&data[start_of_eoie]) as *const _)
            .unwrap_or(true)
        {
            return None;
        }

        Some(offset)
    }
}

mod iter {
    use crate::{extension, extension::Iter, util::read_u32};

    impl<'a> Iter<'a> {
        pub fn new(data_at_beginning_of_extensions_and_truncated: &'a [u8]) -> Self {
            Iter {
                data: data_at_beginning_of_extensions_and_truncated,
            }
        }
    }

    impl<'a> Iterator for Iter<'a> {
        type Item = (extension::Signature, &'a [u8]);

        fn next(&mut self) -> Option<Self::Item> {
            if self.data.len() < 4 + 4 {
                return None;
            }

            let (signature, data) = self.data.split_at(4);
            let (size, data) = data.split_at(4);
            let size = read_u32(size) as usize;

            match data.get(..size) {
                Some(ext_data) => {
                    self.data = &data[size..];
                    Some((signature.try_into().unwrap(), ext_data))
                }
                None => {
                    self.data = &[];
                    None
                }
            }
        }
    }
}

pub struct Iter<'a> {
    data: &'a [u8],
}
