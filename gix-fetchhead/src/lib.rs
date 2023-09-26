#![deny(rust_2018_idioms)]
#![forbid(unsafe_code)]

/// Represents parsed data from the FETCH_HEAD file.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct FetchHead {

    //Head of fetched repository
    pub head: gix_hash::ObjectId,

    //Branch name
    pub branch: &'static str,

    //Remote url
    pub remote: gix_url::Url,
}
