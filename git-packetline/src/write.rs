use crate::{MAX_DATA_LEN, U16_HEX_BYTES};
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
    pub fn set_binary_mode(&mut self) {
        self.binary = true;
    }
    pub fn set_text_mode(&mut self) {
        self.binary = false;
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
    fn write(&mut self, mut buf: &[u8]) -> io::Result<usize> {
        if buf.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "empty packet lines are not permitted as '0004' is invalid",
            ));
        }

        let mut written = 0;
        while !buf.is_empty() {
            let (data, rest) = buf.split_at(buf.len().min(MAX_DATA_LEN));
            written += if self.binary {
                crate::encode::data_to_write(data, &mut self.inner)
            } else {
                crate::encode::text_to_write(data, &mut self.inner)
            }
            .map_err(|err| {
                use crate::encode::Error::*;
                match err {
                    Io(err) => err,
                    DataIsEmpty | DataLengthLimitExceeded(_) => {
                        unreachable!("We are handling empty and large data here, so this can't ever happen")
                    }
                }
            })?;
            // subtract header (and trailng NL) because write-all can't handle writing more than it passes in
            written -= U16_HEX_BYTES + if self.binary { 0 } else { 1 };
            buf = rest;
        }
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}
