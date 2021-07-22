#![allow(missing_docs, unused)]

use crate::data::{self, entry::Header, input};
use git_hash::ObjectId;
use std::io::Write;

pub struct LookupRefDeltaObjectsIter<I, LFn> {
    pub inner: I,
    lookup: LFn,
    /// The cached delta to provide next time we are called
    next_delta: Option<input::Entry>,
    /// Fuse to stop iteration after first missing object.
    error: bool,
    /// The overall pack-offset we accumulated thus far. Each inserted entry offsets all following
    /// objects by its length. We need to determine exactly where the object was inserted to see if its affected at all.
    inserted_entry_length_at_offset: Vec<(u64, u64)>,
    /// The sum of all entries added so far, as a cache
    inserted_entries_length_in_bytes: u64,
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
                    let base_entry = match (self.lookup)(base_id, &mut self.buf) {
                        Some(obj) => {
                            let header = to_header(obj.kind);
                            let compressed = {
                                let mut out = git_features::zlib::stream::deflate::Write::new(Vec::new());
                                if let Err(err) = std::io::copy(&mut &*obj.data, &mut out) {
                                    match err.kind() {
                                        std::io::ErrorKind::Other => return Some(Err(input::Error::Io(err))),
                                        err => {
                                            unreachable!("Should never see other errors than zlib, but got {:?}", err,)
                                        }
                                    }
                                };
                                out.flush().expect("zlib flush should never fail");
                                out.into_inner()
                            };
                            let compressed_size = compressed.len() as u64;
                            let entry = input::Entry {
                                header,
                                header_size: header.size(obj.data.len() as u64) as u16,
                                pack_offset: entry.pack_offset,
                                compressed: Some(compressed),
                                compressed_size,
                                crc32: None,
                                decompressed_size: obj.data.len() as u64,
                                trailer: None,
                            };
                            self.inserted_entry_length_at_offset
                                .push((entry.pack_offset, entry.bytes_in_pack()));
                            self.inserted_entries_length_in_bytes += entry.bytes_in_pack();
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
                        entry.pack_offset += base_entry.bytes_in_pack();
                        entry.header_size = entry.header.size(entry.decompressed_size) as u16;
                        self.next_delta = Some(entry);
                    }
                    Some(Ok(base_entry))
                }
                _ => {
                    if self.inserted_entries_length_in_bytes != 0 {
                        if let Header::OfsDelta { base_distance: _ } = entry.header {
                            todo!("add all objects that have been added since to the  delta offset")
                        }
                        entry.pack_offset += self.inserted_entries_length_in_bytes;
                    }
                    Some(Ok(entry))
                }
            },
            other => other,
        }
    }
}

fn to_header(kind: git_object::Kind) -> Header {
    use git_object::Kind::*;
    match kind {
        Tree => Header::Tree,
        Blob => Header::Blob,
        Commit => Header::Commit,
        Tag => Header::Tag,
    }
}
