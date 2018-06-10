pub mod index {
    use failure::{Error, ResultExt};
    use std::path::Path;
    use filebuffer::FileBuffer;
    use byteorder::{BigEndian, ByteOrder};

    const V2_SIGNATURE: &'static [u8] = b"\xfftOc";
    const FOOTER_LEN: usize = 20;
    const N32_SIZE: usize = 4;

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
        data: FileBuffer,
        kind: Kind,
        version: u32,
        len: usize,
    }

    impl File {
        pub fn kind(&self) -> Kind {
            self.kind.clone()
        }
        pub fn len(&self) -> usize {
            self.len
        }
        pub fn version(&self) -> u32 {
            self.version
        }

        fn init(mut self) -> Result<Self, Error> {
            let idx_len = self.data.len();
            if idx_len < V2_SIGNATURE.len() + FOOTER_LEN {
                bail!("Pack index is truncated and not even empty");
            }
            let (kind, version) = {
                let d = &self.data;
                let kind = if &d[..V2_SIGNATURE.len()] == V2_SIGNATURE {
                    Kind::V2
                } else {
                    Kind::V1
                };
                let version = {
                    let mut v = 1;
                    if let &Kind::V2 = &kind {
                        v = BigEndian::read_u32(
                            &d[V2_SIGNATURE.len()..V2_SIGNATURE.len() + N32_SIZE],
                        );
                        if v != 2 {
                            bail!("Unsupported index version: {}", v);
                        }
                    }
                    v
                };
                (kind, version)
            };
            self.kind = kind;
            self.version = version;
            Ok(self)
        }

        pub fn at(path: &Path) -> Result<File, Error> {
            let file = File {
                data: FileBuffer::open(path)
                    .with_context(|_| format!("Could not map file at '{}'", path.display()))?,
                kind: Default::default(),
                len: 0,
                version: 0,
            };
            Ok(file.init()?)
        }
    }
}
