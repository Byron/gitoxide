mod db {
    use crate::{fixture, hex_to_id};
    use git_odb::loose::Db;
    use pretty_assertions::assert_eq;

    fn ldb() -> Db {
        Db::at(fixture("objects"))
    }

    #[test]
    fn iter() {
        let mut oids = ldb().iter().map(|o| o.unwrap()).collect::<Vec<_>>();
        oids.sort();
        assert_eq!(
            oids,
            vec![
                hex_to_id("37d4e6c5c48ba0d245164c4e10d5f41140cab980"), // blob
                hex_to_id("595dfd62fc1ad283d61bb47a24e7a1f66398f84d"), // blob
                hex_to_id("6ba2a0ded519f737fd5b8d5ccfb141125ef3176f"),
                hex_to_id("722fe60ad4f0276d5a8121970b5bb9dccdad4ef9"),
                hex_to_id("96ae868b3539f551c88fd5f02394d022581b11b0"),
                hex_to_id("ffa700b4aca13b80cb6b98a078e7c96804f8e0ec"),
            ]
        )
    }

    mod locate {
        use crate::hex_to_id;
        use crate::loose::db::ldb;
        use bstr::ByteSlice;
        use git_object::{
            borrowed,
            borrowed::{Entry, Mode},
            Kind, Sign, Time,
        };

        #[test]
        fn tag() {
            let mut o = ldb()
                .locate(&hex_to_id("722fe60ad4f0276d5a8121970b5bb9dccdad4ef9"))
                .unwrap();
            assert_eq!(o.kind, Kind::Tag);
            assert_eq!(o.size, 1024);
            let tag = o.parsed().unwrap();
            let expected = borrowed::Object::Tag(tag_fixture(7200));
            assert_eq!(tag, expected)
        }

        #[test]
        fn tree() {
            let mut o = ldb()
                .locate(&hex_to_id("6ba2a0ded519f737fd5b8d5ccfb141125ef3176f"))
                .unwrap();
            assert_eq!(o.kind, Kind::Tree);
            assert_eq!(o.size, 66);

            let expected = borrowed::Object::Tree(borrowed::Tree(vec![
                Entry {
                    mode: Mode::Tree,
                    filename: b"dir".as_bstr(),
                    oid: &[
                        150, 174, 134, 139, 53, 57, 245, 81, 200, 143, 213, 240, 35, 148, 208, 34,
                        88, 27, 17, 176,
                    ],
                },
                Entry {
                    mode: Mode::Blob,
                    filename: b"file.txt".as_bstr(),
                    oid: &[
                        55, 212, 230, 197, 196, 139, 160, 210, 69, 22, 76, 78, 16, 213, 244, 17,
                        64, 202, 185, 128,
                    ],
                },
            ]));
            let tree = o.parsed().unwrap();
            assert_eq!(tree, expected)
        }

        fn tag_fixture(offset: i32) -> borrowed::Tag<'static> {
            borrowed::Tag {
                target: b"ffa700b4aca13b80cb6b98a078e7c96804f8e0ec".as_bstr(),
                name: b"1.0.0".as_bstr(),
                target_kind: Kind::Commit,
                message: b"for the signature".as_bstr(),
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
-----END PGP SIGNATURE-----"
                        .as_bstr(),
                ),
                signature: borrowed::Signature {
                    name: b"Sebastian Thiel".as_bstr(),
                    email: b"byronimo@gmail.com".as_bstr(),
                    time: Time {
                        time: 1528473343,
                        offset,
                        sign: Sign::Plus,
                    },
                },
            }
        }
    }
}
