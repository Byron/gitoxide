use std::io;

/// An implementor of `Write` which passes all input to an inner `Write` in packet line data encoding, one line per `write(…)`
/// call or as many lines as it takes if the data doesn't fit into the maximum allowed line length.
pub struct Writer<T> {
    pub inner: T,
    binary: bool,
}

impl<T: io::Write> Writer<T> {
    pub fn new(inner: T) -> Self {
        Writer { inner, binary: true }
    }
    pub fn text_mode(mut self) -> Self {
        self.binary = false;
        self
    }
    pub fn binary_mode(mut self) -> Self {
        self.binary = true;
        self
    }
}

impl<T: io::Write> io::Write for Writer<T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if buf.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "empty packet lines are not permitted as '0004' is invalid",
            ));
        }

        if self.binary {
            crate::encode::data_to_write(buf, &mut self.inner)
        } else {
            crate::encode::text_to_write(buf, &mut self.inner)
        }
        .map_err(|err| {
            use crate::encode::Error::*;
            match err {
                Io(err) => err,
                DataIsEmpty | DataLengthLimitExceeded(_) => {
                    unreachable!("We are handling empty and large data here, so this can't ever happen")
                }
            }
        })
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}
