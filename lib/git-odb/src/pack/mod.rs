pub mod index {
    use failure::{Error, ResultExt};
    use std::path::Path;
    use filebuffer::FileBuffer;
    use byteorder::{BigEndian, ByteOrder};
    use object::{self, SHA1_LEN};

    const V2_SIGNATURE: &'static [u8] = b"\xfftOc";
    const FOOTER_LEN: usize = SHA1_LEN * 2;
    const N32_SIZE: usize = 4;
    const FAN_LEN: usize = 256;
    const V1_OFFSET: usize = FAN_LEN * N32_SIZE;
    const V2_SHA1_OFFSET: usize = N32_SIZE * 2 + FAN_LEN * N32_SIZE;

    #[derive(PartialEq, Eq, Debug, Hash, Clone)]
    pub enum Kind {
        V1,
        V2,
    }

    impl Default for Kind {
        fn default() -> Self {
            Kind::V2
        }
    }

    #[derive(PartialEq, Eq, Debug, Hash, Clone)]
    pub struct Entry {
        oid: object::Id,
        ofs: u64,
        crc32: Option<u32>,
    }

    pub struct File {
        data: FileBuffer,
        kind: Kind,
        version: u32,
        size: u32,
        _fan: [u32; FAN_LEN],
    }

    impl File {
        pub fn kind(&self) -> Kind {
            self.kind.clone()
        }
        pub fn size(&self) -> u32 {
            self.size
        }
        pub fn version(&self) -> u32 {
            self.version
        }
        pub fn checksum_of_index(&self) -> object::Id {
            object::id_from_20_bytes(&self.data[self.data.len() - SHA1_LEN..])
        }
        pub fn checksum_of_pack(&self) -> object::Id {
            let from = self.data.len() - SHA1_LEN * 2;
            object::id_from_20_bytes(&self.data[from..from + SHA1_LEN])
        }

        pub fn iter<'a>(&'a self) -> Box<Iterator<Item = Entry> + 'a> {
            match self.kind {
                Kind::V1 => Box::new(
                    self.data[V1_OFFSET..]
                        .chunks(N32_SIZE + SHA1_LEN)
                        .take(self.size as usize)
                        .map(|c| {
                            let (ofs, oid) = c.split_at(N32_SIZE);
                            Entry {
                                oid: object::id_from_20_bytes(oid),
                                ofs: BigEndian::read_u32(ofs) as u64,
                                crc32: None,
                            }
                        }),
                ),
                Kind::V2 => Box::new(
                    izip!(self.data[V2_SHA1_OFFSET..].chunks(SHA1_LEN))
                        .take(self.size as usize)
                        .map(|c| Entry {
                            oid: [0; 20],
                            ofs: 0,
                            crc32: None,
                        }),
                ),
            }
        }

        pub fn at(path: &Path) -> Result<File, Error> {
            let data = FileBuffer::open(path)
                .with_context(|_| format!("Could not map file at '{}'", path.display()))?;
            let idx_len = data.len();
            if idx_len < FAN_LEN * N32_SIZE + FOOTER_LEN {
                bail!("Pack index is truncated and not even empty");
            }
            let (kind, version, fan, size) = {
                let (kind, d) = {
                    let (sig, d) = data.split_at(V2_SIGNATURE.len());
                    if sig == V2_SIGNATURE {
                        (Kind::V2, d)
                    } else {
                        (Kind::V1, &data[..])
                    }
                };
                let (version, d) = {
                    let (mut v, mut d) = (1, d);
                    if let &Kind::V2 = &kind {
                        let (vd, dr) = d.split_at(N32_SIZE);
                        d = dr;
                        v = BigEndian::read_u32(vd);
                        if v != 2 {
                            bail!("Unsupported index version: {}", v);
                        }
                    }
                    (v, d)
                };
                let (fan, bytes_read) = read_fan(d);
                let (_, _d) = d.split_at(bytes_read);
                let size = fan[FAN_LEN - 1];

                (kind, version, fan, size)
            };
            Ok(File {
                data: data,
                kind,
                size,
                version,
                _fan: fan,
            })
        }
    }

    fn read_fan(d: &[u8]) -> ([u32; FAN_LEN], usize) {
        let mut fan = [0; FAN_LEN];
        for (c, f) in d.chunks(N32_SIZE).zip(fan.iter_mut()) {
            *f = BigEndian::read_u32(c);
        }
        (fan, FAN_LEN * N32_SIZE)
    }
}
