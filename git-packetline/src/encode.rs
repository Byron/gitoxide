use crate::{Channel, DELIMITER_LINE, ERR_PREFIX, FLUSH_LINE, MAX_DATA_LEN, RESPONSE_END_LINE};
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

pub fn response_end_to_write(mut out: impl io::Write) -> io::Result<usize> {
    out.write_all(RESPONSE_END_LINE).map(|_| 4)
}

pub fn delim_to_write(mut out: impl io::Write) -> io::Result<usize> {
    out.write_all(DELIMITER_LINE).map(|_| 4)
}

pub fn flush_to_write(mut out: impl io::Write) -> io::Result<usize> {
    out.write_all(FLUSH_LINE).map(|_| 4)
}

pub fn error_to_write(data: &[u8], out: impl io::Write) -> Result<usize, Error> {
    prefixed_data_to_write(ERR_PREFIX, data, out)
}

pub fn band_to_write(kind: Channel, data: &[u8], out: impl io::Write) -> Result<usize, Error> {
    prefixed_data_to_write(&[kind as u8], data, out)
}

pub fn data_to_write(data: &[u8], out: impl io::Write) -> Result<usize, Error> {
    prefixed_data_to_write(&[], data, out)
}

pub fn text_to_write(text: &[u8], out: impl io::Write) -> Result<usize, Error> {
    prefixed_and_suffixed_data_to_write(&[], text, &[b'\n'], out)
}

fn prefixed_data_to_write(prefix: &[u8], data: &[u8], out: impl io::Write) -> Result<usize, Error> {
    prefixed_and_suffixed_data_to_write(prefix, data, &[], out)
}

fn prefixed_and_suffixed_data_to_write(
    prefix: &[u8],
    data: &[u8],
    suffix: &[u8],
    mut out: impl io::Write,
) -> Result<usize, Error> {
    let data_len = prefix.len() + data.len() + suffix.len();
    if data_len > MAX_DATA_LEN {
        return Err(Error::DataLengthLimitExceeded(data_len));
    }
    if data.is_empty() {
        return Err(Error::DataIsEmpty);
    }

    let data_len = data_len + 4;
    let buf = u16_to_hex(data_len as u16);

    out.write_all(&buf)?;
    if !prefix.is_empty() {
        out.write_all(prefix)?;
    }
    out.write_all(data)?;
    if !suffix.is_empty() {
        out.write_all(suffix)?;
    }
    Ok(data_len)
}

pub(crate) fn u16_to_hex(value: u16) -> [u8; 4] {
    let mut buf = [0u8; 4];
    hex::encode_to_slice((value as u16).to_be_bytes(), &mut buf).expect("two bytes to 4 hex chars never fails");
    buf
}
