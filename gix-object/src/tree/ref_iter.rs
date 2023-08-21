use std::convert::TryFrom;

use bstr::BStr;
use winnow::error::ParserError;

use crate::{tree, tree::EntryRef, TreeRef, TreeRefIter};

impl<'a> TreeRefIter<'a> {
    /// Instantiate an iterator from the given tree data.
    pub fn from_bytes(data: &'a [u8]) -> TreeRefIter<'a> {
        TreeRefIter { data }
    }
}

impl<'a> TreeRef<'a> {
    /// Deserialize a Tree from `data`.
    pub fn from_bytes(mut data: &'a [u8]) -> Result<TreeRef<'a>, crate::decode::Error> {
        decode::tree(&mut data).map_err(crate::decode::Error::with_err)
    }

    /// Find an entry named `name` knowing if the entry is a directory or not, using a binary search.
    ///
    /// Note that it's impossible to binary search by name alone as the sort order is special.
    pub fn bisect_entry(&self, name: &BStr, is_dir: bool) -> Option<EntryRef<'a>> {
        static NULL_HASH: gix_hash::ObjectId = gix_hash::Kind::shortest().null();

        let search = EntryRef {
            mode: if is_dir {
                tree::EntryMode::Tree
            } else {
                tree::EntryMode::Blob
            },
            filename: name,
            oid: &NULL_HASH,
        };
        self.entries
            .binary_search_by(|e| e.cmp(&search))
            .ok()
            .map(|idx| self.entries[idx])
    }

    /// Create an instance of the empty tree.
    ///
    /// It's particularly useful as static part of a program.
    pub const fn empty() -> TreeRef<'static> {
        TreeRef { entries: Vec::new() }
    }
}

impl<'a> TreeRefIter<'a> {
    /// Consume self and return all parsed entries.
    pub fn entries(self) -> Result<Vec<EntryRef<'a>>, crate::decode::Error> {
        self.collect()
    }
}

impl<'a> Iterator for TreeRefIter<'a> {
    type Item = Result<EntryRef<'a>, crate::decode::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() {
            return None;
        }
        match decode::fast_entry(self.data) {
            Some((data_left, entry)) => {
                self.data = data_left;
                Some(Ok(entry))
            }
            None => {
                self.data = &[];
                let empty = &[] as &[u8];
                #[allow(clippy::unit_arg)]
                Some(Err(crate::decode::Error::with_err(
                    winnow::error::ErrMode::from_error_kind(&empty, winnow::error::ErrorKind::Verify),
                )))
            }
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for tree::EntryMode {
    type Error = &'a [u8];

    fn try_from(mode: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(match mode {
            b"40000" => tree::EntryMode::Tree,
            b"100644" => tree::EntryMode::Blob,
            b"100755" => tree::EntryMode::BlobExecutable,
            b"120000" => tree::EntryMode::Link,
            b"160000" => tree::EntryMode::Commit,
            b"100664" => tree::EntryMode::Blob, // rare and found in the linux kernel
            b"100640" => tree::EntryMode::Blob, // rare and found in the Rust repo
            _ => return Err(mode),
        })
    }
}

impl TryFrom<u32> for tree::EntryMode {
    type Error = u32;

    fn try_from(mode: u32) -> Result<Self, Self::Error> {
        Ok(match mode {
            0o40000 => tree::EntryMode::Tree,
            0o100644 => tree::EntryMode::Blob,
            0o100755 => tree::EntryMode::BlobExecutable,
            0o120000 => tree::EntryMode::Link,
            0o160000 => tree::EntryMode::Commit,
            0o100664 => tree::EntryMode::Blob, // rare and found in the linux kernel
            0o100640 => tree::EntryMode::Blob, // rare and found in the Rust repo
            _ => return Err(mode),
        })
    }
}

mod decode {
    use std::convert::TryFrom;

    use bstr::ByteSlice;
    use winnow::{
        combinator::{eof, repeat, terminated},
        error::ParserError,
        prelude::*,
        stream::AsChar,
        token::{take, take_while},
    };

    use crate::{parse::SPACE, tree, tree::EntryRef, TreeRef};

    const NULL: &[u8] = b"\0";

    pub fn fast_entry(i: &[u8]) -> Option<(&[u8], EntryRef<'_>)> {
        let mut mode = 0u32;
        let mut spacer_pos = 1;
        for b in i.iter().take_while(|b| **b != b' ') {
            if *b < b'0' || *b > b'7' {
                return None;
            }
            mode = (mode << 3) + (b - b'0') as u32;
            spacer_pos += 1;
        }
        let (_, i) = i.split_at(spacer_pos);
        let mode = tree::EntryMode::try_from(mode).ok()?;
        let (filename, i) = i.split_at(i.find_byte(0)?);
        let i = &i[1..];
        const HASH_LEN_FIXME: usize = 20; // TODO(SHA256): know actual/desired length or we may overshoot
        let (oid, i) = match i.len() {
            len if len < HASH_LEN_FIXME => return None,
            _ => i.split_at(20),
        };
        Some((
            i,
            EntryRef {
                mode,
                filename: filename.as_bstr(),
                oid: gix_hash::oid::try_from_bytes(oid).expect("we counted exactly 20 bytes"),
            },
        ))
    }

    pub fn entry<'a, E: ParserError<&'a [u8]>>(i: &mut &'a [u8]) -> PResult<EntryRef<'a>, E> {
        (
            terminated(take_while(5..=6, AsChar::is_dec_digit), SPACE)
                .verify_map(|mode| tree::EntryMode::try_from(mode).ok()),
            terminated(take_while(1.., |b| b != NULL[0]), NULL),
            take(20u8), // TODO(SHA256): make this compatible with other hash lengths
        )
            .map(|(mode, filename, oid): (_, &[u8], _)| EntryRef {
                mode,
                filename: filename.as_bstr(),
                oid: gix_hash::oid::try_from_bytes(oid).expect("we counted exactly 20 bytes"),
            })
            .parse_next(i)
    }

    pub fn tree<'a, E: ParserError<&'a [u8]>>(i: &mut &'a [u8]) -> PResult<TreeRef<'a>, E> {
        terminated(repeat(0.., entry), eof)
            .map(|entries| TreeRef { entries })
            .parse_next(i)
    }
}
