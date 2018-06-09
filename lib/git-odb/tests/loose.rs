extern crate git_odb as odb;
extern crate hex;

use std::path::PathBuf;
use hex::FromHex;

use odb::{loose::Db, object::{parsed, Kind}};
use std::{fs::File, io::Read};
use odb::Time;
use odb::Sign;

fn fixture(path: &str) -> PathBuf {
    let mut b = PathBuf::from(file!());
    b.pop();
    b.push("fixtures");
    b.push(path);
    b
}

fn fixture_bytes(path: &str) -> Vec<u8> {
    let mut buf = Vec::new();
    File::open(fixture(path))
        .unwrap()
        .read_to_end(&mut buf)
        .unwrap();
    buf
}

fn bin(hex: &str) -> [u8; 20] {
    <[u8; 20]>::from_hex(hex).unwrap()
}
fn ldb() -> Db {
    odb::loose::at(fixture("objects"))
}

#[test]
fn loose_iter() {
    let mut oids = ldb().iter().map(|o| o.unwrap()).collect::<Vec<_>>();
    oids.sort();
    assert_eq!(
        oids,
        vec![
            bin("37d4e6c5c48ba0d245164c4e10d5f41140cab980"),
            bin("595dfd62fc1ad283d61bb47a24e7a1f66398f84d"),
            bin("6ba2a0ded519f737fd5b8d5ccfb141125ef3176f"),
            bin("722fe60ad4f0276d5a8121970b5bb9dccdad4ef9"),
            bin("96ae868b3539f551c88fd5f02394d022581b11b0"),
            bin("ffa700b4aca13b80cb6b98a078e7c96804f8e0ec"),
        ]
    )
}

#[test]
fn loose_find() {
    let mut o = ldb()
        .find(&bin("722fe60ad4f0276d5a8121970b5bb9dccdad4ef9"))
        .unwrap();
    assert_eq!(o.kind, Kind::Tag);
    assert_eq!(o.size, 1024);
    assert_eq!(o.parsed().unwrap(), parsed::Object::Tag(tag_fixture(7200)))
}

#[test]
fn loose_tag_parse() {
    let fixture = fixture_bytes("objects/tag.txt");
    let actual = parsed::Tag::from_bytes(&fixture).unwrap();
    assert_eq!(actual, tag_fixture(9000));
    assert_eq!(
        actual.target(),
        bin("ffa700b4aca13b80cb6b98a078e7c96804f8e0ec")
    );
    assert_eq!(actual.name_str().unwrap(), "1.0.0");
}

fn tag_fixture(offset: i32) -> parsed::Tag<'static> {
    parsed::Tag {
        target_raw: b"ffa700b4aca13b80cb6b98a078e7c96804f8e0ec",
        name_raw: b"1.0.0",
        target_kind: Kind::Commit,
        message: Some(b"for the signature\n"),
        signature: parsed::Signature {
            name: b"Sebastian Thiel",
            email: b"byronimo@gmail.com",
            time: Time {
                time: 1528473343,
                offset,
                sign: Sign::Plus,
            },
        },
    }
}
