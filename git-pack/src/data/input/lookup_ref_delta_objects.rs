#![allow(missing_docs, unused)]

use crate::data;
use crate::data::input;
use git_hash::ObjectId;

pub struct LookupRefDeltaObjectsIter<I, LFn> {
    pub inner: I,
    lookup: LFn,
    /// The cached delta to provide next time we are called
    next_delta: Option<input::Entry>,
    /// Fuse to stop iteration after first missing object.
    error: bool,
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
            Some(Ok(entry)) => match entry.header {
                crate::data::entry::Header::RefDelta { base_id } => {
                    let entry = match (self.lookup)(base_id, &mut self.buf) {
                        Some(obj) => todo!("object to entry"),
                        None => {
                            self.error = true;
                            return Some(Err(input::Error::NotFound { object_id: base_id }));
                        }
                    };
                    self.next_delta = Some(entry);
                    Some(Ok(entry))
                }
                _ => Some(Ok(entry)),
            },
            other => other,
        }
    }
}
