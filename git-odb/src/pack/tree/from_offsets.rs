use crate::{pack, pack::index::access::PackOffset, pack::tree::Tree};
use git_features::{
    interrupt::is_triggered,
    progress::{self, Progress},
};
use std::{
    fs, io,
    io::{BufRead, Read},
    time::Instant,
};

/// Returned by [`Tree::from_offsets_in_pack()`]
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error("{message}")]
    Io { source: io::Error, message: &'static str },
    #[error(transparent)]
    Header(#[from] pack::data::parse::Error),
    #[error("Could find object with id {id} in this pack. Thin packs are not supported")]
    UnresolvedRefDelta { id: git_hash::Id },
    #[error(transparent)]
    Tree(#[from] pack::tree::Error),
    #[error("Interrupted")]
    Interrupted,
}

const PACK_HEADER_LEN: usize = 12;

/// Generate tree from certain input
impl<T> Tree<T> {
    /// Create a new `Tree` from any data sorted by offset, ascending as returned by the `data_sorted_by_offsets` iterator.
    /// * `get_pack_offset(item: &T`) -> PackOffset` is a function returning the pack offset of the given item, which can be used
    /// for obtaining the objects entry within the pack.
    /// * `pack_path` is the path to the pack file itself and from which to read the entry data, which is a pack file matching the offsets
    /// returned by `get_pack_offset(…)`.
    /// * `progress` is used to track progress when creating the tree.
    /// * `resolve_in_pack_id(git_hash::borrowed::Id) -> Option<PackOffset>` takes an object ID and tries to resolve it to an object within this pack if
    /// possible. Failing to do so aborts the operation, and this function is not expected to be called in usual packs. It's a theoretical
    /// possibility though.
    ///
    /// Note that the sort order is ascending. The given pack file path must match the provided offsets.
    pub fn from_offsets_in_pack(
        data_sorted_by_offsets: impl Iterator<Item = T>,
        get_pack_offset: impl Fn(&T) -> PackOffset,
        pack_path: impl AsRef<std::path::Path>,
        mut progress: impl Progress,
        resolve_in_pack_id: impl Fn(git_hash::borrowed::Id<'_>) -> Option<PackOffset>,
    ) -> Result<Self, Error> {
        let mut r = io::BufReader::with_capacity(
            8192 * 8, // this value directly corresponds to performance, 8k (default) is about 4x slower than 64k
            fs::File::open(pack_path).map_err(|err| Error::Io {
                source: err,
                message: "open pack path",
            })?,
        );

        let anticpiated_num_objects = if let Some(num_objects) = data_sorted_by_offsets.size_hint().1 {
            progress.init(Some(num_objects), progress::count("objects"));
            num_objects
        } else {
            0
        };
        let mut tree = Tree::with_capacity(anticpiated_num_objects)?;

        {
            // safety check - assure ourselves it's a pack we can handle
            let mut buf = [0u8; PACK_HEADER_LEN];
            r.read_exact(&mut buf).map_err(|err| Error::Io {
                source: err,
                message: "reading header buffer with at least 12 bytes failed - pack file truncated?",
            })?;
            pack::data::parse::header(&buf)?;
        }

        let then = Instant::now();

        let mut previous_cursor_position = None::<u64>;

        for (idx, data) in data_sorted_by_offsets.enumerate() {
            let pack_offset = get_pack_offset(&data);
            if let Some(previous_offset) = previous_cursor_position {
                Self::advance_cursor_to_pack_offset(&mut r, pack_offset, previous_offset)?;
            };
            let entry = pack::data::Entry::from_read(&mut r, pack_offset).map_err(|err| Error::Io {
                source: err,
                message: "EOF while parsing header",
            })?;
            previous_cursor_position = Some(pack_offset + entry.header_size() as u64);

            use pack::data::Header::*;
            match entry.header {
                Tree | Blob | Commit | Tag => {
                    tree.add_root(pack_offset, data)?;
                }
                RefDelta { base_id } => {
                    resolve_in_pack_id(base_id.to_borrowed())
                        .ok_or(Error::UnresolvedRefDelta { id: base_id })
                        .and_then(|base_pack_offset| {
                            tree.add_child(base_pack_offset, pack_offset, data).map_err(Into::into)
                        })?;
                }
                OfsDelta { base_distance } => {
                    let base_pack_offset = pack_offset
                        .checked_sub(base_distance)
                        .expect("in bound distance for deltas");
                    tree.add_child(base_pack_offset, pack_offset, data)?;
                }
            };
            progress.inc();
            if idx % 10_000 == 0 && is_triggered() {
                return Err(Error::Interrupted);
            }
        }

        progress.show_throughput(then);
        Ok(tree)
    }

    fn advance_cursor_to_pack_offset(
        r: &mut io::BufReader<fs::File>,
        pack_offset: u64,
        previous_offset: u64,
    ) -> Result<(), Error> {
        let mut bytes_to_skip = pack_offset
            .checked_sub(previous_offset)
            .expect("continuously ascending pack offets") as usize;
        while bytes_to_skip != 0 {
            let buf = r.fill_buf().map_err(|err| Error::Io {
                source: err,
                message: "skip bytes",
            })?;
            if buf.is_empty() {
                // This means we have reached the end of file and can't make progress anymore, before we have satisfied our need
                // for more
                return Err(Error::Io {
                    source: io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        "ran out of bytes before reading desired amount of bytes",
                    ),
                    message: "index file is damaged or corrupt",
                });
            }
            let bytes = buf.len().min(bytes_to_skip);
            r.consume(bytes);
            bytes_to_skip -= bytes;
        }
        Ok(())
    }
}
