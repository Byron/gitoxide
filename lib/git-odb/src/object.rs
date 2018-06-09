use std::str;

use failure::Error;

pub type Id = [u8; 20];

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd)]
pub enum Kind {
    Tag,
    Commit,
    Tree,
    Blob,
}

impl Kind {
    pub fn from_bytes(s: &[u8]) -> Result<Kind, Error> {
        Ok(match s {
            b"tag" => Kind::Tag,
            b"commit" => Kind::Commit,
            b"tree" => Kind::Tree,
            b"blob" => Kind::Blob,
            _ => bail!("Unknown object kind: {:?}", str::from_utf8(s)),
        })
    }
}

pub mod parsed {
    use failure::Error;
    use object::{Id, Kind};
    use std::{str, ops::Range};
    use hex::FromHex;

    #[derive(PartialEq, Eq, Debug, Hash)]
    pub enum Object<'data> {
        Tag(Tag<'data>),
    }

    impl<'data> Object<'data> {
        pub fn kind(&self) -> Kind {
            match self {
                Object::Tag(_) => Kind::Tag,
            }
        }
    }

    #[derive(PartialEq, Eq, Debug, Hash)]
    pub struct Tag<'data> {
        pub data: &'data [u8],
        pub target: Range<usize>,
        pub target_kind: Kind,
    }

    fn split2(d: &[u8], v: impl FnOnce(&[u8], &[u8]) -> bool) -> Result<(&[u8], &[u8]), Error> {
        let mut t = d.splitn(2, |&b| b == b' ');
        Ok(match (t.next(), t.next()) {
            (Some(t1), Some(t2)) => {
                if !v(t1, t2) {
                    bail!("Tokens in {:?} are invalid", str::from_utf8(d))
                }
                (t1, t2)
            }
            _ => bail!(
                "didnt find two tokens separated by space in {:?}'",
                str::from_utf8(d)
            ),
        })
    }

    fn range_of(from: &[u8], to: &[u8]) -> Range<usize> {
        let start = to.as_ptr().wrapping_offset_from(from.as_ptr()) as usize;
        start..start + to.len()
    }

    fn range_to_second_token(
        d: &[u8],
        v: impl FnOnce(&[u8], &[u8]) -> bool,
    ) -> Result<Range<usize>, Error> {
        let (_, t2) = split2(d, v)?;
        Ok(range_of(d, t2))
    }

    impl<'data> Tag<'data> {
        pub fn target(&self) -> Id {
            <[u8; 20]>::from_hex(&self.data[self.target.clone()]).expect("prior validation")
        }
        pub fn from_bytes(d: &'data [u8]) -> Result<Tag<'data>, Error> {
            let mut lines = d.split(|&b| b == b'\n');
            let (target, target_kind) =
                match (lines.next(), lines.next(), lines.next(), lines.next()) {
                    (Some(target), Some(kind), Some(_tag), Some(_tagger)) => {
                        let target = range_to_second_token(target, |f, v| {
                            f == b"object" && v.len() == 40 && <[u8; 20]>::from_hex(v).is_ok()
                        })?;
                        let kind = split2(kind, |f, _v| f == b"type")
                            .and_then(|(_, kind)| Kind::from_bytes(kind))?;
                        (target, kind)
                    }
                    _ => bail!("Expected four lines: target, type, tag and tagger"),
                };
            Ok(Tag {
                data: d,
                target,
                target_kind,
            })
        }
    }
}
