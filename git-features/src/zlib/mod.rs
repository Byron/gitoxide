pub use flate2::{Decompress, Status};

/// non-streaming interfaces for decompression
pub mod inflate {
    use quick_error::quick_error;
    quick_error! {
        /// The error returned by various [Inflate methods][super::Inflate]
        #[allow(missing_docs)]
        #[derive(Debug)]
        pub enum Error {
            WriteInflated(err: std::io::Error) {
                display("Could not write all bytes when decompressing content")
                from()
            }
            Inflate(err: flate2::DecompressError) {
                display("Could not decode zip stream, status was '{:?}'", err)
                from()
            }
        }
    }
}

/// Decompress a few bytes of a zlib stream without allocation
pub struct Inflate {
    /// The actual decompressor doing all the work.
    pub state: Decompress,
}

impl Default for Inflate {
    fn default() -> Self {
        Inflate {
            state: Decompress::new(true),
        }
    }
}

impl Inflate {
    /// Run the decompressor exactly once. Cannot be run multiple times
    pub fn once(&mut self, input: &[u8], out: &mut [u8]) -> Result<(flate2::Status, usize, usize), inflate::Error> {
        let before_in = self.state.total_in();
        let before_out = self.state.total_out();
        let status = self.state.decompress(input, out, flate2::FlushDecompress::None)?;
        Ok((
            status,
            (self.state.total_in() - before_in) as usize,
            (self.state.total_out() - before_out) as usize,
        ))
    }
}

///
pub mod stream;
