use crate::{
    pack,
    zlib::stream::{inflate::Inflate, InflateReader},
};
use quick_error::quick_error;
use std::{fs, io, io::Seek};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: io::Error) {
            display("An IO operation failed while streaming an entry")
            from()
            source(err)
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
}

pub struct Iter<R> {
    read: R,
    decompressor: Option<Inflate>,
    offset: u64,
    had_error: bool,
}

impl<R> Iter<R>
where
    R: io::BufRead,
{
    /// Note that `read` is expected to start right past the header
    pub fn new_from_header(
        mut read: R,
    ) -> io::Result<Result<(pack::data::Kind, u32, impl Iterator<Item = Result<Entry, Error>>), pack::data::parse::Error>>
    {
        let mut header_data = [0u8; 12];
        read.read_exact(&mut header_data)?;

        Ok(pack::data::parse::header(&header_data).map(|(kind, num_objects)| {
            assert_eq!(
                kind,
                pack::data::Kind::V2,
                "let's stop here if we see undocumented pack formats"
            );
            (
                kind,
                num_objects,
                Iter::new_from_first_entry(read, 12).take(num_objects as usize),
            )
        }))
    }

    /// `read` must be placed right past the header, and this iterator will fail ungracefully once
    /// it goes past the last object in the pack, i.e. will choke on the trailer if present.
    /// Hence you should only use it with `take(num_objects)`.
    /// Alternatively, use `new_from_header()`
    ///
    /// `offset` is the amount of bytes consumed from `read`, usually the size of the header, for use as offset into the pack.
    /// when resolving ref deltas to their absolute pack offset.
    pub fn new_from_first_entry(read: R, offset: u64) -> Self {
        Iter {
            read,
            decompressor: None,
            offset,
            had_error: false,
        }
    }

    fn next_inner(&mut self) -> Result<Entry, Error> {
        let (header, decompressed_size, header_size) =
            pack::data::Header::from_read(&mut self.read, self.offset).map_err(Error::from)?;

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

        Ok(Entry {
            header,
            // TODO: remove this field once we can pack-encode the header above
            header_size: header_size as u16,
            compressed,
            pack_offset,
            decompressed,
        })
    }
}

impl<R> Iterator for Iter<R>
where
    R: io::BufRead,
{
    type Item = Result<Entry, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.had_error {
            return None;
        }
        let result = self.next_inner();
        self.had_error = result.is_err();
        Some(result)
    }
}

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
    R: io::Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.read.read(buf)
    }
}

impl pack::data::File {
    /// Returns an iterator over the pack file itself, without making use of the memory mapping.
    ///
    /// Note that this iterator is costly as no pack index is used, forcing each entry to be decompressed.
    /// If an index is available, use the `traverse(…)` method instead for maximum performance.
    pub fn iter(&self) -> io::Result<impl Iterator<Item = Result<Entry, Error>>> {
        let mut reader = io::BufReader::new(fs::File::open(&self.path)?);
        reader.seek(io::SeekFrom::Current(12))?;
        Ok(Iter::new_from_first_entry(reader, 12).take(self.num_objects as usize))
    }
}
