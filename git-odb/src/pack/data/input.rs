use crate::{hash, pack, zlib::stream::inflate::ReadBoxed};
use flate2::Decompress;
use git_features::hash::Sha1;
use std::{fs, io};

/// Returned by [`EntriesFromBytesIter::new_from_header()`] and as part of `Item` of [`EntriesFromBytesIter`]
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error("An IO operation failed while streaming an entry")]
    Io(#[from] io::Error),
    #[error(transparent)]
    PackParse(#[from] pack::data::header::decode::Error),
    #[error("pack checksum in trailer was {expected}, but actual checksum was {actual}")]
    ChecksumMismatch {
        expected: git_hash::ObjectId,
        actual: git_hash::ObjectId,
    },
    #[error("pack is incomplete: it was decompressed into {actual} bytes but {expected} bytes where expected.")]
    IncompletePack { actual: u64, expected: u64 },
}

/// An item of the iteration produced by [`EntriesFromBytesIter`]
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Entry {
    /// The header of a pack entry
    pub header: pack::data::entry::Header,
    /// The amount of bytes used to encode the `header`. `pack_offset + header_size` is the beginning of
    /// the compressed data in the pack.
    pub header_size: u16,
    /// The first byte of the entry at which the `header` can be read.
    pub pack_offset: u64,
    /// The bytes consumed while producing `decompressed`
    /// These do not contain the header, which makes it possible to easily replace a RefDelta with offset deltas
    /// when resolving thin packs.
    /// Depends on `CompressionMode` when the iterator is initialized.
    pub compressed: Option<Vec<u8>>,
    /// The amount of bytes the compressed portion of the entry takes, i.e. the portion behind behind the header.
    pub compressed_size: u64,
    /// The CRC32 over the complete entry, that is encoded header and compressed object data.
    /// Depends on `CompressionMode` when the iterator is initialized
    pub crc32: Option<u32>,
    /// The amount of decompressed bytes of the entry.
    pub decompressed_size: u64,
    /// Set for the last object in the iteration, providing the hash over all bytes of the iteration
    /// for use as trailer in a pack or to verify it matches the trailer.
    pub trailer: Option<git_hash::ObjectId>,
}

/// An iterator over [`Entries`][Entry] in a byte stream.
///
/// The iterator used as part of [Bundle::write_stream_to_directory(…)][pack::Bundle::write_stream_to_directory()].
pub struct EntriesFromBytesIter<R> {
    read: R,
    decompressor: Option<Box<Decompress>>,
    offset: u64,
    had_error: bool,
    kind: pack::data::Version,
    objects_left: u32,
    hash: Option<Sha1>,
    mode: Mode,
    compressed: CompressedBytesMode,
    compressed_buf: Option<Vec<u8>>,
}

/// Iteration Mode
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Mode {
    /// Provide the trailer as read from the pack
    AsIs,
    /// Generate an own hash and trigger an error on the last iterated object
    /// if it does not match the hash provided with the pack.
    ///
    /// This way the one iterating the data cannot miss corruption as long as
    /// the iteration is continued through to the end.
    Verify,
    /// Generate an own hash and if there was an error or the objects are depleted early
    /// due to partial packs, return the last valid entry and with our own hash thus far.
    /// Note that the existing pack hash, if present, will be ignored.
    /// As we won't know which objects fails, every object will have the hash obtained thus far.
    /// This also means that algorithms must know about this possibility, or else might wrongfully
    /// assume the pack is finished.
    Restore,
}

/// Define what to do with the compressed bytes portion of a pack [`Entry`]
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum CompressedBytesMode {
    /// Do nothing with the compressed bytes we read
    Ignore,
    /// Only create a CRC32 of the entry, otherwise similar to `Ignore`
    Crc32,
    /// Keep them and pass them along in a newly allocated buffer
    Keep,
    /// As above, but also compute a CRC32
    KeepAndCrc32,
}

impl CompressedBytesMode {
    /// Returns true if a crc32 should be computed
    pub fn crc32(&self) -> bool {
        match self {
            CompressedBytesMode::KeepAndCrc32 | CompressedBytesMode::Crc32 => true,
            CompressedBytesMode::Keep | CompressedBytesMode::Ignore => false,
        }
    }
    /// Returns true if compressed bytes should be kept
    pub fn keep(&self) -> bool {
        match self {
            CompressedBytesMode::Keep | CompressedBytesMode::KeepAndCrc32 => true,
            CompressedBytesMode::Ignore | CompressedBytesMode::Crc32 => false,
        }
    }
}

impl<R> EntriesFromBytesIter<R>
where
    R: io::BufRead,
{
    /// The pack version currently being iterated
    pub fn kind(&self) -> pack::data::Version {
        self.kind
    }

    /// The kind of iteration
    pub fn mode(&self) -> Mode {
        self.mode
    }

    /// Obtain an iterator from a `read` stream to a pack data file and configure it using `mode` and `compressed`.
    ///
    /// Note that `read` is expected at the beginning of a valid pack data file with a header, entries and a trailer.
    pub fn new_from_header(
        mut read: R,
        mode: Mode,
        compressed: CompressedBytesMode,
    ) -> Result<EntriesFromBytesIter<R>, Error> {
        let mut header_data = [0u8; 12];
        read.read_exact(&mut header_data)?;

        let (kind, num_objects) = pack::data::header::decode(&header_data)?;
        assert_eq!(
            kind,
            pack::data::Version::V2,
            "let's stop here if we see undocumented pack formats"
        );
        Ok(EntriesFromBytesIter {
            read,
            decompressor: None,
            compressed,
            offset: 12,
            had_error: false,
            kind,
            objects_left: num_objects,
            hash: if mode != Mode::AsIs {
                let mut hash = Sha1::default();
                hash.update(&header_data);
                Some(hash)
            } else {
                None
            },
            mode,
            compressed_buf: None,
        })
    }

    fn next_inner(&mut self) -> Result<Entry, Error> {
        self.objects_left -= 1; // even an error counts as objects

        // Read header
        let entry = match self.hash.take() {
            Some(hash) => {
                let mut read = read_and_pass_to(
                    &mut self.read,
                    hash::Write {
                        inner: io::sink(),
                        hash,
                    },
                );
                let res = pack::data::Entry::from_read(&mut read, self.offset);
                self.hash = Some(read.write.hash);
                res
            }
            None => pack::data::Entry::from_read(&mut self.read, self.offset),
        }
        .map_err(Error::from)?;

        // Decompress object to learn it's compressed bytes
        let mut decompressor = self
            .decompressor
            .take()
            .unwrap_or_else(|| Box::new(Decompress::new(true)));
        let compressed_buf = self.compressed_buf.take().unwrap_or_else(|| Vec::with_capacity(4096));
        decompressor.reset(true);
        let mut decompressed_reader = ReadBoxed {
            inner: read_and_pass_to(
                &mut self.read,
                if self.compressed.keep() {
                    Vec::with_capacity(entry.decompressed_size as usize)
                } else {
                    compressed_buf
                },
            ),
            decompressor,
        };

        let bytes_copied = io::copy(&mut decompressed_reader, &mut io::sink())?;
        if bytes_copied != entry.decompressed_size {
            return Err(Error::IncompletePack {
                actual: bytes_copied,
                expected: entry.decompressed_size,
            });
        }

        let pack_offset = self.offset;
        let compressed_size = decompressed_reader.decompressor.total_in();
        self.offset += entry.header_size() as u64 + compressed_size;
        self.decompressor = Some(decompressed_reader.decompressor);

        let mut compressed = decompressed_reader.inner.write;
        debug_assert_eq!(
            compressed_size,
            compressed.len() as u64,
            "we must track exactly the same amount of bytes as read by the decompressor"
        );
        if let Some(hash) = self.hash.as_mut() {
            hash.update(&compressed);
        }

        let crc32 = if self.compressed.crc32() {
            let mut header_buf = [0u8; 32];
            let header_len = entry.header.to_write(bytes_copied, header_buf.as_mut())?;
            let state = git_features::hash::crc32_update(0, &header_buf[..header_len]);
            Some(git_features::hash::crc32_update(state, &compressed))
        } else {
            None
        };

        let compressed = if self.compressed.keep() {
            Some(compressed)
        } else {
            compressed.clear();
            self.compressed_buf = Some(compressed);
            None
        };

        // Last objects gets trailer (which is potentially verified)
        let trailer = if self.objects_left == 0 {
            let mut id = git_hash::ObjectId::from([0; 20]);
            if let Err(err) = self.read.read_exact(id.as_mut_slice()) {
                if self.mode != Mode::Restore {
                    return Err(err.into());
                }
            }

            if let Some(hash) = self.hash.take() {
                let actual_id = git_hash::ObjectId::from(hash.digest());
                if self.mode == Mode::Restore {
                    id = actual_id;
                }
                if id != actual_id {
                    return Err(Error::ChecksumMismatch {
                        actual: actual_id,
                        expected: id,
                    });
                }
            }
            Some(id)
        } else if self.mode == Mode::Restore {
            let hash = self.hash.clone().expect("in restore mode a hash is set");
            Some(git_hash::ObjectId::from(hash.digest()))
        } else {
            None
        };

        Ok(Entry {
            header: entry.header,
            header_size: entry.header_size() as u16,
            compressed,
            compressed_size,
            crc32,
            pack_offset,
            decompressed_size: bytes_copied,
            trailer,
        })
    }
}

fn read_and_pass_to<R: io::Read, W: io::Write>(read: &mut R, to: W) -> PassThrough<&mut R, W> {
    PassThrough { read, write: to }
}

impl<R> Iterator for EntriesFromBytesIter<R>
where
    R: io::BufRead,
{
    type Item = Result<Entry, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.had_error || self.objects_left == 0 {
            return None;
        }
        let result = self.next_inner();
        self.had_error = result.is_err();
        if self.had_error {
            self.objects_left = 0;
        }
        if self.mode == Mode::Restore && self.had_error {
            None
        } else {
            Some(result)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.objects_left as usize, Some(self.objects_left as usize))
    }
}
impl<R> std::iter::ExactSizeIterator for EntriesFromBytesIter<R> where R: io::BufRead {}

struct PassThrough<R, W> {
    read: R,
    write: W,
}

impl<R, W> io::BufRead for PassThrough<R, W>
where
    Self: io::Read,
    R: io::BufRead,
    W: io::Write,
{
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.read.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        let buf = self
            .read
            .fill_buf()
            .expect("never fail as we called fill-buf before and this does nothing");
        self.write
            .write_all(&buf[..amt])
            .expect("a write to never fail - should be a memory buffer");
        self.read.consume(amt)
    }
}

impl<R, W> io::Read for PassThrough<R, W>
where
    W: io::Write,
    R: io::Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let bytes_read = self.read.read(buf)?;
        self.write.write_all(&buf[..bytes_read])?;
        Ok(bytes_read)
    }
}

impl pack::data::File {
    /// Returns an iterator over [`Entries`][pack::data::input::Entry], without making use of the memory mapping.
    pub fn streaming_iter(&self) -> Result<EntriesFromBytesIter<impl io::BufRead>, Error> {
        let reader = io::BufReader::with_capacity(4096 * 8, fs::File::open(&self.path)?);
        EntriesFromBytesIter::new_from_header(reader, Mode::Verify, CompressedBytesMode::KeepAndCrc32)
    }
}
