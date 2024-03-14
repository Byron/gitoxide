use bstr::BStr;
use winnow::{error::ParserError, prelude::*};

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
        let input = &mut data;
        match decode::tree.parse_next(input) {
            Ok(tag) => Ok(tag),
            Err(err) => Err(crate::decode::Error::with_err(err, input)),
        }
    }

    /// Find an entry named `name` knowing if the entry is a directory or not, using a binary search.
    ///
    /// Note that it's impossible to binary search by name alone as the sort order is special.
    pub fn bisect_entry(&self, name: &BStr, is_dir: bool) -> Option<EntryRef<'a>> {
        static NULL_HASH: gix_hash::ObjectId = gix_hash::Kind::shortest().null();

        let search = EntryRef {
            mode: if is_dir {
                tree::EntryKind::Tree
            } else {
                tree::EntryKind::Blob
            }
            .into(),
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
                let failing = self.data;
                self.data = &[];
                #[allow(clippy::unit_arg)]
                Some(Err(crate::decode::Error::with_err(
                    winnow::error::ErrMode::from_error_kind(&failing, winnow::error::ErrorKind::Verify),
                    failing,
                )))
            }
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for tree::EntryMode {
    type Error = &'a [u8];

    fn try_from(mode: &'a [u8]) -> Result<Self, Self::Error> {
        mode_from_decimal(mode)
            .map(|(mode, _rest)| tree::EntryMode(mode as u16))
            .ok_or(mode)
    }
}

fn mode_from_decimal(i: &[u8]) -> Option<(u32, &[u8])> {
    let mut mode = 0u32;
    let mut spacer_pos = 1;
    for b in i.iter().take_while(|b| **b != b' ') {
        if *b < b'0' || *b > b'7' {
            return None;
        }
        mode = (mode << 3) + (b - b'0') as u32;
        spacer_pos += 1;
    }
    if i.len() < spacer_pos {
        return None;
    }
    let (_, i) = i.split_at(spacer_pos);
    Some((mode, i))
}

impl TryFrom<u32> for tree::EntryMode {
    type Error = u32;

    fn try_from(mode: u32) -> Result<Self, Self::Error> {
        Ok(match mode {
            0o40000 | 0o120000 | 0o160000 => tree::EntryMode(mode as u16),
            blob_mode if blob_mode & 0o100000 == 0o100000 => tree::EntryMode(mode as u16),
            _ => return Err(mode),
        })
    }
}

mod decode {
    use bstr::ByteSlice;
    use winnow::{error::ParserError, prelude::*};

    use crate::{
        tree,
        tree::{ref_iter::mode_from_decimal, EntryRef},
        TreeRef,
    };

    pub fn fast_entry(i: &[u8]) -> Option<(&[u8], EntryRef<'_>)> {
        let (mode, i) = mode_from_decimal(i)?;
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

    pub fn tree<'a, E: ParserError<&'a [u8]>>(i: &mut &'a [u8]) -> PResult<TreeRef<'a>, E> {
        let mut out = Vec::new();
        let mut i = &**i;
        while !i.is_empty() {
            let Some((rest, entry)) = fast_entry(i) else {
                #[allow(clippy::unit_arg)]
                return Err(winnow::error::ErrMode::from_error_kind(
                    &i,
                    winnow::error::ErrorKind::Verify,
                ));
            };
            i = rest;
            out.push(entry);
        }
        Ok(TreeRef { entries: out })
    }
}
