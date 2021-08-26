use std::convert::TryFrom;

use crate::{immutable::object, tree, tree::EntryRef, TreeRef, TreeRefIter};

impl<'a> TreeRefIter<'a> {
    /// Instantiate an iterator from the given tree data.
    pub fn from_bytes(data: &'a [u8]) -> TreeRefIter<'a> {
        TreeRefIter { data }
    }
}

impl<'a> TreeRef<'a> {
    /// Deserialize a Tree from `data`.
    pub fn from_bytes(data: &'a [u8]) -> Result<TreeRef<'a>, object::decode::Error> {
        decode::tree(data).map(|(_, t)| t).map_err(object::decode::Error::from)
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
    pub fn entries(self) -> Result<Vec<EntryRef<'a>>, object::decode::Error> {
        self.collect()
    }
}

impl<'a> Default for TreeRefIter<'a> {
    fn default() -> Self {
        TreeRefIter { data: &[] }
    }
}

impl<'a> Iterator for TreeRefIter<'a> {
    type Item = Result<EntryRef<'a>, object::decode::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() {
            return None;
        }
        match decode::entry(self.data) {
            Ok((data_left, entry)) => {
                self.data = data_left;
                Some(Ok(entry))
            }
            Err(err) => {
                self.data = &[];
                Some(Err(err.into()))
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
            b"100664" => tree::EntryMode::Blob, // rare and found in the linux kernel
            b"100640" => tree::EntryMode::Blob, // rare and found in the Rust repo
            b"100755" => tree::EntryMode::BlobExecutable,
            b"120000" => tree::EntryMode::Link,
            b"160000" => tree::EntryMode::Commit,
            _ => return Err(mode),
        })
    }
}

mod decode {
    use std::convert::TryFrom;

    use bstr::ByteSlice;
    use nom::{
        bytes::complete::{tag, take, take_while1, take_while_m_n},
        character::is_digit,
        combinator::all_consuming,
        error::ParseError,
        multi::many0,
        sequence::terminated,
        IResult,
    };

    use crate::{immutable::parse::SPACE, tree, tree::EntryRef, TreeRef};

    const NULL: &[u8] = b"\0";

    pub fn entry<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&[u8], EntryRef<'_>, E> {
        let (i, mode) = terminated(take_while_m_n(5, 6, is_digit), tag(SPACE))(i)?;
        let mode = tree::EntryMode::try_from(mode)
            .map_err(|invalid| nom::Err::Error(E::from_error_kind(invalid, nom::error::ErrorKind::MapRes)))?;
        let (i, filename) = terminated(take_while1(|b| b != NULL[0]), tag(NULL))(i)?;
        let (i, oid) = take(20u8)(i)?;

        Ok((
            i,
            EntryRef {
                mode,
                filename: filename.as_bstr(),
                oid: git_hash::oid::try_from(oid).expect("we counted exactly 20 bytes"),
            },
        ))
    }

    pub fn tree<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], TreeRef<'a>, E> {
        let (i, entries) = all_consuming(many0(entry))(i)?;
        Ok((i, TreeRef { entries }))
    }
}
