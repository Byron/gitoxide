use crate::stream::Entry;
use gix_object::bstr::BStr;
use std::io::{ErrorKind, Read};
use std::path::PathBuf;

/// The source of an additional entry
pub enum Source {
    /// There is no content, typically the case with directories which are always considered empty.
    Null,
    /// Read from the file at the given path.
    Path(PathBuf),
    /// Read from memory.
    Memory(Vec<u8>),
}

impl Source {
    pub(crate) fn len(&self) -> Option<usize> {
        match self {
            Source::Null => Some(0),
            Source::Path(_) => None,
            Source::Memory(buf) => Some(buf.len()),
        }
    }
}

impl std::fmt::Debug for Entry<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Entry")
            .field("path_buf", &self.relative_path())
            .field("mode", &self.mode)
            .field("id", &self.id)
            .field("remaining", &self.remaining)
            .finish()
    }
}

impl Entry<'_> {
    /// Return the path of this entry as slash-separated path relative to the repository.
    pub fn relative_path(&self) -> &BStr {
        self.path_buf.as_ref().expect("always set during our lifetime").as_ref()
    }
}

impl<'a> Drop for Entry<'a> {
    fn drop(&mut self) {
        if self.remaining == Some(0) {
            self.parent.path_buf = self.path_buf.take();
        }
    }
}

impl Entry<'_> {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        if self.parent.pos >= self.parent.filled {
            let mut u16_buf = [0; 2];
            self.parent.read.read_exact(&mut u16_buf)?;
            let nb = u16::from_le_bytes(u16_buf) as usize;

            if nb != 0 {
                self.parent
                    .read
                    .read_exact(&mut self.parent.buf[self.parent.filled..][..nb])?;
            }
            self.parent.filled = nb;
            self.parent.pos = 0;
        }
        Ok(&self.parent.buf[self.parent.pos..self.parent.filled])
    }
}

impl std::io::Read for Entry<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let buf_len = buf.len();
        if let Some(err) = self.parent.err.lock().take() {
            return Err(std::io::Error::new(ErrorKind::Other, err));
        }
        let bytes_read = match self.remaining.as_mut() {
            None => {
                // We expect a zero-read to indicate the end of stream, which is the default way of streams to end.
                // In our case though, it requires sending an extra zero-write, so we avoid that usually.
                let input = self.fill_buf()?;
                let nb = input.len().min(buf.len());
                buf[..nb].copy_from_slice(&input[..nb]);
                self.parent.pos += nb;
                nb
            }
            Some(remaining) => {
                let bytes_read = self.parent.read.read(&mut buf[..buf_len.min(*remaining)])?;
                *remaining -= bytes_read;
                bytes_read
            }
        };
        if bytes_read == 0 {
            self.remaining = Some(0);
        }
        Ok(bytes_read)
    }
}
