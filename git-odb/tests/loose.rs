extern crate git_odb as odb;
extern crate hex;

mod utils;

use utils::*;

use std::{fs::File, io::Read};
use odb::{loose::Db, object::{parsed, Kind}};
use odb::Time;
use odb::Sign;

pub fn fixture_bytes(path: &str) -> Vec<u8> {
    let mut buf = Vec::new();
    File::open(fixture(path))
        .unwrap()
        .read_to_end(&mut buf)
        .unwrap();
    buf
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
}

fn tag_fixture(offset: i32) -> parsed::Tag<'static> {
    parsed::Tag {
        target_raw: b"ffa700b4aca13b80cb6b98a078e7c96804f8e0ec",
        name_raw: b"1.0.0",
        target_kind: Kind::Commit,
        message: Some(b"for the signature\n"),
        pgp_signature: Some(
            b"-----BEGIN PGP SIGNATURE-----
Comment: GPGTools - https://gpgtools.org

iQIzBAABCgAdFiEEw7xSvXbiwjusbsBqZl+Z+p2ZlmwFAlsapyYACgkQZl+Z+p2Z
lmy6Ug/+KzvzqiNpzz1bMVVAzp8NCbiEO3QGYPyeQc521lBwpaTrRYR+oHJY15r3
OdL5WDysTpjN8N5FNyfmvzkuPdTkK3JlYmO7VRjdA2xu/B6vIZLaOfAowFrhMvKo
8eoqwGcAP3rC5TuWEgzq2qhbjS4JXFLd4NLjWEFqT2Y2UKm+g8TeGOsa/0pF4Nq5
xeW4qCYR0WcQLFedbpkKHxag2GfaXKvzNNJdqYhVQssNa6BeSmsfDvlWYNe617wV
NvsR/zJT0wHb5SSH+h6QmwA7LQIQF//83Vc3aF7kv9D54r3ibXW5TjZ3WoeTUZO7
kefkzJ12EYDCFLPhHvXPog518nO8Ot46dX+okrF0/B4N3RFTvjKr7VAGTzv2D/Dg
DrD531S2F71b+JIRh641eeP7bjWFQi3tWLtrEOtjjsKPJfYRMKpYFnAO4UUJ6Rck
Z5fFXEUCO8d5WT56jzKDjmVoY01lA87O1YsP/J+zQAlc9v1k6jqeQ53LZNgTN+ue
5fJuSPT3T43pSOD1VQSr3aZ2Anc4Qu7K8uX9lkpxF9Sc0tDbeCosFLZMWNVp6m+e
cjHJZXWmV4CcRfmLsXzU8s2cR9A0DBvOxhPD1TlKC2JhBFXigjuL9U4Rbq9tdegB
2n8f2douw6624Tn/6Lm4a7AoxmU+CMiYagDxDL3RuZ8CAfh3bn0=
=aIns
-----END PGP SIGNATURE-----
",
        ),
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
