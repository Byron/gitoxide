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
                                    let entry = match input::Entry::from_data_obj(&obj, entry.pack_offset) {
                                        Err(err) => return Some(Err(err)),
                                        Ok(e) => e,
                                    };
                                    self.inserted_entry_length_at_offset.push(Change {
                                        at_pack_offset: entry.pack_offset,
                                        _change_in_bytes: entry.bytes_in_pack() as i64,
                                        oid: base_id,
                                    });
                                    self.inserted_entries_length_in_bytes += entry.bytes_in_pack() as i64;
                                    entry
                                }
                                None => {
                                    self.error = true;
                                    return Some(Err(input::Error::NotFound { object_id: base_id }));
                                }
                            };

                            {
                                entry.header = Header::OfsDelta {
                                    base_distance: base_entry.bytes_in_pack(),
                                };
                                let previous_headersize = entry.header_size;
                                entry.header_size = entry.header.size(entry.decompressed_size) as u16;
                                assert!(
                                    self.inserted_entries_length_in_bytes >= 0,
                                    "we should never have a negative value here"
                                );
                                entry.pack_offset += self.inserted_entries_length_in_bytes.unsigned_abs();

                                let change = entry.header_size as i64 - previous_headersize as i64;
                                self.inserted_entry_length_at_offset.push(Change {
                                    at_pack_offset: entry.pack_offset,
                                    _change_in_bytes: change,
                                    oid: ObjectId::null_sha1(),
                                });
                                self.inserted_entries_length_in_bytes += change;
                                self.next_delta = Some(entry);
                            }
                            Some(Ok(base_entry))
                        }
                        Some(base_entry) => {
                            entry.pack_offset = {
                                let new_ofs = entry.pack_offset as i64 + self.inserted_entries_length_in_bytes;
                                new_ofs.try_into().expect("offset value is never becomes negative")
                            };
                            entry.header = Header::OfsDelta {
                                base_distance: entry.pack_offset - base_entry.at_pack_offset,
                            };
                            entry.header_size = entry.header.size(entry.decompressed_size) as u16;
                            Some(Ok(entry))
                        }
                    }
                }
                _ => {
                    if self.inserted_entries_length_in_bytes != 0 {
                        if let Header::OfsDelta { base_distance: _ } = entry.header {
                            todo!("add all objects that have been added since to the  delta offset")
                        }
                        entry.pack_offset = {
                            let new_ofs = entry.pack_offset as i64 + self.inserted_entries_length_in_bytes;
                            new_ofs.try_into().expect("offset value is never becomes negative")
                        };
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

struct Change {
    at_pack_offset: u64,
    _change_in_bytes: i64,
    oid: ObjectId,
}
