use crate::{
    hash, pack,
    zlib::stream::{inflate::Inflate, InflateReader},
};
use git_features::hash::Sha1;
use git_object::owned;
use quick_error::quick_error;
use std::{fs, io};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: io::Error) {
            display("An IO operation failed while streaming an entry")
            from()
            source(err)
        }
        PackParse(err: pack::data::parse::Error) {
            display("The pack header could not be parsed")
            from()
            source(err)
        }
        ChecksumMismatch { expected: owned::Id, actual: owned::Id } {
            display("pack checksum in trailer was {}, but actual checksum was {}", expected, actual)
        }
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Entry {
    pub header: pack::data::Header,
    /// amount of bytes used to encode the `header`. `pack_offset + header_size` is the beginning of the compressed data in the pack.
    pub header_size: u16,
    pub pack_offset: u64,
    /// amount bytes consumed while producing `decompressed`
    pub compressed: Vec<u8>,
    /// The decompressed data.
    pub decompressed: Vec<u8>,
    /// Set for the last object in the iteration, providing the hash over all bytes of the iteration
    /// for use as trailer in a pack
    pub trailer: Option<owned::Id>,
}

pub struct Iter<R> {
    read: R,
    decompressor: Option<Inflate>,
    offset: u64,
    had_error: bool,
    kind: pack::data::Kind,
    objects_left: u32,
    hash: Option<Sha1>,
    mode: Mode,
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Mode {
    /// Provide the trailer as read from the pack
    AsIs,
    /// Generate an own hash and trigger an error on the last iterated object
    /// if it does not match the hash provided with the pack
    Verify,
    /// Generate an own hash and if there was an error or the objects are depleted early
    /// due to partial packs, return the last valid entry and with our own hash thus far.
    /// Note that the existing pack hash, if present, will be ignored.
    /// As we won't know which objects fails, every object will have the hash obtained thus far.
    /// This also means that algorithms must know about this possibility, or else might wrongfully
    /// assume the pack is finished.
    Restore,
}

impl<R> Iter<R>
where
    R: io::BufRead,
{
    pub fn kind(&self) -> pack::data::Kind {
        self.kind
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    /// Note that `read` is expected at the beginning of a valid pack file with header and trailer
    /// If `verify` is true, we will assert the SHA1 is actually correct before returning the last entry.
    /// Otherwise bit there is a chance that some kinds of bitrot or inconsistencies will not be detected.
    pub fn new_from_header(mut read: R, trailer: Mode) -> Result<Iter<R>, Error> {
        let mut header_data = [0u8; 12];
        read.read_exact(&mut header_data)?;

        let (kind, num_objects) = pack::data::parse::header(&header_data)?;
        assert_eq!(
            kind,
            pack::data::Kind::V2,
            "let's stop here if we see undocumented pack formats"
        );
        Ok(Iter {
            read,
            decompressor: None,
            offset: 12,
            had_error: false,
            kind,
            objects_left: num_objects,
            hash: if trailer != Mode::AsIs {
                let mut hash = Sha1::default();
                hash.update(&header_data);
                Some(hash)
            } else {
                None
            },
            mode: trailer,
        })
    }

    fn next_inner(&mut self) -> Result<Entry, Error> {
        self.objects_left -= 1; // even an error counts as objects

        // Read header
        let pack::data::Entry {
            header,
            decompressed_size,
            header_size,
            ..
        } = match self.hash.take() {
            Some(hash) => {
                let mut read = PassThrough {
                    read: &mut self.read,
                    write: hash::Write {
                        inner: io::sink(),
                        hash,
                    },
                };
                let res = pack::data::Entry::from_read(&mut read, self.offset);
                self.hash = Some(read.write.hash);
                res
            }
            None => pack::data::Entry::from_read(&mut self.read, self.offset),
        }
        .map_err(Error::from)?;

        // Decompress object to learn it's compressed bytes
        let mut decompressor = self.decompressor.take().unwrap_or_default();
        decompressor.reset();
        let mut reader = InflateReader {
            inner: PassThrough {
                read: &mut self.read,
                write: Vec::with_capacity((decompressed_size / 2) as usize),
            },
            decompressor,
        };

        let mut decompressed = Vec::with_capacity(decompressed_size as usize);
        let bytes_copied = io::copy(&mut reader, &mut decompressed)?;
        assert_eq!(
            bytes_copied, decompressed_size,
            "We should have decompressed {} bytes, but got {} instead",
            decompressed_size, bytes_copied
        );

        let pack_offset = self.offset;
        let compressed_size = reader.decompressor.total_in;
        self.offset += header_size as u64 + compressed_size;
        self.decompressor = Some(reader.decompressor);
        let mut compressed = reader.inner.write;
        compressed.shrink_to_fit();
        assert_eq!(
            compressed_size,
            compressed.len() as u64,
            "we must track exactly the same amount of bytes as read by the decompressor"
        );
        if let Some(hash) = self.hash.as_mut() {
            hash.update(&compressed);
        }

        // Last objects gets trailer (which is potentially verified)
        let trailer = if self.objects_left == 0 {
            let mut id = owned::Id::from([0; 20]);
            if let Err(err) = self.read.read_exact(id.as_mut_slice()) {
                if self.mode != Mode::Restore {
                    return Err(err.into());
                }
            }

            if let Some(hash) = self.hash.take() {
                let actual_id = owned::Id::from(hash.digest());
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
            Some(owned::Id::from(hash.digest()))
        } else {
            None
        };

        Ok(Entry {
            header,
            // TODO: remove this field once we can pack-encode the header above
            header_size: header_size as u16,
            compressed,
            pack_offset,
            decompressed,
            trailer,
        })
    }
}

impl<R> Iterator for Iter<R>
where
    R: io::BufRead,
{
    type Item = Result<Entry, Error>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.objects_left as usize, Some(self.objects_left as usize))
    }

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
}
impl<R> std::iter::ExactSizeIterator for Iter<R> where R: io::BufRead {}

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
    /// Returns an iterator over the pack file itself, without making use of the memory mapping.
    ///
    /// Note that this iterator is costly as no pack index is used, forcing each entry to be decompressed.
    /// If an index is available, use the `traverse(…)` method instead for maximum performance.
    pub fn iter(&self) -> Result<Iter<impl io::BufRead>, Error> {
        let reader = io::BufReader::new(fs::File::open(&self.path)?);
        Iter::new_from_header(reader, Mode::AsIs)
    }
}
