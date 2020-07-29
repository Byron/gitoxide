use crate::pack;
use quick_error::quick_error;
use std::{fs, io, io::Seek};

#[derive(Debug)]
pub struct Iter<'a, R> {
    read: R,
    _lifetime: std::marker::PhantomData<&'a ()>,
}

impl<'a, R> Iter<'a, R>
where
    R: io::Read,
{
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
                Iter {
                    read,
                    _lifetime: std::marker::PhantomData,
                }
                .take(num_objects as usize),
            )
        }))
    }

    /// `read` must be placed right past the header, and this iterator will fail ungracefully once
    /// it goes past the last object in the pack, i.e. will choke on the trailer if present.
    /// Hence you should only use it with `take(num_objects)`.
    /// Alternatively, use `new_from_header()`
    pub fn new_from_first_entry(read: R) -> Self {
        Iter {
            read,
            _lifetime: std::marker::PhantomData,
        }
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
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
    pub pack_offset: u64,
    /// The compressed data making up this entry
    #[cfg_attr(feature = "serde1", serde(borrow))]
    pub compressed: &'a [u8],
    /// The decompressed data (stemming from `compressed`)
    pub decompressed: &'a [u8],
}

impl<'a, R> Iterator for Iter<'a, R>
where
    R: io::Read,
{
    type Item = Result<Entry<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!("iter")
    }
}

impl pack::data::File {
    /// Note that this iterator is costly as no pack index is used, forcing each entry to be decompressed.
    /// If an index is available, use the `traverse(…)` method instead for maximum performance.
    pub fn iter(&self) -> io::Result<(pack::data::Kind, u32, impl Iterator<Item = Result<Entry<'_>, Error>>)> {
        let mut reader = io::BufReader::new(fs::File::open(&self.path)?);
        reader.seek(io::SeekFrom::Current(12))?;
        Ok((self.kind, self.num_objects, Iter::new_from_first_entry(reader)))
    }
}
