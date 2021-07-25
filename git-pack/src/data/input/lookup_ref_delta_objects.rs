#![allow(missing_docs)]

use crate::data::{self, entry::Header, input};
use git_hash::ObjectId;
use std::convert::TryInto;

pub struct LookupRefDeltaObjectsIter<I, LFn> {
    pub inner: I,
    lookup: LFn,
    /// The cached delta to provide next time we are called
    next_delta: Option<input::Entry>,
    /// Fuse to stop iteration after first missing object.
    error: bool,
    /// The overall pack-offset we accumulated thus far. Each inserted entry offsets all following
    /// objects by its length. We need to determine exactly where the object was inserted to see if its affected at all.
    inserted_entry_length_at_offset: Vec<Change>,
    /// The sum of all entries added so far, as a cache
    inserted_entries_length_in_bytes: i64,
    buf: Vec<u8>,
}

impl<I, LFn> LookupRefDeltaObjectsIter<I, LFn>
where
    I: Iterator<Item = Result<input::Entry, input::Error>>,
    LFn: for<'a> FnMut(ObjectId, &'a mut Vec<u8>) -> Option<data::Object<'a>>,
{
    pub fn new(iter: I, lookup: LFn) -> Self {
        LookupRefDeltaObjectsIter {
            inner: iter,
            lookup,
            error: false,
            inserted_entry_length_at_offset: Vec::new(),
            inserted_entries_length_in_bytes: 0,
            next_delta: None,
            buf: Vec::new(),
        }
    }

    fn shifted_pack_offset(&self, pack_offset: u64) -> u64 {
        let new_ofs = pack_offset as i64 + self.inserted_entries_length_in_bytes;
        new_ofs.try_into().expect("offset value is never becomes negative")
    }

    /// positive `size_change` values mean an object grew or was more commonly, was inserted. Negative values
    /// mean the object shrunk, usually because there header changed from ref-deltas to ofs deltas.
    fn track_change(
        &mut self,
        shifted_pack_offset: u64,
        pack_offset: u64,
        size_change: i64,
        oid: impl Into<Option<ObjectId>>,
    ) {
        if size_change == 0 {
            return;
        }
        self.inserted_entry_length_at_offset.push(Change {
            shifted_pack_offset,
            pack_offset,
            size_change_in_bytes: size_change,
            oid: oid.into().unwrap_or_else(ObjectId::null_sha1),
        });
        self.inserted_entries_length_in_bytes += size_change;
    }

    fn shift_entry_and_point_to_base_by_offset(&mut self, entry: &mut input::Entry, base_distance: u64) {
        let pack_offset = entry.pack_offset;
        entry.pack_offset = self.shifted_pack_offset(pack_offset);
        entry.header = Header::OfsDelta { base_distance };
        let previous_header_size = entry.header_size;
        entry.header_size = entry.header.size(entry.decompressed_size) as u16;

        let change = entry.header_size as i64 - previous_header_size as i64;
        entry.crc32 = Some(entry.compute_crc32());
        self.track_change(entry.pack_offset, pack_offset, change, None);
    }
}

impl<I, LFn> Iterator for LookupRefDeltaObjectsIter<I, LFn>
where
    I: Iterator<Item = Result<input::Entry, input::Error>>,
    LFn: for<'a> FnMut(ObjectId, &'a mut Vec<u8>) -> Option<data::Object<'a>>,
{
    type Item = Result<input::Entry, input::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.error {
            return None;
        }
        if let Some(delta) = self.next_delta.take() {
            return Some(Ok(delta));
        }
        match self.inner.next() {
            Some(Ok(mut entry)) => match entry.header {
                Header::RefDelta { base_id } => {
                    match self.inserted_entry_length_at_offset.iter().rfind(|e| e.oid == base_id) {
                        None => {
                            let base_entry = match (self.lookup)(base_id, &mut self.buf) {
                                Some(obj) => {
                                    let current_pack_offset = entry.pack_offset;
                                    let mut entry = match input::Entry::from_data_obj(&obj, 0) {
                                        Err(err) => return Some(Err(err)),
                                        Ok(e) => e,
                                    };
                                    entry.pack_offset = self.shifted_pack_offset(current_pack_offset);
                                    self.track_change(
                                        entry.pack_offset,
                                        current_pack_offset,
                                        entry.bytes_in_pack() as i64,
                                        base_id,
                                    );
                                    entry
                                }
                                None => {
                                    self.error = true;
                                    return Some(Err(input::Error::NotFound { object_id: base_id }));
                                }
                            };

                            {
                                self.shift_entry_and_point_to_base_by_offset(&mut entry, base_entry.bytes_in_pack());
                                self.next_delta = Some(entry);
                            }
                            Some(Ok(base_entry))
                        }
                        Some(base_entry) => {
                            let base_distance =
                                self.shifted_pack_offset(entry.pack_offset) - base_entry.shifted_pack_offset;
                            self.shift_entry_and_point_to_base_by_offset(&mut entry, base_distance);
                            Some(Ok(entry))
                        }
                    }
                }
                _ => {
                    if self.inserted_entries_length_in_bytes != 0 {
                        if let Header::OfsDelta { base_distance } = entry.header {
                            let base_pack_offset = entry
                                .pack_offset
                                .checked_sub(base_distance)
                                .expect("distance to be in range of pack");
                            match self
                                .inserted_entry_length_at_offset
                                .binary_search_by_key(&base_pack_offset, |c| c.pack_offset)
                            {
                                Ok(index) => {
                                    let index = {
                                        let maybe_index_of_actual_entry = index + 1;
                                        self.inserted_entry_length_at_offset
                                            .get(maybe_index_of_actual_entry)
                                            .and_then(|c| {
                                                if c.pack_offset == base_pack_offset {
                                                    Some(maybe_index_of_actual_entry)
                                                } else {
                                                    None
                                                }
                                            })
                                            .unwrap_or(index)
                                    };
                                    let new_distance = self
                                        .shifted_pack_offset(entry.pack_offset)
                                        .checked_sub(self.inserted_entry_length_at_offset[index].shifted_pack_offset)
                                        .expect("a base that is behind us in the pack");
                                    self.shift_entry_and_point_to_base_by_offset(&mut entry, new_distance);
                                }
                                Err(index) => {
                                    let change_since_offset = self.inserted_entry_length_at_offset[index..]
                                        .iter()
                                        .map(|c| c.size_change_in_bytes)
                                        .sum::<i64>();
                                    let new_distance: u64 = {
                                        (base_distance as i64 + change_since_offset)
                                            .try_into()
                                            .expect("it still points behind us")
                                    };
                                    self.shift_entry_and_point_to_base_by_offset(&mut entry, new_distance);
                                }
                            }
                        } else {
                            entry.pack_offset = self.shifted_pack_offset(entry.pack_offset);
                        }
                    }
                    Some(Ok(entry))
                }
            },
            other => other,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

#[derive(Debug)]
struct Change {
    pack_offset: u64,
    shifted_pack_offset: u64,
    size_change_in_bytes: i64,
    oid: ObjectId,
}
