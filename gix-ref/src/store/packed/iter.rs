use gix_object::bstr::{BString, ByteSlice};
use winnow::{
    combinator::{preceded, rest},
    prelude::*,
    stream::Stream as _,
};

use crate::store_impl::{packed, packed::decode};

/// packed-refs specific functionality
impl packed::Buffer {
    /// Return an iterator of references stored in this packed refs buffer, ordered by reference name.
    ///
    /// # Note
    ///
    /// There is no namespace support in packed iterators. It can be emulated using `iter_prefixed(…)`.
    pub fn iter(&self) -> Result<packed::Iter<'_>, packed::iter::Error> {
        packed::Iter::new(self.as_ref())
    }

    /// Return an iterator yielding only references matching the given prefix, ordered by reference name.
    pub fn iter_prefixed(&self, prefix: BString) -> Result<packed::Iter<'_>, packed::iter::Error> {
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

        let start = self.cursor.checkpoint();
        match decode::reference::<()>.parse_next(&mut self.cursor) {
            Ok(reference) => {
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
                self.cursor.reset(start);
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
    pub fn new(packed: &'a [u8]) -> Result<Self, Error> {
        Self::new_with_prefix(packed, None)
    }

    /// Returns an iterators whose references will only match the given prefix.
    ///
    /// It assumes that the underlying `packed` buffer is indeed sorted
    pub(in crate::store_impl::packed) fn new_with_prefix(
        packed: &'a [u8],
        prefix: Option<BString>,
    ) -> Result<Self, Error> {
        if packed.is_empty() {
            Ok(packed::Iter {
                cursor: packed,
                prefix,
                current_line: 1,
            })
        } else if packed[0] == b'#' {
            let mut input = packed;
            let refs = preceded(decode::header::<()>, rest)
                .parse_next(&mut input)
                .map_err(|_| Error::Header {
                    invalid_first_line: packed.lines().next().unwrap_or(packed).into(),
                })?;
            Ok(packed::Iter {
                cursor: refs,
                prefix,
                current_line: 2,
            })
        } else {
            Ok(packed::Iter {
                cursor: packed,
                prefix,
                current_line: 1,
            })
        }
    }
}

mod error {
    use gix_object::bstr::BString;

    /// The error returned by [`Iter`][super::packed::Iter],
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("The header existed but could not be parsed: {invalid_first_line:?}")]
        Header { invalid_first_line: BString },
        #[error("Invalid reference in line {line_number}: {invalid_line:?}")]
        Reference { invalid_line: BString, line_number: usize },
    }
}

pub use error::Error;
