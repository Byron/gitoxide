#![allow(missing_docs, unused)]

use bstr::BStr;
use git_hash::ObjectId;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Line<'a> {
    pub previous_oid: ObjectId,
    pub new_oid: ObjectId,
    #[cfg_attr(feature = "serde1", serde(borrow))]
    pub signature: git_actor::immutable::Signature<'a>,
    pub message: &'a BStr,
}

mod decode {
    use crate::file::log::Line;
    use nom::IResult;

    pub fn line<'a>(bytes: &'a [u8]) -> IResult<&[u8], Line<'a>> {
        todo!("line parsing")
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use bstr::ByteSlice;
        use git_actor::{Sign, Time};
        use git_hash::ObjectId;

        fn hex_to_oid(hex: &str) -> ObjectId {
            ObjectId::from_hex(hex.as_bytes()).unwrap()
        }

        #[test]
        fn entry_with_message_without_newline() {
            let ( remaining, res ) = line(b"a5828ae6b52137b913b978e16cd2334482eb4c1f 89b43f80a514aee58b662ad606e6352e03eaeee4 Sebastian Thiel <foo@example.com> 1618030561 +0800	pull --ff-only: Fast-forward").expect("successful parsing");
            assert!(remaining.is_empty(), "all consuming even without trailing newline");
            assert_eq!(
                res,
                Line {
                    previous_oid: hex_to_oid("a5828ae6b52137b913b978e16cd2334482eb4c1f"),
                    new_oid: hex_to_oid("89b43f80a514aee58b662ad606e6352e03eaeee4"),
                    signature: git_actor::immutable::Signature {
                        name: b"Sebastian Thiel".as_bstr(),
                        email: b"foo@example.com".as_bstr(),
                        time: Time {
                            time: 1618030561,
                            offset: 28800,
                            sign: Sign::Plus
                        }
                    },
                    message: b"pull --ff-only: Fast-forward".as_bstr()
                }
            );
        }
    }
}
