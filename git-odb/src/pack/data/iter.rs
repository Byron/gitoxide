use crate::pack;
use std::{fs, io};

#[derive(Debug)]
pub struct Iter<R> {
    read: R,
}

impl<R> Iter<R>
where
    R: io::Read,
{
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
    pub fn iter(&self) -> io::Result<impl Iterator<Item = ()>> {
        Ok(Iter::new(io::BufReader::new(fs::File::open(&self.path)?)))
    }
}
