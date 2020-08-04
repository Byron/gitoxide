use super::Status;
use miniz_modified::InflateState;
use miniz_oxide::{inflate, DataFormat, MZError, MZFlush, MZStatus};
use quick_error::quick_error;
use std::{io, io::BufRead};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Decompression {
            display("The decompression failed due to an unknown error")
        }
        ZLibNeedDict(adler: u32) {
            display("Probably the stream is damaged, adler value is {}", adler)
        }
    }
}

pub(crate) struct Inflate {
    state: InflateState,
    pub(crate) total_in: u64,
    total_out: u64,
}

impl Default for Inflate {
    fn default() -> Self {
        Inflate {
            state: InflateState::new(DataFormat::Zlib),
            total_in: 0,
            total_out: 0,
        }
    }
}

impl Inflate {
    pub fn reset(&mut self) {
        self.state.reset(DataFormat::Zlib);
        self.total_in = 0;
        self.total_out = 0;
    }

    fn decompress(&mut self, input: &[u8], output: &mut [u8], flush: MZFlush) -> Result<Status, Error> {
        let res = inflate::stream::inflate(&mut self.state, input, output, flush);
        self.total_in += res.bytes_consumed as u64;
        self.total_out += res.bytes_written as u64;

        match res.status {
            Ok(status) => match status {
                MZStatus::Ok => Ok(Status::Ok),
                MZStatus::StreamEnd => Ok(Status::StreamEnd),
                MZStatus::NeedDict => Err(Error::ZLibNeedDict(self.state.decompressor().adler32().unwrap_or(0))),
            },
            Err(status) => match status {
                MZError::Buf => Ok(Status::BufError),
                _ => Err(Error::Decompression),
            },
        }
    }
}

/// Provide streaming decompression using the `std::io::Read` trait.
/// If `std::io::BufReader` is used, an allocation for the input buffer will be performed.
pub struct InflateReader<R> {
    pub(crate) inner: R,
    pub(crate) decompressor: Inflate,
}

impl<R> InflateReader<R>
where
    R: io::BufRead,
{
    pub fn from_read(read: R) -> InflateReader<R> {
        // TODO: Performance opportunity - a buf reader that doesn't allocate
        InflateReader {
            decompressor: Inflate::default(),
            inner: read,
        }
    }

    pub fn reset(&mut self, read: R) {
        self.inner = read;
        self.decompressor.reset();
    }
}

impl<R> io::Read for InflateReader<R>
where
    R: BufRead,
{
    fn read(&mut self, into: &mut [u8]) -> io::Result<usize> {
        read(&mut self.inner, &mut self.decompressor, into)
    }
}

/// The boxed variant is faster for what we do (moving the decompressor in and out a lot)
pub struct InflateReaderBoxed<R> {
    pub(crate) inner: R,
    pub(crate) decompressor: Box<Inflate>,
}

impl<R> io::Read for InflateReaderBoxed<R>
where
    R: BufRead,
{
    fn read(&mut self, into: &mut [u8]) -> io::Result<usize> {
        read(&mut self.inner, &mut self.decompressor, into)
    }
}

/// Adapted from [flate2](https://github.com/alexcrichton/flate2-rs/blob/57972d77dab09acad4aa2fa3beedb1f69fa64b27/src/zio.rs#L118)
fn read<R>(obj: &mut R, data: &mut Inflate, dst: &mut [u8]) -> io::Result<usize>
where
    R: BufRead,
{
    loop {
        let (read, consumed, ret, eof);
        {
            let input = obj.fill_buf()?;
            eof = input.is_empty();
            let before_out = data.total_out;
            let before_in = data.total_in;
            let flush = if eof { MZFlush::Finish } else { MZFlush::None };
            ret = data.decompress(input, dst, flush);
            read = (data.total_out - before_out) as usize;
            consumed = (data.total_in - before_in) as usize;
        }
        obj.consume(consumed);

        match ret {
            // If we haven't ready any data and we haven't hit EOF yet,
            // then we need to keep asking for more data because if we
            // return that 0 bytes of data have been read then it will
            // be interpreted as EOF.
            Ok(Status::Ok) | Ok(Status::BufError) if read == 0 && !eof && !dst.is_empty() => continue,
            Ok(Status::Ok) | Ok(Status::BufError) | Ok(Status::StreamEnd) => return Ok(read),

            Err(..) => return Err(io::Error::new(io::ErrorKind::InvalidInput, "corrupt deflate stream")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use git_object::bstr::ByteSlice;
    use std::{io::Read, path::PathBuf};

    fn fixture_path(path: &str) -> PathBuf {
        PathBuf::from("tests/fixtures").join(path)
    }

    #[test]
    fn small_file_decompress() -> Result<(), Box<dyn std::error::Error>> {
        let r = InflateReader::from_read(io::BufReader::new(std::fs::File::open(fixture_path(
            "objects/37/d4e6c5c48ba0d245164c4e10d5f41140cab980",
        ))?));
        let mut bytes = r.bytes();
        let content = bytes.by_ref().take(16).collect::<Result<Vec<_>, _>>()?;
        assert_eq!(content.as_slice().as_bstr(), b"blob 9\0hi there\n".as_bstr());
        assert!(bytes.next().is_none());
        Ok(())
    }
}

mod miniz_modified {
    // MIT License
    //
    // Copyright (c) 2017 Frommi
    //
    // Permission is hereby granted, free of charge, to any person obtaining a copy
    // of this software and associated documentation files (the "Software"), to deal
    // in the Software without restriction, including without limitation the rights
    // to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    // copies of the Software, and to permit persons to whom the Software is
    // furnished to do so, subject to the following conditions:
    //
    // The above copyright notice and this permission notice shall be included in all
    // copies or substantial portions of the Software.
    //
    // THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    // IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    // FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    // AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    // LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    // OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    // SOFTWARE.

    //! Extra streaming decompression functionality.
    //!
    //! As of now this is mainly inteded for use to build a higher-level wrapper.
    use std::boxed::Box;
    use std::{cmp, mem};

    use miniz_oxide::inflate::core::{decompress, inflate_flags, DecompressorOxide, TINFL_LZ_DICT_SIZE};
    use miniz_oxide::inflate::TINFLStatus;
    use miniz_oxide::{DataFormat, MZError, MZFlush, MZResult, MZStatus, StreamResult};

    /// A struct that compbines a decompressor with extra data for streaming decompression.
    ///
    pub struct InflateState {
        /// Inner decompressor struct
        decomp: DecompressorOxide,

        /// Buffer of input bytes for matches.
        /// TODO: Could probably do this a bit cleaner with some
        /// Cursor-like class.
        /// We may also look into whether we need to keep a buffer here, or just one in the
        /// decompressor struct.
        dict: [u8; TINFL_LZ_DICT_SIZE],
        /// Where in the buffer are we currently at?
        dict_ofs: usize,
        /// How many bytes of data to be flushed is there currently in the buffer?
        dict_avail: usize,

        first_call: bool,
        has_flushed: bool,

        /// Whether the input data is wrapped in a zlib header and checksum.
        /// TODO: This should be stored in the decompressor.
        data_format: DataFormat,
        last_status: TINFLStatus,
    }

    impl Default for InflateState {
        fn default() -> Self {
            InflateState {
                decomp: DecompressorOxide::default(),
                dict: [0; TINFL_LZ_DICT_SIZE],
                dict_ofs: 0,
                dict_avail: 0,
                first_call: true,
                has_flushed: false,
                data_format: DataFormat::Raw,
                last_status: TINFLStatus::NeedsMoreInput,
            }
        }
    }
    impl InflateState {
        /// Create a new state.
        ///
        /// Note that this struct is quite large due to internal buffers, and as such storing it on
        /// the stack is not recommended.
        ///
        /// # Parameters
        /// `data_format`: Determines whether the compressed data is assumed to wrapped with zlib
        /// metadata.
        pub fn new(data_format: DataFormat) -> InflateState {
            let mut b = InflateState::default();
            b.data_format = data_format;
            b
        }

        /// Create a new state on the heap.
        ///
        /// # Parameters
        /// `data_format`: Determines whether the compressed data is assumed to wrapped with zlib
        /// metadata.
        pub fn new_boxed(data_format: DataFormat) -> Box<InflateState> {
            let mut b: Box<InflateState> = Box::default();
            b.data_format = data_format;
            b
        }

        /// Access the innner decompressor.
        pub fn decompressor(&mut self) -> &mut DecompressorOxide {
            &mut self.decomp
        }

        /// Return the status of the last call to `inflate` with this `InflateState`.
        pub fn last_status(&self) -> TINFLStatus {
            self.last_status
        }

        /// Create a new state using miniz/zlib style window bits parameter.
        ///
        /// The decompressor does not support different window sizes. As such,
        /// any positive (>0) value will set the zlib header flag, while a negative one
        /// will not.
        pub fn new_boxed_with_window_bits(window_bits: i32) -> Box<InflateState> {
            let mut b: Box<InflateState> = Box::default();
            b.data_format = from_window_bits(window_bits);
            b
        }

        /// Reset the decompressor without re-allocating memory, using the given
        /// data format.
        pub fn reset(&mut self, data_format: DataFormat) {
            self.decompressor().init();
            // Don't reset it - it's not required for this to work and it costs 5s for 7.5 million decompressions
            // self.dict = [0; TINFL_LZ_DICT_SIZE];
            self.dict_ofs = 0;
            self.dict_avail = 0;
            self.first_call = true;
            self.has_flushed = false;
            self.data_format = data_format;
            self.last_status = TINFLStatus::NeedsMoreInput;
        }
    }

    /// Try to decompress from `input` to `output` with the given `InflateState`
    ///
    /// # Errors
    ///
    /// Returns `MZError::Buf` If the size of the `output` slice is empty or no progress was made due to
    /// lack of expected input data or called after the decompression was
    /// finished without MZFlush::Finish.
    ///
    /// Returns `MZError::Param` if the compressor parameters are set wrong.
    pub fn inflate(state: &mut InflateState, input: &[u8], output: &mut [u8], flush: MZFlush) -> StreamResult {
        let mut bytes_consumed = 0;
        let mut bytes_written = 0;
        let mut next_in = input;
        let mut next_out = output;

        if flush == MZFlush::Full {
            return error(MZError::Stream);
        }

        let mut decomp_flags = inflate_flags::TINFL_FLAG_COMPUTE_ADLER32;
        if state.data_format == DataFormat::Zlib {
            decomp_flags |= inflate_flags::TINFL_FLAG_PARSE_ZLIB_HEADER;
        }

        let first_call = state.first_call;
        state.first_call = false;
        if (state.last_status as i32) < 0 {
            return error(MZError::Data);
        }

        if state.has_flushed && (flush != MZFlush::Finish) {
            return error(MZError::Stream);
        }
        state.has_flushed |= flush == MZFlush::Finish;

        if (flush == MZFlush::Finish) && first_call {
            decomp_flags |= inflate_flags::TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF;

            let status = decompress(&mut state.decomp, next_in, next_out, 0, decomp_flags);
            let in_bytes = status.1;
            let out_bytes = status.2;
            let status = status.0;

            state.last_status = status;

            bytes_consumed += in_bytes;
            bytes_written += out_bytes;

            let ret_status = {
                if (status as i32) < 0 {
                    Err(MZError::Data)
                } else if status != TINFLStatus::Done {
                    state.last_status = TINFLStatus::Failed;
                    Err(MZError::Buf)
                } else {
                    Ok(MZStatus::StreamEnd)
                }
            };
            return StreamResult {
                bytes_consumed,
                bytes_written,
                status: ret_status,
            };
        }

        if flush != MZFlush::Finish {
            decomp_flags |= inflate_flags::TINFL_FLAG_HAS_MORE_INPUT;
        }

        if state.dict_avail != 0 {
            bytes_written += push_dict_out(state, &mut next_out);
            return StreamResult {
                bytes_consumed,
                bytes_written,
                status: Ok(if (state.last_status == TINFLStatus::Done) && (state.dict_avail == 0) {
                    MZStatus::StreamEnd
                } else {
                    MZStatus::Ok
                }),
            };
        }

        let status = inflate_loop(
            state,
            &mut next_in,
            &mut next_out,
            &mut bytes_consumed,
            &mut bytes_written,
            decomp_flags,
            flush,
        );
        StreamResult {
            bytes_consumed,
            bytes_written,
            status,
        }
    }

    fn inflate_loop(
        state: &mut InflateState,
        next_in: &mut &[u8],
        next_out: &mut &mut [u8],
        total_in: &mut usize,
        total_out: &mut usize,
        decomp_flags: u32,
        flush: MZFlush,
    ) -> MZResult {
        let orig_in_len = next_in.len();
        loop {
            let status = decompress(
                &mut state.decomp,
                *next_in,
                &mut state.dict,
                state.dict_ofs,
                decomp_flags,
            );

            let in_bytes = status.1;
            let out_bytes = status.2;
            let status = status.0;

            state.last_status = status;

            *next_in = &next_in[in_bytes..];
            *total_in += in_bytes;

            state.dict_avail = out_bytes;
            *total_out += push_dict_out(state, next_out);

            // The stream was corrupted, and decompression failed.
            if (status as i32) < 0 {
                return Err(MZError::Data);
            }

            // The decompressor has flushed all it's data and is waiting for more input, but
            // there was no more input provided.
            if (status == TINFLStatus::NeedsMoreInput) && orig_in_len == 0 {
                return Err(MZError::Buf);
            }

            if flush == MZFlush::Finish {
                if status == TINFLStatus::Done {
                    // There is not enough space in the output buffer to flush the remaining
                    // decompressed data in the internal buffer.
                    return if state.dict_avail != 0 {
                        Err(MZError::Buf)
                    } else {
                        Ok(MZStatus::StreamEnd)
                    };
                // No more space in the output buffer, but we're not done.
                } else if next_out.is_empty() {
                    return Err(MZError::Buf);
                }
            } else {
                // We're not expected to finish, so it's fine if we can't flush everything yet.
                let empty_buf = next_in.is_empty() || next_out.is_empty();
                if (status == TINFLStatus::Done) || empty_buf || (state.dict_avail != 0) {
                    return if (status == TINFLStatus::Done) && (state.dict_avail == 0) {
                        // No more data left, we're done.
                        Ok(MZStatus::StreamEnd)
                    } else {
                        // Ok for now, still waiting for more input data or output space.
                        Ok(MZStatus::Ok)
                    };
                }
            }
        }
    }

    fn push_dict_out(state: &mut InflateState, next_out: &mut &mut [u8]) -> usize {
        let n = cmp::min(state.dict_avail as usize, next_out.len());
        (next_out[..n]).copy_from_slice(&state.dict[state.dict_ofs..state.dict_ofs + n]);
        *next_out = &mut mem::replace(next_out, &mut [])[n..];
        state.dict_avail -= n;
        state.dict_ofs = (state.dict_ofs + (n)) & (TINFL_LZ_DICT_SIZE - 1);
        n
    }

    fn from_window_bits(window_bits: i32) -> DataFormat {
        if window_bits > 0 {
            DataFormat::Zlib
        } else {
            DataFormat::Raw
        }
    }

    pub(crate) fn error(error: MZError) -> StreamResult {
        StreamResult {
            bytes_consumed: 0,
            bytes_written: 0,
            status: Err(error),
        }
    }

    #[cfg(test)]
    mod test {
        use super::{inflate, InflateState};
        use miniz_oxide::{DataFormat, MZFlush, MZStatus};
        use std::vec;

        #[test]
        fn test_state() {
            let encoded = [
                120u8, 156, 243, 72, 205, 201, 201, 215, 81, 168, 202, 201, 76, 82, 4, 0, 27, 101, 4, 19,
            ];
            let mut out = vec![0; 50];
            let mut state = InflateState::new_boxed(DataFormat::Zlib);
            let res = inflate(&mut state, &encoded, &mut out, MZFlush::Finish);
            let status = res.status.expect("Failed to decompress!");
            assert_eq!(status, MZStatus::StreamEnd);
            assert_eq!(out[..res.bytes_written as usize], b"Hello, zlib!"[..]);
            assert_eq!(res.bytes_consumed, encoded.len());

            state.reset(DataFormat::Zlib);
            let status = res.status.expect("Failed to decompress!");
            assert_eq!(status, MZStatus::StreamEnd);
            assert_eq!(out[..res.bytes_written as usize], b"Hello, zlib!"[..]);
            assert_eq!(res.bytes_consumed, encoded.len());
        }
    }
}
