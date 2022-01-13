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

    pub fn decode(data: &[u8]) -> Result<(Vec, &[u8]), decode::Error> {
        let (num_bits, data) = decode::u32(data).ok_or(decode::Error::Corrupt("eof reading amount of bits"))?;
        let (len, data) = decode::u32(data).ok_or(decode::Error::Corrupt("eof reading chunk length"))?;
        let len = len as usize;

        // NOTE: git does this by copying all bytes first, and then it will change the endianess in a separate loop.
        //       Maybe it's faster, but we can't do it without unsafe. Let's leave it to the optimizer and maybe
        //       one day somebody will find out that it's worth it to use unsafe here.
        let (mut bits, data) = decode::split_at_pos(data, len * std::mem::size_of::<u64>())
            .ok_or(decode::Error::Corrupt("eof while reading bit data"))?;
        let mut buf = std::vec::Vec::<u64>::with_capacity(len);
        for _ in 0..len {
            let (bit_num, rest) = bits.split_at(std::mem::size_of::<u64>());
            bits = rest;
            buf.push(u64::from_be_bytes(bit_num.try_into().unwrap()))
        }

        let (rlw, data) = decode::u32(data).ok_or(decode::Error::Corrupt("eof while reading run length width"))?;
        dbg!(rlw);

        Ok((
            Vec {
                num_bits,
                bits: buf,
                rlw: rlw as usize,
            },
            data,
        ))
    }

    /// A growable collection of u64 that are seen as stream of individual bits.
    #[allow(dead_code)]
    pub struct Vec {
        num_bits: u32,
        bits: std::vec::Vec<u64>,
        /// RLW is an offset into the `bits` buffer, so `1` translates into &bits[1] essentially.
        rlw: usize,
    }
}
