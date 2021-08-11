use crate::data::input;
use git_features::{
    hash,
    hash::Sha1,
    zlib::{stream::inflate::ReadBoxed, Decompress},
};
use std::{fs, io};

/// An iterator over [`Entries`][input::Entry] in a byte stream.
///
/// The iterator used as part of [Bundle::write_to_directory(…)][crate::Bundle::write_to_directory()].
pub struct BytesToEntriesIter<BR> {
    read: BR,
    decompressor: Option<Box<Decompress>>,
    offset: u64,
    had_error: bool,
    kind: crate::data::Version,
    objects_left: u32,
    hash: Option<Sha1>,
    mode: input::Mode,
    compressed: input::EntryDataMode,
    compressed_buf: Option<Vec<u8>>,
}

impl<BR> BytesToEntriesIter<BR>
where
    BR: io::BufRead,
{
    /// The pack version currently being iterated
    pub fn kind(&self) -> crate::data::Version {
        self.kind
    }

    /// The kind of iteration
    pub fn mode(&self) -> input::Mode {
        self.mode
    }

    /// Obtain an iterator from a `read` stream to a pack data file and configure it using `mode` and `compressed`.
    ///
    /// Note that `read` is expected at the beginning of a valid pack data file with a header, entries and a trailer.
    pub fn new_from_header(
        mut read: BR,
        mode: input::Mode,
        compressed: input::EntryDataMode,
    ) -> Result<BytesToEntriesIter<BR>, input::Error> {
        let mut header_data = [0u8; 12];
        read.read_exact(&mut header_data)?;

        let (kind, num_objects) = crate::data::header::decode(&header_data)?;
        assert_eq!(
            kind,
            crate::data::Version::V2,
            "let's stop here if we see undocumented pack formats"
        );
        Ok(BytesToEntriesIter {
            read,
            decompressor: None,
            compressed,
            offset: 12,
            had_error: false,
            kind,
            objects_left: num_objects,
            hash: (mode != input::Mode::AsIs).then(|| {
                let mut hash = Sha1::default();
                hash.update(&header_data);
                hash
            }),
            mode,
            compressed_buf: None,
        })
    }

    fn next_inner(&mut self) -> Result<input::Entry, input::Error> {
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
                let res = crate::data::Entry::from_read(&mut read, self.offset);
                self.hash = Some(read.write.hash);
                res
            }
            None => crate::data::Entry::from_read(&mut self.read, self.offset),
        }
        .map_err(input::Error::from)?;

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
            return Err(input::Error::IncompletePack {
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
            let header_len = entry.header.write_to(bytes_copied, header_buf.as_mut())?;
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
                if self.mode != input::Mode::Restore {
                    return Err(err.into());
                }
            }

            if let Some(hash) = self.hash.take() {
                let actual_id = git_hash::ObjectId::from(hash.digest());
                if self.mode == input::Mode::Restore {
                    id = actual_id;
                }
                if id != actual_id {
                    return Err(input::Error::ChecksumMismatch {
                        actual: actual_id,
                        expected: id,
                    });
                }
            }
            Some(id)
        } else if self.mode == input::Mode::Restore {
            let hash = self.hash.clone().expect("in restore mode a hash is set");
            Some(git_hash::ObjectId::from(hash.digest()))
        } else {
            None
        };

        Ok(input::Entry {
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

impl<R> Iterator for BytesToEntriesIter<R>
where
    R: io::BufRead,
{
    type Item = Result<input::Entry, input::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.had_error || self.objects_left == 0 {
            return None;
        }
        let result = self.next_inner();
        self.had_error = result.is_err();
        if self.had_error {
            self.objects_left = 0;
        }
        if self.mode == input::Mode::Restore && self.had_error {
            None
        } else {
            Some(result)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.objects_left as usize, Some(self.objects_left as usize))
    }
}

impl<R> std::iter::ExactSizeIterator for BytesToEntriesIter<R> where R: io::BufRead {}

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

impl crate::data::File {
    /// Returns an iterator over [`Entries`][crate::data::input::Entry], without making use of the memory mapping.
    pub fn streaming_iter(&self) -> Result<BytesToEntriesIter<impl io::BufRead>, input::Error> {
        let reader = io::BufReader::with_capacity(4096 * 8, fs::File::open(&self.path)?);
        BytesToEntriesIter::new_from_header(reader, input::Mode::Verify, input::EntryDataMode::KeepAndCrc32)
    }
}
