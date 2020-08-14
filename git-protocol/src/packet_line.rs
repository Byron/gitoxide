pub mod encode {
    use quick_error::quick_error;
    use std::io;

    const MAX_DATA_LEN: usize = 65516;

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
        out.write_all(b"0000").map(|_| 4)
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
}
