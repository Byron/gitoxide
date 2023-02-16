use std::convert::TryInto;

use crate::{extension, extension::Iter, util::from_be_u32};

impl<'a> Iter<'a> {
    /// Create a new extension iterator at the entrypoint for extensions until the end of the extensions.
    pub fn new(data_at_beginning_of_extensions_and_truncated: &'a [u8]) -> Self {
        Iter {
            data: data_at_beginning_of_extensions_and_truncated,
            consumed: 0,
        }
    }

    /// Create a new iterator at with a data block to the end of the file, and we automatically remove the trailing
    /// hash of type `object_hash`.
    pub fn new_without_checksum(
        data_at_beginning_of_extensions: &'a [u8],
        object_hash: gix_hash::Kind,
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
