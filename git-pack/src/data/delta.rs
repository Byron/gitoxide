/// Given the decompressed pack delta `d`, decode a size in bytes (either the base object size or the result object size)
/// Equivalent to [this canonical git function](https://github.com/git/git/blob/311531c9de557d25ac087c1637818bd2aad6eb3a/delta.h#L89)
pub fn decode_header_size(d: &[u8]) -> (u64, usize) {
    let mut i = 0;
    let mut size = 0u64;
    let mut consumed = 0;
    for cmd in d.iter() {
        consumed += 1;
        size |= (*cmd as u64 & 0x7f) << i;
        i += 7;
        if *cmd & 0x80 == 0 {
            break;
        }
    }
    (size, consumed)
}

pub fn apply(base: &[u8], mut target: &mut [u8], data: &[u8]) {
    let mut i = 0;
    while let Some(cmd) = data.get(i) {
        i += 1;
        match cmd {
            cmd if cmd & 0b1000_0000 != 0 => {
                let (mut ofs, mut size): (u32, u32) = (0, 0);
                if cmd & 0b0000_0001 != 0 {
                    ofs = data[i] as u32;
                    i += 1;
                }
                if cmd & 0b0000_0010 != 0 {
                    ofs |= (data[i] as u32) << 8;
                    i += 1;
                }
                if cmd & 0b0000_0100 != 0 {
                    ofs |= (data[i] as u32) << 16;
                    i += 1;
                }
                if cmd & 0b0000_1000 != 0 {
                    ofs |= (data[i] as u32) << 24;
                    i += 1;
                }
                if cmd & 0b0001_0000 != 0 {
                    size = data[i] as u32;
                    i += 1;
                }
                if cmd & 0b0010_0000 != 0 {
                    size |= (data[i] as u32) << 8;
                    i += 1;
                }
                if cmd & 0b0100_0000 != 0 {
                    size |= (data[i] as u32) << 16;
                    i += 1;
                }
                if size == 0 {
                    size = 0x10000; // 65536
                }
                let ofs = ofs as usize;
                std::io::Write::write(&mut target, &base[ofs..ofs + size as usize])
                    .expect("delta copy from base: byte slices must match");
            }
            0 => panic!("encountered unsupported command code: 0"),
            size => {
                std::io::Write::write(&mut target, &data[i..i + *size as usize])
                    .expect("delta copy data: slice sizes to match up");
                i += *size as usize;
            }
        }
    }
    assert_eq!(i, data.len());
    assert_eq!(target.len(), 0);
}
