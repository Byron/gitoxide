pub const SHA1_SIZE: usize = 20;

/// A SHA1 identifying objects
pub type Id = [u8; SHA1_SIZE];

pub fn id_from_20_bytes(b: &[u8]) -> Id {
    let mut id = [0; SHA1_SIZE];
    id.copy_from_slice(b);
    id
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd)]
pub enum Kind {
    Tag,
    Commit,
    Tree,
    Blob,
}
quick_error! {
    #[derive(Debug)]
    pub enum Error {
        InvalidObjectKind(kind: Vec<u8>) {
            display("Unknown object kind: {:?}", std::str::from_utf8(&kind))
        }
    }
}

impl Kind {
    pub fn from_bytes(s: &[u8]) -> Result<Kind, Error> {
        Ok(match s {
            b"tag" => Kind::Tag,
            b"commit" => Kind::Commit,
            b"tree" => Kind::Tree,
            b"blob" => Kind::Blob,
            _ => return Err(Error::InvalidObjectKind(s.to_owned())),
        })
    }
}

pub mod parsed;

#[cfg(test)]
mod tests {
    use super::*;
}
