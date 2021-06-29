use std::convert::TryFrom;

use crate::{immutable::object, tree};
use bstr::BStr;

/// A directory snapshot containing files (blobs), directories (trees) and submodules (commits).
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Tree<'a> {
    /// The directories and files contained in this tree.
    #[cfg_attr(feature = "serde1", serde(borrow))]
    pub entries: Vec<Entry<'a>>,
}

/// A directory snapshot containing files (blobs), directories (trees) and submodules (commits), lazily evaluated.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct TreeIter<'a> {
    /// The directories and files contained in this tree.
    #[cfg_attr(feature = "serde1", serde(borrow))]
    data: &'a [u8],
}

impl<'a> TreeIter<'a> {
    /// Instantiate an iterator from the given tree data.
    pub fn from_bytes(data: &'a [u8]) -> TreeIter<'a> {
        TreeIter { data }
    }
}

/// An element of a [`Tree`][Tree::entries].
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Entry<'a> {
    /// The kind of object to which `oid` is pointing.
    pub mode: tree::EntryMode,
    /// The name of the file in the parent tree.
    pub filename: &'a BStr,
    /// The id of the object representing the entry.
    #[cfg_attr(feature = "serde1", serde(borrow))]
    pub oid: &'a git_hash::oid,
}

impl<'a> Tree<'a> {
    /// Deserialize a Tree from `data`.
    pub fn from_bytes(data: &'a [u8]) -> Result<Tree<'a>, object::decode::Error> {
        decode::tree(data).map(|(_, t)| t).map_err(object::decode::Error::from)
    }

    /// Create an instance of the empty tree.
    ///
    /// It's particularly useful as static part of a program.
    pub const fn empty() -> Tree<'static> {
        Tree { entries: Vec::new() }
    }
}

impl<'a> TreeIter<'a> {
    /// Consume self and return all parsed entries.
    pub fn entries(self) -> Result<Vec<Entry<'a>>, object::decode::Error> {
        self.collect()
    }
}

impl<'a> Default for TreeIter<'a> {
    fn default() -> Self {
        TreeIter { data: &[] }
    }
}

impl<'a> Iterator for TreeIter<'a> {
    type Item = Result<Entry<'a>, object::decode::Error>;

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

    use crate::{
        immutable::{parse::SPACE, tree::Entry, Tree},
        tree,
    };
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

    const NULL: &[u8] = b"\0";

    pub fn entry<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&[u8], Entry<'_>, E> {
        let (i, mode) = terminated(take_while_m_n(5, 6, is_digit), tag(SPACE))(i)?;
        let mode = tree::EntryMode::try_from(mode)
            .map_err(|invalid| nom::Err::Error(E::from_error_kind(invalid, nom::error::ErrorKind::MapRes)))?;
        let (i, filename) = terminated(take_while1(|b| b != NULL[0]), tag(NULL))(i)?;
        let (i, oid) = take(20u8)(i)?;

        Ok((
            i,
            Entry {
                mode,
                filename: filename.as_bstr(),
                oid: git_hash::oid::try_from(oid).expect("we counted exactly 20 bytes"),
            },
        ))
    }

    pub fn tree<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Tree<'a>, E> {
        let (i, entries) = all_consuming(many0(entry))(i)?;
        Ok((i, Tree { entries }))
    }
}
