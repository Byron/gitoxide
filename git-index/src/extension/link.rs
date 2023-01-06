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

        if let Some(bitmaps) = &self.bitmaps {
            let mut split_entry_count = 0;
            let shared_entries = shared_index.entries_mut();
            let split_entries = split_index.entries();

            bitmaps.replace.for_each_set_bit(|index| {
                let shared_entry = shared_entries
                    .get_mut(index)
                    .expect("index to shared entry exceeds shared index length");
                let split_entry = split_entries
                    .get(split_entry_count)
                    .expect("index to split entry exceeds split index length");

                // TODO: maybe we can move this check to a seperate `verify_link_extension` function,
                // together with the check for entries both marked as replace and remove,
                // called at some point after decoding
                if !split_entry.path.is_empty() {
                    panic!("corrupt link extension, path should be empty")
                }

                shared_entry.stat = split_entry.stat;
                shared_entry.id = split_entry.id;
                shared_entry.flags = split_entry.flags;
                shared_entry.mode = split_entry.mode;

                split_entry_count += 1;
                Some(())
            });

            if split_entries.len() > split_entry_count {
                split_entries[split_entry_count..].iter().for_each(|split_entry| {
                    let mut e = split_entry.clone();
                    let start = shared_index.path_backing.len();
                    e.path = start..start + split_entry.path.len();
                    shared_index.entries.push(e);

                    shared_index
                        .path_backing
                        .extend_from_slice(&split_index.path_backing[split_entry.path.clone()]);
                });
            }

            let mut removed_count = 0;
            bitmaps.delete.for_each_set_bit(|index| {
                if index - removed_count >= shared_index.entries.len() {
                    panic!("index to shared entry exceeds shared index length")
                }
                shared_index.entries.remove(index - removed_count);
                removed_count += 1;
                Some(())
            });

            let mut shared_entries = std::mem::take(&mut shared_index.entries);
            shared_entries.sort_by(|a, b| a.cmp(b, &shared_index.state));

            std::mem::swap(&mut split_index.entries, &mut shared_entries);
            std::mem::swap(&mut split_index.path_backing, &mut shared_index.path_backing);
        }

        Ok(())
    }
}
