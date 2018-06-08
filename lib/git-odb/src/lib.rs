extern crate failure;
extern crate hex;
extern crate walkdir;

pub type ObjectId = [u8; 20];

pub mod loose {
    use ObjectId;

    use std::path::PathBuf;

    use walkdir::WalkDir;
    use failure::Error;
    use hex::FromHex;

    pub struct LooseObjectDb {
        pub path: PathBuf,
    }

    impl LooseObjectDb {
        pub fn iter(&self) -> impl Iterator<Item = Result<ObjectId, Error>> {
            use std::path::Component::Normal;
            WalkDir::new(&self.path)
                .min_depth(2)
                .max_depth(3)
                .follow_links(false)
                .into_iter()
                .map(|e| e.map_err(Error::from))
                .filter_map(|e| {
                    let mut is_valid_path = false;
                    let e = e.map(|e| {
                        let p = e.path();
                        let mut pc = p.components();
                        println!("{}", p.display());
                        if let (Some(Normal(c1)), Some(Normal(c2))) = (pc.next(), pc.next()) {
                            if c1.len() == 2 && c2.len() == 38 {
                                if let (Some(c1), Some(c2)) = (c1.to_str(), c2.to_str()) {
                                    let mut buf = [0u8; 40];
                                    {
                                        let (first_byte, rest) = buf.split_at_mut(2);
                                        first_byte.copy_from_slice(c1.as_bytes());
                                        rest.copy_from_slice(c2.as_bytes());
                                    }
                                    if let Ok(b) = <[u8; 20]>::from_hex(&buf[..]) {
                                        is_valid_path = true;
                                        return b;
                                    }
                                }
                            }
                        }
                        [0u8; 20]
                    });
                    if is_valid_path {
                        Some(e)
                    } else {
                        None
                    }
                })
        }
    }

    pub fn at(path: impl Into<PathBuf>) -> LooseObjectDb {
        LooseObjectDb { path: path.into() }
    }
}
