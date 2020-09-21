use byteorder::WriteBytesExt;
use git_object as object;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{message}: {:?}", std::str::from_utf8(.number))]
    ParseIntegerError {
        source: btoi::ParseIntegerError,
        message: &'static str,
        number: Vec<u8>,
    },
    #[error("{0}")]
    InvalidHeader(&'static str),
    #[error(transparent)]
    ObjectHeader(#[from] object::Error),
}

pub fn decode(input: &[u8]) -> Result<(object::Kind, u64, usize), Error> {
    let header_end = input
        .iter()
        .position(|&b| b == 0)
        .ok_or_else(|| Error::InvalidHeader("Did not find 0 byte in header"))?;
    let header = &input[..header_end];
    let mut split = header.split(|&b| b == b' ');
    match (split.next(), split.next()) {
        (Some(kind), Some(size)) => Ok((
            object::Kind::from_bytes(kind)?,
            btoi::btoi(size).map_err(|source| Error::ParseIntegerError {
                message: "Object size in header could not be parsed",
                number: size.to_owned(),
                source,
            })?,
            header_end + 1, // account for 0 byte
        )),
        _ => Err(Error::InvalidHeader("Expected '<type> <size>'")),
    }
}

fn kind_to_bytes_with_space(object: object::Kind) -> &'static [u8] {
    use object::Kind::*;
    match object {
        Tree => b"tree ",
        Blob => b"blob ",
        Commit => b"commit ",
        Tag => b"tag ",
    }
}

pub fn encode(object: object::Kind, size: u64, mut out: impl std::io::Write) -> Result<usize, std::io::Error> {
    let mut written = out.write(kind_to_bytes_with_space(object))?;
    written += itoa::write(&mut out, size)?;
    out.write_u8(0)?;
    Ok(written + 1)
}

#[cfg(test)]
mod tests {
    mod encode_decode_round_trip {
        use crate::loose::object::header;
        use git_object::bstr::ByteSlice;

        #[test]
        fn all() -> Result<(), Box<dyn std::error::Error>> {
            let mut buf = [0; 20];
            for (kind, size, expected) in &[
                (git_object::Kind::Tree, 1234, &b"tree 1234\0"[..]),
                (git_object::Kind::Blob, 0, b"blob 0\0"),
                (git_object::Kind::Commit, 24241, b"commit 24241\0"),
                (git_object::Kind::Tag, 9999999999, b"tag 9999999999\0"),
            ] {
                let written = header::encode(*kind, *size, &mut buf[..])?;
                assert_eq!(buf[..written].as_bstr(), expected.as_bstr());
                let (actual_kind, actual_size, actual_read) = header::decode(&buf[..written])?;
                assert_eq!(actual_kind, *kind);
                assert_eq!(actual_size, *size);
                assert_eq!(actual_read, written);
            }
            Ok(())
        }
    }
}
