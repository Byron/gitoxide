use crate::data::delta;
use crate::{data, data::file::decode::Error, data::File};

/// A return value of a resolve function, which given an [`ObjectId`][git_hash::ObjectId] determines where an object can be found.
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum ResolvedBase {
    /// Indicate an object is within this pack, at the given entry, and thus can be looked up locally.
    InPack(data::Entry),
    /// Indicates the object of `kind` was found outside of the pack.
    OutOfPack {
        /// The kind of object we found when reading the header of the out-of-pack base.
        kind: git_object::Kind,
        /// The amount of deltas encountered if the object was packed as well.
        num_deltas: Option<u32>,
    },
}

/// Additional information and statistics about a successfully decoded object produced by [`File::decode_header()`].
///
/// Useful to understand the effectiveness of the pack compression or the cost of decompression.
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Outcome {
    /// The kind of resolved object.
    pub kind: git_object::Kind,
    /// The decompressed size of the object.
    pub object_size: u64,
    /// The amount of deltas in the chain of objects that had to be resolved beforehand.
    pub num_deltas: u32,
}

/// Obtain object information quickly.
impl File {
    /// Resolve the object header information starting at `entry`, following the chain of entries as needed.
    ///
    /// The `entry` determines which object to decode, and is commonly obtained with the help of a pack index file or through pack iteration.
    ///
    /// `resolve` is a function to lookup objects with the given [`ObjectId`][git_hash::ObjectId], in case the full object id
    /// is used to refer to a base object, instead of an in-pack offset.
    pub fn decode_header(
        &self,
        mut entry: data::Entry,
        resolve: impl Fn(&git_hash::oid) -> Option<ResolvedBase>,
    ) -> Result<Outcome, Error> {
        use crate::data::entry::Header::*;
        let mut num_deltas = 0;
        let mut first_delta_decompressed_size = None::<u64>;
        loop {
            match entry.header {
                Tree | Blob | Commit | Tag => {
                    return Ok(Outcome {
                        kind: entry.header.as_kind().expect("always valid for non-refs"),
                        object_size: first_delta_decompressed_size.unwrap_or(entry.decompressed_size),
                        num_deltas,
                    });
                }
                OfsDelta { base_distance } => {
                    num_deltas += 1;
                    if first_delta_decompressed_size.is_none() {
                        first_delta_decompressed_size = Some(self.decode_delta_object_size(&entry)?);
                    }
                    entry = self.entry(entry.base_pack_offset(base_distance))
                }
                RefDelta { base_id } => {
                    num_deltas += 1;
                    if first_delta_decompressed_size.is_none() {
                        first_delta_decompressed_size = Some(self.decode_delta_object_size(&entry)?);
                    }
                    match resolve(base_id.as_ref()) {
                        Some(ResolvedBase::InPack(base_entry)) => entry = base_entry,
                        Some(ResolvedBase::OutOfPack {
                            kind,
                            num_deltas: origin_num_deltas,
                        }) => {
                            return Ok(Outcome {
                                kind,
                                object_size: first_delta_decompressed_size.unwrap_or(entry.decompressed_size),
                                num_deltas: origin_num_deltas.unwrap_or_default() + num_deltas,
                            })
                        }
                        None => return Err(Error::DeltaBaseUnresolved(base_id)),
                    }
                }
            };
        }
    }

    #[inline]
    fn decode_delta_object_size(&self, entry: &data::Entry) -> Result<u64, Error> {
        let mut buf = [0_u8; 32];
        let used = self.decompress_entry_from_data_offset_2(entry.data_offset, &mut buf)?.1;
        let buf = &buf[..used];
        let (_base_size, offset) = delta::decode_header_size(buf);
        let (result_size, _offset) = delta::decode_header_size(&buf[offset..]);
        Ok(result_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_of_decode_entry_outcome() {
        assert_eq!(
            std::mem::size_of::<Outcome>(),
            16,
            "this shouldn't change without use noticing as it's returned a lot"
        );
    }
}
