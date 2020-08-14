use crate::packet_line::{ERR_PREFIX, FLUSH_LINE, MAX_DATA_LEN};
use quick_error::quick_error;
use std::io;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: io::Error) {
            display("An error occurred while writing")
            from()
            source(err)
        }
        DataLengthLimitExceeded(length_in_bytes: usize) {
            display("Cannot encode more than {} bytes, got {}", MAX_DATA_LEN, length_in_bytes)
        }
        DataIsEmpty {
            display("Empty lines are invalid")
        }
    }
}

pub fn flush_to_write(mut out: impl io::Write) -> io::Result<usize> {
    out.write_all(FLUSH_LINE).map(|_| 4)
}

pub fn error_to_write(data: &[u8], out: impl io::Write) -> Result<usize, Error> {
    let data_with_prefix_end = data.len() + ERR_PREFIX.len();
    if data_with_prefix_end > MAX_DATA_LEN {
        return Err(Error::DataLengthLimitExceeded(data.len() - ERR_PREFIX.len()));
    }
    // This is a big buffer, but it's only used on error, so the program is on the way out
    let mut buf = [0u8; MAX_DATA_LEN];
    buf[..ERR_PREFIX.len()].copy_from_slice(ERR_PREFIX);
    buf[ERR_PREFIX.len()..data_with_prefix_end].copy_from_slice(data);
    data_to_write(&buf[..data_with_prefix_end], out)
}

pub fn data_to_write(data: &[u8], mut out: impl io::Write) -> Result<usize, Error> {
    if data.len() > MAX_DATA_LEN {
        return Err(Error::DataLengthLimitExceeded(data.len()));
    }
    if data.is_empty() {
        return Err(Error::DataIsEmpty);
    }

    let mut buf = [0u8; 4];
    let data_len = data.len() + 4;
    hex::encode_to_slice((data_len as u16).to_be_bytes(), &mut buf).expect("two bytes to 4 hex chars never fails");
    out.write_all(&buf)?;
    out.write_all(data)?;
    Ok(data_len)
}
