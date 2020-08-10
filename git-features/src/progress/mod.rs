use std::io;

pub use prodash::progress::{Discard, DoOrDiscard, Either};
pub use prodash::{unit, Progress};

/// A structure passing every 'read' call through to the contained Progress instance using `inc_by(bytes_read)`.
pub struct Read<R, P> {
    pub reader: R,
    pub progress: P,
}

impl<R, P> io::Read for Read<R, P>
where
    R: io::Read,
    P: Progress,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let bytes_read = self.reader.read(buf)?;
        self.progress.inc_by(bytes_read as usize);
        Ok(bytes_read)
    }
}
