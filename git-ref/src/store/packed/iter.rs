use crate::store::{packed, packed::decode};
use bstr::{BString, ByteSlice};

/// packed-refs specific functionality
impl packed::Buffer {
    /// Return an iterator of references stored in this packed refs buffer, ordered by reference name.
    pub fn iter(&self) -> packed::Iter<'_> {
        packed::Iter::new(self)
    }

    /// Return an iterator yielding only references matching the given prefix, ordered by reference name.
    pub fn iter_prefixed(&self, prefix: impl Into<BString>) -> packed::Iter<'_> {
        let prefix = prefix.into();
        let first_record_with_prefix = self.binary_search_by(prefix.as_bstr()).unwrap_or_else(|(_, pos)| pos);
        packed::Iter::new_with_prefix(&self.as_ref()[first_record_with_prefix..], Some(prefix))
    }
}

impl<'a> Iterator for packed::Iter<'a> {
    type Item = Result<packed::Reference<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor.is_empty() {
            return None;
        }

        match decode::reference::<()>(self.cursor) {
            Ok((rest, reference)) => {
                self.cursor = rest;
                self.current_line += 1;
                if let Some(ref prefix) = self.prefix {
                    if !reference.name.as_bstr().starts_with_str(prefix) {
                        self.cursor = &[];
                        return None;
                    }
                }
                Some(Ok(reference))
            }
            Err(_) => {
                let (failed_line, next_cursor) = self
                    .cursor
                    .find_byte(b'\n')
                    .map_or((self.cursor, &[][..]), |pos| self.cursor.split_at(pos + 1));
                self.cursor = next_cursor;
                let line_number = self.current_line;
                self.current_line += 1;

                Some(Err(Error::Reference {
                    invalid_line: failed_line
                        .get(..failed_line.len().saturating_sub(1))
                        .unwrap_or(failed_line)
                        .into(),
                    line_number,
                }))
            }
        }
    }
}

impl<'a> packed::Iter<'a> {
    /// Return a new iterator after successfully parsing the possibly existing first line of the given `packed` refs buffer.
    pub fn new(packed: &'a packed::Buffer) -> Self {
        Self::new_with_prefix(packed, None)
    }

    /// Returns an iterators whose references will only match the given prefix.
    ///
    /// It assumes that the underlying `packed` buffer is indeed sorted
    pub(in crate::store::packed) fn new_with_prefix(packed: &'a packed::Buffer, prefix: Option<BString>) -> Self {
        packed::Iter {
            packed,
            cursor,
            prefix,
            current_line: 1,
        }
    }
}

mod error {
    use bstr::BString;
    use quick_error::quick_error;

    quick_error! {
        /// The error returned by [`Iter::new(…)`][super::Iter::new()],
        #[derive(Debug)]
        #[allow(missing_docs)]
        pub enum Error {
            Reference { invalid_line: BString, line_number: usize } {
                display("Invalid reference in line {}: '{}'", line_number, invalid_line)
            }
        }
    }
}

pub use error::Error;
