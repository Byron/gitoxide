use crate::{pack, pack::index::access::PackOffset, pack::tree::Tree};
use git_features::progress::{self, Progress};
use quick_error::quick_error;
use std::{
    fs, io,
    io::{BufRead, Read},
    time::Instant,
};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: io::Error, msg: &'static str) {
            display("{}", msg)
            source(err)
        }
        Header(err: pack::data::parse::Error) {
            source(err)
            from()
        }
        UnresolvedRefDelta(id: git_object::owned::Id) {
            display("Could find object with id {} in this pack. Thin packs are not supported", id)
        }
        Tree(err: pack::tree::Error) {
            display("An error occurred when handling the delta tree")
            source(err)
            from()
        }
    }
}

const PACK_HEADER_LEN: usize = 12;

/// Generate tree from certain input
impl<T> Tree<T> {
    /// The sort order is ascending. The given packfile path must match the provided offsets.
    pub fn from_offsets_in_pack(
        data_sorted_by_offsets: impl Iterator<Item = T>,
        get_pack_offset: impl Fn(&T) -> PackOffset,
        pack_path: impl AsRef<std::path::Path>,
        mut progress: impl Progress,
        resolve_in_pack_id: impl Fn(git_object::borrowed::Id) -> Option<PackOffset>,
    ) -> Result<Self, Error> {
        let mut r = io::BufReader::with_capacity(
            8192 * 8, // this value directly corresponds to performance, 8k (default) is about 4x slower than 64k
            fs::File::open(pack_path).map_err(|err| Error::Io(err, "open pack path"))?,
        );

        let anticpiated_num_objects = if let Some(num_objects) = data_sorted_by_offsets.size_hint().1 {
            progress.init(Some(num_objects), Some(progress::count("objects")));
            num_objects
        } else {
            0
        };
        let mut tree = Tree::with_capacity(anticpiated_num_objects)?;

        {
            // safety check - assure ourselves it's a pack we can handle
            let mut buf = [0u8; PACK_HEADER_LEN];
            r.read_exact(&mut buf).map_err(|err| {
                Error::Io(
                    err,
                    "reading header buffer with at least 12 bytes failed - pack file truncated?",
                )
            })?;
            pack::data::parse::header(&buf)?;
        }

        let then = Instant::now();

        let mut previous_cursor_position = None::<u64>;

        let mut num_objects = 0;
        for data in data_sorted_by_offsets {
            let pack_offset = get_pack_offset(&data);
            num_objects += 1;
            if let Some(previous_offset) = previous_cursor_position {
                Self::advance_cursor_to_pack_offset(&mut r, pack_offset, previous_offset)?;
            };
            let entry = pack::data::Entry::from_read(&mut r, pack_offset)
                .map_err(|err| Error::Io(err, "EOF while parsing header"))?;
            previous_cursor_position = Some(pack_offset + entry.header_size() as u64);

            use pack::data::Header::*;
            match entry.header {
                Tree | Blob | Commit | Tag => {
                    tree.add_root(pack_offset, data)?;
                }
                RefDelta { base_id } => {
                    resolve_in_pack_id(base_id.to_borrowed())
                        .ok_or_else(|| Error::UnresolvedRefDelta(base_id))
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
        }

        progress.show_throughput(then, num_objects, "entries");
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
            let buf = r.fill_buf().map_err(|err| Error::Io(err, "skip bytes"))?;
            if buf.is_empty() {
                // This means we have reached the end of file and can't make progress anymore, before we have satisfied our need
                // for more
                return Err(Error::Io(
                    io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        "ran out of bytes before reading desired amount of bytes",
                    ),
                    "index file is damaged or corrupt",
                ));
            }
            let bytes = buf.len().min(bytes_to_skip);
            r.consume(bytes);
            bytes_to_skip -= bytes;
        }
        Ok(())
    }
}
