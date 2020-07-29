use crate::pack;
use std::io::Read;
use std::{fs, io};

#[derive(Debug)]
pub struct Iter<R> {
    read: R,
}

impl<R> Iter<R>
where
    R: io::Read,
{
    pub fn new_with_header(
        header_data: &[u8; 12],
        read: R,
    ) -> Result<(pack::data::Kind, u32, impl Iterator<Item = ()>), pack::data::parse::Error> {
        let (kind, num_objects) = pack::data::parse::header(header_data)?;
        Ok((kind, num_objects, Iter { read }.take(num_objects as usize)))
    }

    /// `read` must be placed right past the header, and this iterator will fail ungracefully once
    /// it goes past the last object in the pack, i.e. will choke on the trailer if present.
    /// Hence you should only use it with `take(num_objects)`.
    /// Alternatively, use `new_with_header()`
    pub fn new(read: R) -> Self {
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
    pub fn iter(
        &self,
    ) -> io::Result<Result<(pack::data::Kind, u32, impl Iterator<Item = ()>), pack::data::parse::Error>> {
        let mut header = [0u8; 12];
        let mut reader = io::BufReader::new(fs::File::open(&self.path)?);
        reader.read_exact(&mut header)?;
        Ok(Iter::new_with_header(&header, reader))
    }
}
