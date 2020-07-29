use crate::pack;
use std::io::Seek;
use std::{fs, io};

#[derive(Debug)]
pub struct Iter<R> {
    read: R,
}

impl<R> Iter<R>
where
    R: io::Read,
{
    // Note that `read` is expected to start right past the header
    pub fn new_from_header(
        mut read: R,
    ) -> io::Result<Result<(pack::data::Kind, u32, impl Iterator<Item = ()>), pack::data::parse::Error>> {
        let mut header_data = [0u8; 12];
        read.read_exact(&mut header_data)?;

        Ok(pack::data::parse::header(&header_data)
            .map(|(kind, num_objects)| (kind, num_objects, Iter { read }.take(num_objects as usize))))
    }

    /// `read` must be placed right past the header, and this iterator will fail ungracefully once
    /// it goes past the last object in the pack, i.e. will choke on the trailer if present.
    /// Hence you should only use it with `take(num_objects)`.
    /// Alternatively, use `new_from_header()`
    pub fn new_from_first_entry(read: R) -> Self {
        Iter { read }
    }
}

impl<R> Iterator for Iter<R>
where
    R: io::Read,
{
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!("iter")
    }
}

impl pack::data::File {
    pub fn iter(&self) -> io::Result<(pack::data::Kind, u32, impl Iterator<Item = ()>)> {
        let mut reader = io::BufReader::new(fs::File::open(&self.path)?);
        reader.seek(io::SeekFrom::Current(12))?;
        Ok((self.kind, self.num_objects, Iter::new_from_first_entry(reader)))
    }
}
