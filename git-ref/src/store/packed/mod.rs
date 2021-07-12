#![allow(missing_docs, dead_code)]

use bstr::BStr;
use git_hash::ObjectId;

#[derive(Debug, PartialEq, Eq)]
enum Peeled {
    Unspecified,
    Partial,
    Fully,
}

/// Information parsed from the header of a packed ref file
#[derive(Debug, PartialEq, Eq)]
struct Header {
    peeled: Peeled,
    sorted: bool,
}

impl Default for Header {
    fn default() -> Self {
        Header {
            peeled: Peeled::Unspecified,
            sorted: false,
        }
    }
}

#[derive(Debug)]
#[non_exhaustive]
struct Reference<'a> {
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

mod decode;

///
pub mod iter {
    mod error {
        use bstr::BString;
        use quick_error::quick_error;

        quick_error! {
            #[derive(Debug)]
            pub enum Error {
                Reference{ invalid_line: BString, line_number: usize } {
                    display("Invalid reference in line {}: '{}'", line_number, invalid_line)
                }
            }
        }
    }
    pub use error::Error;

    struct ForwardIter<'a> {
        cursor: &'a [u8],
        hash: git_hash::Kind,
    }
}
