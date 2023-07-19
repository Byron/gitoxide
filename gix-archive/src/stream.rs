use crate::Stream;
use gix_object::bstr::{BStr, BString};
use std::io::{ErrorKind, Read};

/// The error returned by [`next_entry()`][Stream::next_entry()].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Could not find a blob or tree for archival")]
    Find(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),
    #[error("Could not query attributes for path \"{path}\"")]
    Attributes {
        path: BString,
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },
}

/// An entry in a stream. Note that they must be consumed fully, by reading from them till exhaustion.
///
/// ### Drop behaviour
///
/// If the entry is dropped without reading it till exhaustion, the stream is tainted and
/// [`next_entry()`][Stream::next_entry()] will panic next time it is called.
pub struct Entry<'a> {
    /// Access to our parent
    parent: &'a mut Stream,

    /// The path relative to the repository at which data should be written.
    path_buf: Option<BString>,
    /// The amount of bytes left to read
    remaining: usize,
}

impl Entry<'_> {
    /// Return the path of this entry as slash-separated path relative to the repository.
    pub fn relative_path(&self) -> &BStr {
        self.path_buf.as_ref().expect("always set during our lifetime").as_ref()
    }
}

impl<'a> Drop for Entry<'a> {
    fn drop(&mut self) {
        if self.remaining == 0 {
            self.parent.path_buf = self.path_buf.take();
        }
    }
}

impl std::io::Read for Entry<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let buf_len = buf.len();
        if let Some(err) = self.parent.err.lock().take() {
            return Err(std::io::Error::new(ErrorKind::Other, err));
        }
        let bytes_read = self.parent.read.read(&mut buf[..buf_len.min(self.remaining)])?;
        self.remaining -= bytes_read;
        Ok(bytes_read)
    }
}

impl Stream {
    /// Access the next entry of the stream or `None` if there is nothing more to read.
    pub fn next_entry(&mut self) -> Result<Option<Entry<'_>>, Error> {
        assert!(
            self.path_buf.is_some(),
            "BUG: must consume and drop entry before getting the next one"
        );
        let res = self.read_entry_info();
        match res {
            Ok(remaining) => {
                if let Some(err) = self.err.lock().take() {
                    return Err(err);
                }
                Ok(Some(Entry {
                    path_buf: self.path_buf.take(),
                    parent: self,
                    remaining,
                }))
            }
            Err(err) => {
                if err.kind() == ErrorKind::UnexpectedEof {
                    if let Some(err) = self.err.lock().take() {
                        return Err(err);
                    }
                }
                Err(err.into())
            }
        }
    }

    // Format: [usize-LE][usize-LE][relative_path_bytes][object_stream]
    fn read_entry_info(&mut self) -> Result<usize, std::io::Error> {
        let mut buf = [0; std::mem::size_of::<usize>()];

        self.read.read_exact(&mut buf)?;
        let path_len = usize::from_le_bytes(buf);

        self.read.read_exact(&mut buf)?;
        let stream_size = usize::from_le_bytes(buf);

        let path_buf = self.path_buf.as_mut().expect("set while producing an entry");
        clear_and_set_capacity(path_buf, path_len);

        // SAFETY: `clear_and_set_capacity` assures the vec has the right capacity to hold `path_len`
        #[allow(unsafe_code)]
        unsafe {
            path_buf.set_len(path_len);
        }
        self.read.read_exact(path_buf)?;

        Ok(stream_size)
    }
}

fn clear_and_set_capacity(buf: &mut Vec<u8>, cap: usize) {
    buf.clear();
    if buf.capacity() < cap {
        buf.reserve(cap);
        assert!(buf.capacity() >= cap, "{} >= {}", buf.capacity(), cap);
    }
}
