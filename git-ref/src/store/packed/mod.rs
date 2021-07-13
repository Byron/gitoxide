#![allow(missing_docs, dead_code)]

use bstr::BStr;
use git_hash::ObjectId;

#[derive(Debug, PartialEq, Eq)]
enum Peeled {
    Unspecified,
    Partial,
    Fully,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct Reference<'a> {
    /// The unvalidated full name of the reference.
    pub full_name: &'a BStr,
    /// The target object id of the reference, hex encoded.
    pub target: &'a BStr,
    /// The fully peeled object id, hex encoded, that the ref is ultimately pointing to
    /// i.e. when all indirections are removed.
    pub object: Option<&'a BStr>,
}

impl<'a> Reference<'a> {
    /// Decode the target as object
    pub fn target(&self) -> ObjectId {
        git_hash::ObjectId::from_hex(self.target).expect("parser validation")
    }

    /// Decode the object this reference is ultimately pointing to. Note that this is
    /// the [`target()`] if this is not a fully peeled reference like a tag.
    pub fn object(&self) -> ObjectId {
        self.object.map_or_else(
            || self.target(),
            |id| ObjectId::from_hex(id).expect("parser validation"),
        )
    }
}

/// An iterator over references in a packed refs file
pub struct Iter<'a> {
    /// The position at which to parse the next reference
    cursor: &'a [u8],
    /// The next line
    current_line: usize,
}

mod decode;

///
pub mod iter {
    use crate::store::{packed, packed::decode};
    use bstr::ByteSlice;

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
                        .map_or((self.cursor, &[][..]), |pos| self.cursor.split_at(pos));
                    self.cursor = next_cursor;
                    let line_number = self.current_line;
                    self.current_line += 1;

                    Some(Err(Error::Reference {
                        invalid_line: failed_line.into(),
                        line_number,
                    }))
                }
            }
        }
    }

    impl<'a> packed::Iter<'a> {
        pub fn new(packed: &'a [u8]) -> Result<Self, Error> {
            if packed.is_empty() {
                Ok(packed::Iter {
                    cursor: packed,
                    current_line: 0,
                })
            } else if packed[0] == b'#' {
                let (refs, _header) = decode::header::<()>(packed).map_err(|_| Error::Header {
                    invalid_first_line: packed.lines().next().unwrap_or(packed).into(),
                })?;
                Ok(packed::Iter {
                    cursor: refs,
                    current_line: 1,
                })
            } else {
                Ok(packed::Iter {
                    cursor: packed,
                    current_line: 0,
                })
            }
        }
    }

    mod error {
        use bstr::BString;
        use quick_error::quick_error;

        quick_error! {
            #[derive(Debug)]
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
}
