use super::u16_to_hex;
use crate::{encode::Error, Channel, DELIMITER_LINE, ERR_PREFIX, FLUSH_LINE, MAX_DATA_LEN, RESPONSE_END_LINE};
use std::io;

/// Write a response-end message to `out`.
pub fn response_end_to_write(mut out: impl io::Write) -> io::Result<usize> {
    out.write_all(RESPONSE_END_LINE).map(|_| 4)
}

/// Write a delim message to `out`.
pub fn delim_to_write(mut out: impl io::Write) -> io::Result<usize> {
    out.write_all(DELIMITER_LINE).map(|_| 4)
}

/// Write a flush message to `out`.
pub fn flush_to_write(mut out: impl io::Write) -> io::Result<usize> {
    out.write_all(FLUSH_LINE).map(|_| 4)
}

/// Write an error `message` to `out`.
pub fn error_to_write(message: &[u8], out: impl io::Write) -> Result<usize, Error> {
    prefixed_data_to_write(ERR_PREFIX, message, out)
}

/// Write `data` of `kind` to `out` using side-band encoding.
pub fn band_to_write(kind: Channel, data: &[u8], out: impl io::Write) -> Result<usize, Error> {
    prefixed_data_to_write(&[kind as u8], data, out)
}

/// Write a `data` message to `out`.
pub fn data_to_write(data: &[u8], out: impl io::Write) -> Result<usize, Error> {
    prefixed_data_to_write(&[], data, out)
}

/// Write a `text` message to `out`, which is assured to end in a newline.
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
