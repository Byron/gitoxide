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
    use super::Vec;

    impl Vec {
        /// Call `f(index)` for each bit that is true, given the index of the bit that identifies it uniquely within the bit array.
        ///
        /// The index is sequential like in any other vector.
        pub fn for_each_set_bit(&self, _f: impl FnMut(usize)) {
            todo!("for each")
        }
    }
}

/// A growable collection of u64 that are seen as stream of individual bits.
#[allow(dead_code)]
pub struct Vec {
    num_bits: u32,
    bits: std::vec::Vec<u64>,
    /// RLW is an offset into the `bits` buffer, so `1` translates into &bits[1] essentially.
    rlw: usize,
}
