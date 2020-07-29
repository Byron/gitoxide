use crate::{
    pack,
    zlib::stream::{inflate::Inflate, InflateReader},
};
use quick_error::quick_error;
use std::{fs, io, io::Seek};

pub struct Iter<'a, R> {
    read: R,
    decompressor: Option<Inflate>,
    compressed_bytes: Vec<u8>,
    decompressed_bytes: Vec<u8>,
    offset: u64,
    had_error: bool,
    _item_lifetime: std::marker::PhantomData<&'a ()>,
}

impl<'a, R> Iter<'a, R>
where
    R: io::BufRead,
{
    fn buffers() -> (Vec<u8>, Vec<u8>) {
        let base = 4096;
        (Vec::with_capacity(base * 2), Vec::with_capacity(base * 4))
    }
    // Note that `read` is expected to start right past the header
    pub fn new_from_header(
        mut read: R,
    ) -> io::Result<
        Result<(pack::data::Kind, u32, impl Iterator<Item = Result<Entry<'a>, Error>>), pack::data::parse::Error>,
    > {
        let mut header_data = [0u8; 12];
        read.read_exact(&mut header_data)?;

        Ok(pack::data::parse::header(&header_data).map(|(kind, num_objects)| {
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
        let (compressed_bytes, decompressed_bytes) = Self::buffers();
        Iter {
            read,
            decompressor: None,
            compressed_bytes,
            decompressed_bytes,
            offset,
            had_error: false,
            _item_lifetime: std::marker::PhantomData,
        }
    }

    fn next_inner(&mut self) -> Result<Entry<'a>, Error> {
        let (header, decompressed_size, header_size) =
            pack::data::Header::from_read(&mut self.read, self.offset).map_err(Error::from)?;

        let decompressor = self.decompressor.take().unwrap_or_default();
        let mut reader = InflateReader {
            inner: &mut self.read,
            decompressor,
        };

        self.decompressed_bytes.clear();
        self.decompressed_bytes.resize(decompressed_size as usize, 0);
        let bytes_copied = io::copy(&mut reader, &mut self.decompressed_bytes)?;
        assert_eq!(
            bytes_copied, decompressed_size,
            "We should have decompressed {} bytes, but got {} instead",
            decompressed_size, bytes_copied
        );

        let pack_offset = self.offset;
        self.offset += header_size as u64 + reader.decompressor.total_in;
        self.decompressor = Some(reader.decompressor);

        Ok(Entry {
            header,
            header_size: header_size as u16,
            pack_offset,
            compressed: &[],
            // decompressed: &self.decompressed_bytes[..decompressed_size as usize],
            decompressed: &[],
        })
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: io::Error) {
            display("An IO operation failed while streaming an entry")
            from()
            source(err)
        }
        Zlib(err: crate::zlib::Error) {
            display("The stream could not be decompressed")
            source(err)
            from()
        }
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Entry<'a> {
    pub header: pack::data::Header,
    /// amount of bytes used to encode the `header`. `pack_offset + header_size` is the beginning of the compressed data in the pack.
    pub header_size: u16,
    pub pack_offset: u64,
    /// The compressed data making up this entry
    #[cfg_attr(feature = "serde1", serde(borrow))]
    pub compressed: &'a [u8],
    /// The decompressed data (stemming from `compressed`)
    pub decompressed: &'a [u8],
}

impl<'a, R> Iterator for Iter<'a, R>
where
    R: io::BufRead,
{
    type Item = Result<Entry<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.had_error {
            return None;
        }
        let result = self.next_inner();
        self.had_error = result.is_err();
        Some(result)
    }
}

impl pack::data::File {
    /// Note that this iterator is costly as no pack index is used, forcing each entry to be decompressed.
    /// If an index is available, use the `traverse(…)` method instead for maximum performance.
    pub fn iter(&self) -> io::Result<(pack::data::Kind, u32, impl Iterator<Item = Result<Entry<'_>, Error>>)> {
        let mut reader = io::BufReader::new(fs::File::open(&self.path)?);
        reader.seek(io::SeekFrom::Current(12))?;
        Ok((self.kind, self.num_objects, Iter::new_from_first_entry(reader, 12)))
    }
}
