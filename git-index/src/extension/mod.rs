use smallvec::SmallVec;

const MIN_SIZE: usize = 4 /* signature */ + 4 /* size */;

pub type Signature = [u8; 4];

pub struct Iter<'a> {
    data: &'a [u8],
    pub consumed: usize,
}

/// A structure to associate object ids of a tree with sections in the index entries list.
///
/// It allows to more quickly build trees by avoiding as it can quickly re-use portions of the index and its associated tree ids
/// if there wa sno change to them. Portions of this tree are invalidated as the index is changed.
pub struct Tree {
    name: SmallVec<[u8; 23]>,
    /// Only set if there are any entries in the index we are associated with.
    id: Option<tree::NodeId>,
    children: Vec<Tree>,
}

pub struct Link {
    pub shared_index_checksum: git_hash::ObjectId,
}

mod iter;

pub(crate) mod decode;

pub(crate) mod tree;

pub(crate) mod end_of_index_entry;

pub(crate) mod index_entry_offset_table;

pub mod link {
    use crate::extension::{Link, Signature};
    use crate::util::split_at_pos;

    pub const SIGNATURE: Signature = *b"link";

    pub mod decode {
        use quick_error::quick_error;

        quick_error! {
            #[derive(Debug)]
            pub enum Error {
                Corrupt(message: &'static str) {
                    display("{}", message)
                }
            }
        }
    }

    pub fn decode(data: &[u8], object_hash: git_hash::Kind) -> Result<Link, decode::Error> {
        let (id, data) = split_at_pos(data, object_hash.len_in_bytes())
            .ok_or(decode::Error::Corrupt(
                "link extension too short to read share index checksum",
            ))
            .map(|(id, d)| (git_hash::ObjectId::from(id), d))?;

        if data.is_empty() {
            return Ok(Link {
                shared_index_checksum: id,
            });
        }
        todo!("decode link bitmaps")
    }
}
