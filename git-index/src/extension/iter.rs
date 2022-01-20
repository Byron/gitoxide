use std::convert::TryInto;

use crate::{extension, extension::Iter, util::from_be_u32};

impl<'a> Iter<'a> {
    pub fn new(data_at_beginning_of_extensions_and_truncated: &'a [u8]) -> Self {
        Iter {
            data: data_at_beginning_of_extensions_and_truncated,
            consumed: 0,
        }
    }

    pub fn new_without_checksum(
        data_at_beginning_of_extensions: &'a [u8],
        object_hash: git_hash::Kind,
    ) -> Option<Self> {
        let end = data_at_beginning_of_extensions
            .len()
            .checked_sub(object_hash.len_in_bytes())?;
        Iter {
            data: &data_at_beginning_of_extensions[..end],
            consumed: 0,
        }
        .into()
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
        self.data = data;
        self.consumed += 4 + 4;

        let size = from_be_u32(size) as usize;

        match data.get(..size) {
            Some(ext_data) => {
                self.data = &data[size..];
                self.consumed += size;
                Some((signature.try_into().unwrap(), ext_data))
            }
            None => {
                self.data = &[];
                None
            }
        }
    }
}
