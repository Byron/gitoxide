#![deny(rust_2018_idioms)]
#![forbid(unsafe_code)]

use gix_object::bstr::BString;

/// Represents parsed data from the FETCH_HEAD file.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct FetchHeadEntry {
    //Head of fetched repository
    pub head: gix_hash::ObjectId,

    //Merge status
    pub merge_status: bool,

    //Branch name
    pub branch: &'static str,

    //Remote url
    pub remote: BString,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct FetchHead {
    //List of fetch entries contained in the FETCH_HEAD FILE
    pub entries: Vec<FetchHeadEntry>,
}

pub mod parse;
pub mod write;
