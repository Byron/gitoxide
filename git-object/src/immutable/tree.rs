use std::convert::TryFrom;

use crate::{
    immutable::{object::decode, parse::SPACE},
    tree,
};
use bstr::{BStr, ByteSlice};
use nom::{
    bytes::complete::{tag, take, take_while1, take_while_m_n},
    character::is_digit,
    combinator::all_consuming,
    multi::many1,
    sequence::terminated,
    IResult,
};

/// A directory snapshot containing files (blobs), directories (trees) and submodules (commits).
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Tree<'a> {
    /// The directories and files contained in this tree.
    #[cfg_attr(feature = "serde1", serde(borrow))]
    pub entries: Vec<Entry<'a>>,
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
    pub fn from_bytes(data: &'a [u8]) -> Result<Tree<'a>, decode::Error> {
        parse(data).map(|(_, t)| t).map_err(decode::Error::from)
    }

    /// Create an instance of the empty tree.
    ///
    /// It's particularly useful as static part of a program.
    pub const fn empty() -> Tree<'static> {
        Tree { entries: Vec::new() }
    }
}

impl TryFrom<&[u8]> for tree::EntryMode {
    type Error = decode::Error;

    fn try_from(mode: &[u8]) -> Result<Self, Self::Error> {
        Ok(match mode {
            b"40000" => tree::EntryMode::Tree,
            b"100644" => tree::EntryMode::Blob,
            b"100664" => tree::EntryMode::Blob, // rare and found in the linux kernel
            b"100640" => tree::EntryMode::Blob, // rare and found in the Rust repo
            b"100755" => tree::EntryMode::BlobExecutable,
            b"120000" => tree::EntryMode::Link,
            b"160000" => tree::EntryMode::Commit,
            _ => return Err(decode::Error::NomDetail(mode.into(), "unknown tree mode")),
        })
    }
}

const NULL: &[u8] = b"\0";
fn parse_entry(i: &[u8]) -> IResult<&[u8], Entry<'_>, decode::Error> {
    let (i, mode) = terminated(take_while_m_n(5, 6, is_digit), tag(SPACE))(i)?;
    let mode = tree::EntryMode::try_from(mode).map_err(nom::Err::Error)?;
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

fn parse(i: &[u8]) -> IResult<&[u8], Tree<'_>, decode::Error> {
    let (i, entries) = all_consuming(many1(parse_entry))(i)?;
    Ok((i, Tree { entries }))
}
