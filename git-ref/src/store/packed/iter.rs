use crate::store::{packed, packed::decode};
use bstr::ByteSlice;

/// packed-refs specific functionality
impl packed::Buffer {
    /// Return an iterator of references stored in this packed refs buffer.
    pub fn iter(&self) -> Result<packed::Iter<'_>, packed::iter::Error> {
        packed::Iter::new(self.as_ref())
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
    pub fn new(packed: &'a [u8]) -> Result<Self, Error> {
        if packed.is_empty() {
            Ok(packed::Iter {
                cursor: packed,
                current_line: 1,
            })
        } else if packed[0] == b'#' {
            let (refs, _header) = decode::header::<()>(packed).map_err(|_| Error::Header {
                invalid_first_line: packed.lines().next().unwrap_or(packed).into(),
            })?;
            Ok(packed::Iter {
                cursor: refs,
                current_line: 2,
            })
        } else {
            Ok(packed::Iter {
                cursor: packed,
                current_line: 1,
            })
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
            Header { invalid_first_line: BString } {
                display("The header existed but could not be parsed: '{}'", invalid_first_line)
            }
            Reference { invalid_line: BString, line_number: usize } {
                display("Invalid reference in line {}: '{}'", line_number, invalid_line)
            }
        }
    }
}

pub use error::Error;
