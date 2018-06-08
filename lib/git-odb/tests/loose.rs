extern crate git_odb as odb;
extern crate hex;

use std::path::PathBuf;
use hex::FromHex;

use odb::loose::Db;
use odb::ObjectKind;

fn fixture(path: &str) -> PathBuf {
    let mut b = PathBuf::from(file!());
    b.pop();
    b.push("fixtures");
    b.push(path);
    b
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
    let object = ldb()
        .find(&bin("722fe60ad4f0276d5a8121970b5bb9dccdad4ef9"))
        .unwrap();
    assert_eq!(object.kind(), ObjectKind::Tag);
    assert_eq!(object.size(), 42)
}
