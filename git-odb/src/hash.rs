use git_features::hash;
use git_object::HashKind;
use std::io;

pub(crate) struct Write<T> {
    pub hash: hash::Sha1,
    pub inner: T,
}

impl<T> io::Write for Write<T>
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

impl<T> Write<T>
where
    T: io::Write,
{
    pub fn new(inner: T, kind: git_hash::Kind) -> Self {
        match kind {
            HashKind::Sha1 => Write {
                inner,
                hash: hash::Sha1::default(),
            },
        }
    }
}
