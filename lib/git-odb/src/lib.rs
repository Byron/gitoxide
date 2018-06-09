#[macro_use]
extern crate failure;
extern crate hex;
extern crate miniz_oxide;
extern crate walkdir;

pub mod object {
    use std::str;

    use failure::Error;

    pub type Id = [u8; 20];

    #[derive(PartialEq, Eq, Debug)]
    pub enum Kind {
        Tag,
    }

    impl Kind {
        pub fn from_bytes(s: &[u8]) -> Result<Kind, Error> {
            Ok(match s {
                b"tag" => Kind::Tag,
                _ => bail!("Unknown object kind: {:?}", str::from_utf8(s)),
            })
        }
    }
}

pub mod loose;
