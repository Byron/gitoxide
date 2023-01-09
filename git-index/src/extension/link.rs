use crate::{
    extension::{Link, Signature},
    util::split_at_pos,
};

/// The signature of the link extension.
pub const SIGNATURE: Signature = *b"link";

/// Bitmaps to know which entries to delete or replace, even though details are still unknown.
#[derive(Clone)]
pub struct Bitmaps {
    /// A bitmap to signal which entries to delete, maybe.
    pub delete: git_bitmap::ewah::Vec,
    /// A bitmap to signal which entries to replace, maybe.
    pub replace: git_bitmap::ewah::Vec,
}

#[derive(Clone)]
struct VerifiedBitmaps {
    pub delete: Vec<usize>,
    pub replace: Vec<usize>,
}

///
pub mod decode {

    /// The error returned when decoding link extensions.
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("{0}")]
        Corrupt(&'static str),
        #[error("{kind} bitmap corrupt")]
        BitmapDecode {
            err: git_bitmap::ewah::decode::Error,
            kind: &'static str,
        },
    }
}

pub(crate) fn decode(data: &[u8], object_hash: git_hash::Kind) -> Result<Link, decode::Error> {
    let (id, data) = split_at_pos(data, object_hash.len_in_bytes())
        .ok_or(decode::Error::Corrupt(
            "link extension too short to read share index checksum",
        ))
        .map(|(id, d)| (git_hash::ObjectId::from(id), d))?;

    if data.is_empty() {
        return Ok(Link {
            shared_index_checksum: id,
            bitmaps: None,
        });
    }

    let (delete, data) =
        git_bitmap::ewah::decode(data).map_err(|err| decode::Error::BitmapDecode { kind: "delete", err })?;
    let (replace, data) =
        git_bitmap::ewah::decode(data).map_err(|err| decode::Error::BitmapDecode { kind: "replace", err })?;

    if !data.is_empty() {
        return Err(decode::Error::Corrupt("garbage trailing link extension"));
    }

    Ok(Link {
        shared_index_checksum: id,
        bitmaps: Some(Bitmaps { delete, replace }),
    })
}

impl Link {
    pub(crate) fn dissolve_into(
        self,
        split_index: &mut crate::File,
        object_hash: git_hash::Kind,
        options: crate::decode::Options,
    ) -> Result<(), crate::file::init::Error> {
        let shared_index_path = split_index
            .path
            .parent()
            .expect("split index file in .git folder")
            .join(format!("sharedindex.{}", self.shared_index_checksum));
        let mut shared_index = crate::File::at(
            &shared_index_path,
            object_hash,
            crate::decode::Options {
                expected_checksum: self.shared_index_checksum.into(),
                ..options
            },
        )?;

        if let Some(bitmaps) = self.verify_bitmaps(split_index, &shared_index)? {
            let shared_entries = shared_index.entries_mut();
            let split_entries = split_index.entries();

            bitmaps
                .replace
                .iter()
                .enumerate()
                .for_each(|(split_entry_index, &replace_index)| {
                    let shared_entry = &mut shared_entries[replace_index];
                    let split_entry = &split_entries[split_entry_index];

                    shared_entry.stat = split_entry.stat;
                    shared_entry.id = split_entry.id;
                    shared_entry.flags = split_entry.flags;
                    shared_entry.mode = split_entry.mode;
                });

            if split_entries.len() > bitmaps.replace.len() {
                split_entries[bitmaps.replace.len()..].iter().for_each(|split_entry| {
                    let mut e = split_entry.clone();
                    let start = shared_index.path_backing.len();
                    e.path = start..start + split_entry.path.len();
                    shared_index.entries.push(e);

                    shared_index
                        .path_backing
                        .extend_from_slice(&split_index.path_backing[split_entry.path.clone()]);
                });
            }

            bitmaps.delete.iter().rev().for_each(|&i| {
                shared_index.entries.remove(i);
            });

            let mut shared_entries = std::mem::take(&mut shared_index.entries);
            shared_entries.sort_by(|a, b| a.cmp(b, &shared_index.state));

            std::mem::swap(&mut split_index.entries, &mut shared_entries);
            std::mem::swap(&mut split_index.path_backing, &mut shared_index.path_backing);
        }

        Ok(())
    }

    fn verify_bitmaps(
        &self,
        split_index: &crate::File,
        shared_index: &crate::File,
    ) -> Result<Option<VerifiedBitmaps>, decode::Error> {
        if let Some(bitmaps) = &self.bitmaps {
            let mut replace_bitmap: Vec<usize> = Vec::new();
            let mut delete_bitmap: Vec<usize> = Vec::new();

            bitmaps.replace.for_each_set_bit(|index| {
                replace_bitmap.push(index);
                Some(())
            });
            bitmaps.delete.for_each_set_bit(|index| {
                delete_bitmap.push(index);
                Some(())
            });

            let split_entries = split_index.entries();
            let shared_entries = shared_index.entries();

            if replace_bitmap.len() > split_entries.len() {
                return Err(decode::Error::Corrupt(
                    "replace bitmap length exceeds split index length - more entries in bitmap than found in split index",
                ));
            }

            if let Some(&index) = replace_bitmap.last() {
                if index >= shared_entries.len() {
                    return Err(decode::Error::Corrupt(
                        "replace bitmap length exceeds shared index length - more entries in bitmap than found in shared index",
                    ));
                }
            }

            if let Some(&index) = delete_bitmap.last() {
                if index >= shared_entries.len() {
                    return Err(decode::Error::Corrupt(
                        "delete bitmap length exceeds shared index length - more entries in bitmap than found in shared index",
                    ));
                }
            }

            for (split_entry_index, replace_index) in replace_bitmap.iter().enumerate() {
                if delete_bitmap.iter().any(|delete_index| delete_index == replace_index) {
                    return Err(decode::Error::Corrupt(
                        "replace and delete bitmap point at the same index",
                    ));
                }

                if !split_entries
                    .get(split_entry_index)
                    .expect("already checked")
                    .path
                    .is_empty()
                {
                    return Err(decode::Error::Corrupt("paths in split index entries should be empty"));
                }
            }

            Ok(Some(VerifiedBitmaps {
                replace: replace_bitmap,
                delete: delete_bitmap,
            }))
        } else {
            Ok(None)
        }
    }
}
