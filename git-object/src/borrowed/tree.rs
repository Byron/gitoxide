use crate::{borrowed, borrowed::parse::SPACE, borrowed::Error, TreeMode};
use bstr::{BStr, ByteSlice};
use nom::{
    bytes::complete::{tag, take, take_while1, take_while_m_n},
    character::is_digit,
    combinator::all_consuming,
    multi::many1,
    sequence::terminated,
    IResult,
};
use std::convert::TryFrom;

/// A directory recording contained files (blobs), directories (trees) and submodules (commits).
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Tree<'a> {
    /// The directories and files contained in this tree.
    #[cfg_attr(feature = "serde1", serde(borrow))]
    pub entries: Vec<Entry<'a>>,
}

/// An element of a [`Tree`][Tree::entries]
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Entry<'a> {
    /// The kind of object to which `oid` is pointing
    pub mode: TreeMode,
    /// The name of the file in the parent tree.
    pub filename: &'a BStr,
    /// The id of the object representing the entry.
    #[cfg_attr(feature = "serde1", serde(borrow))]
    pub oid: borrowed::Id<'a>,
}

impl<'a> Tree<'a> {
    /// Deserialize a Tree from `data`
    pub fn from_bytes(data: &'a [u8]) -> Result<Tree<'a>, Error> {
        parse(data).map(|(_, t)| t).map_err(Error::from)
    }
}

impl TryFrom<&[u8]> for TreeMode {
    type Error = Error;

    fn try_from(mode: &[u8]) -> Result<Self, Self::Error> {
        Ok(match mode {
            b"40000" => TreeMode::Tree,
            b"100644" => TreeMode::Blob,
            b"100664" => TreeMode::Blob, // rare and found in the linux kernel
            b"100640" => TreeMode::Blob, // rare and found in the Rust repo
            b"100755" => TreeMode::BlobExecutable,
            b"120000" => TreeMode::Link,
            b"160000" => TreeMode::Commit,
            _ => return Err(Error::NomDetail(mode.into(), "unknown tree mode")),
        })
    }
}

const NULL: &[u8] = b"\0";
fn parse_entry(i: &[u8]) -> IResult<&[u8], Entry<'_>, Error> {
    let (i, mode) = terminated(take_while_m_n(5, 6, is_digit), tag(SPACE))(i)?;
    let mode = TreeMode::try_from(mode).map_err(nom::Err::Error)?;
    let (i, filename) = terminated(take_while1(|b| b != NULL[0]), tag(NULL))(i)?;
    let (i, oid) = take(20u8)(i)?;

    Ok((
        i,
        Entry {
            mode,
            filename: filename.as_bstr(),
            oid: borrowed::Id::try_from(oid).expect("we counted exactly 20 bytes"),
        },
    ))
}

fn parse(i: &[u8]) -> IResult<&[u8], Tree<'_>, Error> {
    let (i, entries) = all_consuming(many1(parse_entry))(i)?;
    Ok((i, Tree { entries }))
}
