use super::*;
use crate::parsed::tag::parse_signature_nom;
use crate::{parsed, Sign};
use bstr::ByteSlice;
use hex::FromHex;
use std::path::PathBuf;

pub fn bin(hex: &str) -> [u8; 20] {
    <[u8; 20]>::from_hex(hex).unwrap()
}

pub fn fixture(path: &str) -> PathBuf {
    PathBuf::from("src/parsed/tests/fixtures").join(path)
}

fn fixture_bytes(path: &str) -> Vec<u8> {
    std::fs::read(fixture(path)).unwrap()
}

mod tag;

#[test]
fn parse_signature() {
    assert_eq!(
        parse_signature_nom(b"Sebastian Thiel <byronimo@gmail.com> 1528473343 +0230")
            .unwrap()
            .1,
        parsed::Signature {
            name: b"Sebastian Thiel".as_bstr(),
            email: b"byronimo@gmail.com".as_bstr(),
            time: Time {
                time: 1528473343,
                offset: 9000,
                sign: Sign::Plus,
            },
        }
    );
}
