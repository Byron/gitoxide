#![deny(unsafe_code, missing_docs, rust_2018_idioms)]
#![allow(missing_docs)]
//! An implementation of the shared parts of git bitmaps used in `git-pack`, `git-index` and `git-worktree`.
//!
//! Note that many tests are performed indirectly by tests in the aforementioned consumer crates.

/// Bitmap utilities for the advanced word-aligned hybrid bitmap
pub mod ewah {
    pub mod decode {
        use quick_error::quick_error;

        quick_error! {
            #[derive(Debug)]
            pub enum Error {
                Corrupt(message: &'static str) {
                    display("{}", message)
                }
            }
        }

        #[inline]
        pub(crate) fn split_at_pos(data: &[u8], pos: usize) -> Option<(&[u8], &[u8])> {
            if data.len() < pos {
                return None;
            }
            data.split_at(pos).into()
        }

        #[inline]
        pub(crate) fn u32(data: &[u8]) -> Option<(u32, &[u8])> {
            split_at_pos(data, 4).map(|(num, data)| (u32::from_be_bytes(num.try_into().unwrap()), data))
        }
    }

    pub fn decode(data: &[u8]) -> Result<(Array, &[u8]), decode::Error> {
        let (num_bits, data) = decode::u32(data).ok_or_else(|| decode::Error::Corrupt("eof reading amount of bits"))?;
        let (len, data) = decode::u32(data).ok_or_else(|| decode::Error::Corrupt("eof reading chunk length"))?;
        let len = len as usize;

        let (mut bits, data) =
            decode::split_at_pos(data, len).ok_or_else(|| decode::Error::Corrupt("eof while reading bit data"))?;
        let mut buf = Vec::<u64>::with_capacity(len);
        for _ in 0..len {
            let (bit_num, rest) = bits.split_at(8);
            bits = rest;
            buf.push(u64::from_be_bytes(bit_num.try_into().unwrap()))
        }

        let (rlw, data) =
            decode::u32(data).ok_or_else(|| decode::Error::Corrupt("eof while reading run length width"))?;

        Ok((
            Array {
                num_bits,
                bits: buf,
                rlw,
            },
            data,
        ))
    }

    #[allow(dead_code)]
    pub struct Array {
        num_bits: u32,
        bits: Vec<u64>,
        rlw: u32,
    }
}
