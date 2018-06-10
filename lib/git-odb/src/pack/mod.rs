pub mod index {
    use failure::{Error, ResultExt};
    use std::path::Path;
    use filebuffer::FileBuffer;
    use byteorder::{BigEndian, ByteOrder};

    const V2_SIGNATURE: &'static [u8] = b"\xfftOc";
    const FOOTER_LEN: usize = 20;
    const N32_SIZE: usize = 4;
    const FAN_LEN: usize = 256;

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

    pub struct File {
        _data: FileBuffer,
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
                _data: data,
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
