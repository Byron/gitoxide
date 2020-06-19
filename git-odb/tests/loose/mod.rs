use bstr::ByteSlice;
use git_object::{borrowed, Sign, Time};

mod object {
    use git_odb::loose::Object;

    #[test]
    fn size_in_memory() {
        assert_eq!(
            std::mem::size_of::<Object>(),
            848,
            "Loose objects should not grow larger unexpectedly"
        )
    }
}

mod db {
    use crate::{fixture, hex_to_id};
    use git_odb::loose::Db;
    use pretty_assertions::assert_eq;

    fn ldb() -> Db {
        Db::at(fixture("objects"))
    }

    #[test]
    fn iter() {
        let mut oids = ldb().iter().map(Result::unwrap).collect::<Vec<_>>();
        oids.sort();
        assert_eq!(
            oids,
            vec![
                hex_to_id("37d4e6c5c48ba0d245164c4e10d5f41140cab980"), // blob
                hex_to_id("595dfd62fc1ad283d61bb47a24e7a1f66398f84d"), // blob
                hex_to_id("6ba2a0ded519f737fd5b8d5ccfb141125ef3176f"), // tree
                hex_to_id("722fe60ad4f0276d5a8121970b5bb9dccdad4ef9"), // tag
                hex_to_id("96ae868b3539f551c88fd5f02394d022581b11b0"), // tree
                hex_to_id("ffa700b4aca13b80cb6b98a078e7c96804f8e0ec"), // commit
            ]
        )
    }

    mod locate {
        use crate::{hex_to_id, loose::db::ldb, loose::signature};
        use bstr::ByteSlice;
        use git_object::{
            borrowed,
            borrowed::{TreeEntry, TreeMode},
            Kind,
        };
        use git_odb::loose;

        #[test]
        fn tag() {
            let mut o = locate("722fe60ad4f0276d5a8121970b5bb9dccdad4ef9");
            assert_eq!(o.kind, Kind::Tag);
            assert_eq!(o.size, 1024);
            let tag = o.decode().unwrap();
            let expected = borrowed::Object::Tag(borrowed::Tag {
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
                signature: signature(1528473343),
            });
            assert_eq!(tag, expected)
        }

        #[test]
        fn commit() {
            let mut o = locate("ffa700b4aca13b80cb6b98a078e7c96804f8e0ec");
            assert_eq!(o.kind, Kind::Commit);
            assert_eq!(o.size, 1084);
            let commit = o.decode().unwrap();
            let expected = borrowed::Object::Commit(borrowed::Commit {
                tree: b"6ba2a0ded519f737fd5b8d5ccfb141125ef3176f".as_bstr(),
                parents: vec![].into(),
                author: signature(1528473303),
                committer: signature(1528473303),
                encoding: None,
                message: b"initial commit\n".as_bstr(),
                pgp_signature: Some(b"-----BEGIN PGP SIGNATURE-----\n Comment: GPGTools - https://gpgtools.org\n \n iQIzBAABCgAdFiEEw7xSvXbiwjusbsBqZl+Z+p2ZlmwFAlsaptwACgkQZl+Z+p2Z\n lmxXSQ//fj6t7aWoEKeMdFigfj6OXWPUyrRbS0N9kpJeOfA0BIOea/6Jbn8J5qh1\n YRfrySOzHPXR5Y+w4GwLiVas66qyhAbk4yeqZM0JxBjHDyPyRGhjUd3y7WjEa6bj\n P0ACAIkYZQ/Q/LDE3eubmhAwEobBH3nZbwE+/zDIG0i265bD5C0iDumVOiKkSelw\n cr6FZVw1HH+GcabFkeLRZLNGmPqGdbeBwYERqb0U1aRCzV1xLYteoKwyWcYaH8E3\n 97z1rwhUO/L7o8WUEJtP3CLB0zuocslMxskf6bCeubBnRNJ0YrRmxGarxCP3vn4D\n 3a/MwECnl6mnUU9t+OnfvrzLDN73rlq8iasUq6hGe7Sje7waX6b2UGpxHqwykmXg\n VimD6Ah7svJanHryfJn38DvJW/wOMqmAnSUAp+Y8W9EIe0xVntCmtMyoKuqBoY7T\n JlZ1kHJte6ELIM5JOY9Gx7D0ZCSKZJQqyjoqtl36dsomT0I78/+7QS1DP4S6XB7d\n c3BYH0JkW81p7AAFbE543ttN0Z4wKXErMFqUKnPZUIEuybtlNYV+krRdfDBWQysT\n 3MBebjguVQ60oGs06PzeYBosKGQrHggAcwduLFuqXhLTJqN4UQ18RkE0vbtG3YA0\n +XtZQM13vURdfwFI5qitAGgw4EzPVrkWWzApzLCrRPEMbvP+b9A=\n =2qqN\n -----END PGP SIGNATURE-----".as_bstr()),
            });
            assert_eq!(commit, expected)
        }

        #[test]
        fn blob() {
            let mut o = locate("37d4e6c5c48ba0d245164c4e10d5f41140cab980");
            assert_eq!(
                o.decode().unwrap(),
                borrowed::Object::Blob(borrowed::Blob {
                    data: &[98, 108, 111, 98, 32, 57, 0, 104, 105, 32, 116, 104, 101, 114, 101, 10]
                }),
                "blobs cannot be parsed, but it's not an error either"
            );
        }

        fn locate(hex: &str) -> loose::Object {
            ldb().locate(&hex_to_id(hex)).unwrap()
        }

        #[test]
        fn tree() {
            let mut o = locate("6ba2a0ded519f737fd5b8d5ccfb141125ef3176f");
            assert_eq!(o.kind, Kind::Tree);
            assert_eq!(o.size, 66);

            let expected = borrowed::Object::Tree(borrowed::Tree {
                entries: vec![
                    TreeEntry {
                        mode: TreeMode::Tree,
                        filename: b"dir".as_bstr(),
                        oid: &[
                            150, 174, 134, 139, 53, 57, 245, 81, 200, 143, 213, 240, 35, 148, 208,
                            34, 88, 27, 17, 176,
                        ],
                    },
                    TreeEntry {
                        mode: TreeMode::Blob,
                        filename: b"file.txt".as_bstr(),
                        oid: &[
                            55, 212, 230, 197, 196, 139, 160, 210, 69, 22, 76, 78, 16, 213, 244,
                            17, 64, 202, 185, 128,
                        ],
                    },
                ],
            });
            let tree = o.decode().unwrap();
            assert_eq!(tree, expected)
        }
    }
}

fn signature(time: u32) -> borrowed::Signature<'static> {
    borrowed::Signature {
        name: b"Sebastian Thiel".as_bstr(),
        email: b"byronimo@gmail.com".as_bstr(),
        time: Time {
            time,
            offset: 7200,
            sign: Sign::Plus,
        },
    }
}
