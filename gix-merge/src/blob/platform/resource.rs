use crate::blob::{
    pipeline,
    platform::{Resource, ResourceRef},
};

impl<'a> ResourceRef<'a> {
    pub(super) fn new(cache: &'a Resource) -> Self {
        ResourceRef {
            data: cache.data.map_or(Data::Missing, |data| match data {
                pipeline::Data::Buffer => Data::Buffer(&cache.buffer),
                pipeline::Data::TooLarge { size } => Data::TooLarge { size },
            }),
            rela_path: cache.rela_path.as_ref(),
            id: &cache.id,
        }
    }
}

/// The data of a mergeable resource, as it could be determined and computed previously.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Data<'a> {
    /// The object is missing, either because it didn't exist in the working tree or because its `id` was null.
    /// Such data equals an empty buffer.
    Missing,
    /// The textual data as processed and ready for merging, i.e. suitable for storage in Git.
    Buffer(&'a [u8]),
    /// The file or blob is above the big-file threshold and cannot be processed.
    ///
    /// In this state, the file cannot be merged.
    TooLarge {
        /// The size of the object prior to performing any filtering or as it was found on disk.
        ///
        /// Note that technically, the size isn't always representative of the same 'state' of the
        /// content, as once it can be the size of the blob in Git, and once it's the size of file
        /// in the worktree.
        size: u64,
    },
}

impl<'a> Data<'a> {
    /// Return ourselves as slice of bytes if this instance stores data.
    /// Note that missing data is interpreted as empty slice, to facilitate additions and deletions.
    pub fn as_slice(&self) -> Option<&'a [u8]> {
        match self {
            Data::Buffer(d) => Some(d),
            Data::Missing => Some(&[]),
            Data::TooLarge { .. } => None,
        }
    }
}
