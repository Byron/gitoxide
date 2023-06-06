pub use flate2::{Decompress, Status};

/// non-streaming interfaces for decompression
pub mod inflate {
    /// The error returned by various [Inflate methods][super::Inflate]
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("Could not write all bytes when decompressing content")]
        WriteInflated(#[from] std::io::Error),
        #[error("Could not decode zip stream, status was '{0:?}'")]
        Inflate(#[from] flate2::DecompressError),
        #[error("The zlib status indicated an error, status was '{0:?}'")]
        Status(flate2::Status),
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

    /// Ready this instance for decoding another data stream.
    pub fn reset(&mut self) {
        self.state.reset(true);
    }
}

///
pub mod stream;
