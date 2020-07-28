use git_features::hash;
use git_object::HashKind;
use std::io;

pub struct HashWrite<T> {
    pub hash: hash::Sha1,
    pub inner: T,
}
impl<T> io::Write for HashWrite<T>
where
    T: io::Write,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.hash.update(buf);
        self.inner.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

impl<T> HashWrite<T>
where
    T: io::Write,
{
    pub fn new(inner: T, kind: HashKind) -> Self {
        match kind {
            HashKind::Sha1 => HashWrite {
                inner,
                hash: hash::Sha1::default(),
            },
        }
    }
}
