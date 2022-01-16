use crate::{extension, extension::Signature, util::from_be_u32};

pub fn header(data: &[u8]) -> (Signature, u32, &[u8]) {
    let (signature, data) = data.split_at(4);
    let (size, data) = data.split_at(4);
    (signature.try_into().unwrap(), from_be_u32(size), data)
}

mod error {
    use quick_error::quick_error;

    use crate::extension;
    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            MandatoryUnimplemented(signature: extension::Signature) {
                display("Encountered mandatory extension '{}' which isn't implemented yet", String::from_utf8_lossy(signature))
            }
            Link(err: extension::link::decode::Error) {
                display("Could not parse mandatory link extension")
                source(err)
                from()
            }
        }
    }
}
pub use error::Error;

pub fn all(maybe_beginning_of_extensions: &[u8], object_hash: git_hash::Kind) -> Result<(Outcome, &[u8]), Error> {
    let mut ext_iter = match extension::Iter::new_without_checksum(maybe_beginning_of_extensions, object_hash) {
        Some(iter) => iter,
        None => return Ok((Outcome::default(), maybe_beginning_of_extensions)),
    };

    let mut ext = Outcome::default();
    for (signature, ext_data) in ext_iter.by_ref() {
        match signature {
            extension::tree::SIGNATURE => {
                ext.tree = extension::tree::decode(ext_data, object_hash);
            }
            extension::resolve_undo::SIGNATURE => {
                ext.resolve_undo = extension::resolve_undo::decode(ext_data, object_hash);
            }
            extension::untracked_cache::SIGNATURE => {
                ext.untracked = extension::untracked_cache::decode(ext_data, object_hash);
            }
            extension::fs_monitor::SIGNATURE => {
                ext.fs_monitor = extension::fs_monitor::decode(ext_data);
            }
            extension::end_of_index_entry::SIGNATURE => {} // skip already done
            extension::index_entry_offset_table::SIGNATURE => {} // not relevant/obtained already
            mandatory if mandatory[0].is_ascii_lowercase() => match mandatory {
                extension::link::SIGNATURE => ext.link = extension::link::decode(ext_data, object_hash)?.into(),
                extension::sparse::SIGNATURE => {
                    if !ext_data.is_empty() {
                        // only used as a marker, if this changes we need this implementation.
                        return Err(Error::MandatoryUnimplemented(mandatory));
                    }
                    ext.is_sparse = true
                }
                unknown => return Err(Error::MandatoryUnimplemented(unknown)),
            },
            _unknown => {} // skip unknown extensions, too
        }
    }
    Ok((ext, &maybe_beginning_of_extensions[ext_iter.consumed..]))
}

#[derive(Default)]
pub struct Outcome {
    pub tree: Option<extension::Tree>,
    pub link: Option<extension::Link>,
    pub resolve_undo: Option<extension::resolve_undo::Paths>,
    pub untracked: Option<extension::UntrackedCache>,
    pub fs_monitor: Option<extension::FsMonitor>,
    pub is_sparse: bool,
}
