use std::convert::TryInto;

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
}

pub fn decode(data: &[u8]) -> Result<(Vec, &[u8]), decode::Error> {
    use self::decode::Error;
    use crate::decode;

    let (num_bits, data) = decode::u32(data).ok_or(Error::Corrupt("eof reading amount of bits"))?;
    let (len, data) = decode::u32(data).ok_or(Error::Corrupt("eof reading chunk length"))?;
    let len = len as usize;

    // NOTE: git does this by copying all bytes first, and then it will change the endianess in a separate loop.
    //       Maybe it's faster, but we can't do it without unsafe. Let's leave it to the optimizer and maybe
    //       one day somebody will find out that it's worth it to use unsafe here.
    let (mut bits, data) = decode::split_at_pos(data, len * std::mem::size_of::<u64>())
        .ok_or(Error::Corrupt("eof while reading bit data"))?;
    let mut buf = std::vec::Vec::<u64>::with_capacity(len);
    for _ in 0..len {
        let (bit_num, rest) = bits.split_at(std::mem::size_of::<u64>());
        bits = rest;
        buf.push(u64::from_be_bytes(bit_num.try_into().unwrap()))
    }

    let (rlw, data) = decode::u32(data).ok_or(Error::Corrupt("eof while reading run length width"))?;

    Ok((
        Vec {
            num_bits,
            bits: buf,
            rlw: rlw as usize,
        },
        data,
    ))
}

mod access {
    use std::convert::TryInto;

    use super::Vec;

    impl Vec {
        /// Call `f(index)` for each bit that is true, given the index of the bit that identifies it uniquely within the bit array.
        /// If `f` returns `None` the iteration will be stopped and `None` is returned.
        ///
        /// The index is sequential like in any other vector.
        pub fn for_each_set_bit(&self, mut f: impl FnMut(usize) -> Option<()>) -> Option<()> {
            let mut index = 0;
            let mut iter = self.bits.iter();
            while let Some(word) = iter.next() {
                if rlw_runbit_is_set(word) {
                    let len = rlw_running_len_bits(word);
                    for _ in 0..len {
                        f(index)?;
                        index += 1;
                    }
                } else {
                    index += rlw_running_len_bits(word);
                }

                for _ in 0..rlw_literal_words(word) {
                    let word = iter
                        .next()
                        .expect("BUG: ran out of words while going through uncompressed portion");
                    for bit_index in 0..64 {
                        if word & (1 << bit_index) != 0 {
                            f(index)?;
                        }
                        index += 1;
                    }
                }
            }
            Some(())
        }

        /// The amount of bits we are currently holding.
        pub fn num_bits(&self) -> usize {
            self.num_bits.try_into().expect("we are not on 16 bit systems")
        }
    }

    #[inline]
    fn rlw_running_len_bits(w: &u64) -> usize {
        rlw_running_len(w) * 64
    }

    #[inline]
    fn rlw_running_len(w: &u64) -> usize {
        (w >> 1) as usize & RLW_LARGEST_RUNNING_COUNT
    }

    #[inline]
    fn rlw_literal_words(w: &u64) -> usize {
        (w >> (1 + RLW_RUNNING_BITS)) as usize
    }

    #[inline]
    fn rlw_runbit_is_set(w: &u64) -> bool {
        w & 1 == 1
    }

    const RLW_RUNNING_BITS: usize = 32;
    const RLW_LARGEST_RUNNING_COUNT: usize = (1 << RLW_RUNNING_BITS) - 1;
}

/// A growable collection of u64 that are seen as stream of individual bits.
#[allow(dead_code)]
pub struct Vec {
    num_bits: u32,
    bits: std::vec::Vec<u64>,
    /// RLW is an offset into the `bits` buffer, so `1` translates into &bits\[1] essentially.
    rlw: usize,
}
