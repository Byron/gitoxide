use crate::{fixture_path, hex_to_id};
use git_odb::loose::{self, Db};
use pretty_assertions::assert_eq;

fn ldb() -> Db {
    Db::at(fixture_path("objects"))
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
            hex_to_id("a706d7cd20fc8ce71489f34b50cf01011c104193"), // blob (big)
            hex_to_id("ffa700b4aca13b80cb6b98a078e7c96804f8e0ec"), // commit
        ]
    )
}

pub fn locate(hex: &str) -> loose::Object {
    ldb().locate(hex_to_id(hex).to_borrowed()).unwrap().unwrap()
}

mod locate {
    use crate::{
        hex_to_id,
        loose::db::{ldb, locate},
        loose::signature,
    };
    use git_object::{borrowed, borrowed::tree, bstr::ByteSlice, Kind, TreeMode};
    use git_odb::loose;
    use std::io::Read;

    #[test]
    fn tag() {
        let mut o = locate("722fe60ad4f0276d5a8121970b5bb9dccdad4ef9");
        assert_eq!(o.kind, Kind::Tag);
        assert_eq!(o.size, 1024);
        let tag = o.decode().unwrap();
        let expected = borrowed::Tag {
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
-----END PGP SIGNATURE-----
"
                .as_bstr(),
            ),
            signature: Some(signature(1528473343)),
        };
        assert_eq!(tag.as_tag().unwrap(), &expected)
    }

    #[test]
    fn commit() {
        let mut o = locate("ffa700b4aca13b80cb6b98a078e7c96804f8e0ec");
        assert_eq!(o.kind, Kind::Commit);
        assert_eq!(o.size, 1084);
        let expected = borrowed::Commit {
            tree: b"6ba2a0ded519f737fd5b8d5ccfb141125ef3176f".as_bstr(),
            parents: vec![].into(),
            author: signature(1528473303),
            committer: signature(1528473303),
            encoding: None,
            message: b"initial commit\n".as_bstr(),
            extra_headers: vec![(b"gpgsig".as_bstr(), b"-----BEGIN PGP SIGNATURE-----\nComment: GPGTools - https://gpgtools.org\n\niQIzBAABCgAdFiEEw7xSvXbiwjusbsBqZl+Z+p2ZlmwFAlsaptwACgkQZl+Z+p2Z\nlmxXSQ//fj6t7aWoEKeMdFigfj6OXWPUyrRbS0N9kpJeOfA0BIOea/6Jbn8J5qh1\nYRfrySOzHPXR5Y+w4GwLiVas66qyhAbk4yeqZM0JxBjHDyPyRGhjUd3y7WjEa6bj\nP0ACAIkYZQ/Q/LDE3eubmhAwEobBH3nZbwE+/zDIG0i265bD5C0iDumVOiKkSelw\ncr6FZVw1HH+GcabFkeLRZLNGmPqGdbeBwYERqb0U1aRCzV1xLYteoKwyWcYaH8E3\n97z1rwhUO/L7o8WUEJtP3CLB0zuocslMxskf6bCeubBnRNJ0YrRmxGarxCP3vn4D\n3a/MwECnl6mnUU9t+OnfvrzLDN73rlq8iasUq6hGe7Sje7waX6b2UGpxHqwykmXg\nVimD6Ah7svJanHryfJn38DvJW/wOMqmAnSUAp+Y8W9EIe0xVntCmtMyoKuqBoY7T\nJlZ1kHJte6ELIM5JOY9Gx7D0ZCSKZJQqyjoqtl36dsomT0I78/+7QS1DP4S6XB7d\nc3BYH0JkW81p7AAFbE543ttN0Z4wKXErMFqUKnPZUIEuybtlNYV+krRdfDBWQysT\n3MBebjguVQ60oGs06PzeYBosKGQrHggAcwduLFuqXhLTJqN4UQ18RkE0vbtG3YA0\n+XtZQM13vURdfwFI5qitAGgw4EzPVrkWWzApzLCrRPEMbvP+b9A=\n=2qqN\n-----END PGP SIGNATURE-----".as_bstr().into())]
        };
        let object = o.decode().unwrap();
        assert_eq!(object.as_commit().unwrap(), &expected)
    }

    #[test]
    fn blob_stream() {
        let o = locate("37d4e6c5c48ba0d245164c4e10d5f41140cab980");
        assert_eq!(
            o.stream()
                .unwrap()
                .bytes()
                .collect::<Result<Vec<_>, _>>()
                .unwrap()
                .as_slice()
                .as_bstr(),
            b"hi there\n".as_bstr()
        );
    }

    #[test]
    fn blob() {
        let mut o = locate("37d4e6c5c48ba0d245164c4e10d5f41140cab980");
        assert_eq!(
            o.decode().unwrap().as_blob().unwrap(),
            &borrowed::Blob {
                data: &[104, 105, 32, 116, 104, 101, 114, 101, 10]
            },
            "small blobs are treated similarly to other object types and are read into memory at once when the header is read"
        );
    }

    #[test]
    fn blob_not_existing() {
        assert_eq!(try_locate("37d4e6c5c48ba0d245164c4e10d5f41140cab989"), None);
    }

    #[test]
    fn blob_big_stream() {
        let o = locate("a706d7cd20fc8ce71489f34b50cf01011c104193");
        let size = o.size;
        assert_eq!(o.stream().unwrap().bytes().filter_map(Result::ok).count(), size);
    }

    #[test]
    fn blob_big() {
        let mut o = locate("a706d7cd20fc8ce71489f34b50cf01011c104193");
        let size = o.size;
        assert_eq!(
            o.decode().unwrap().as_blob().unwrap().data.len(),
            size,
            "bigger blobs are not read completely when the header is parsed and thus need an extra step"
        );
    }

    fn try_locate(hex: &str) -> Option<loose::Object> {
        ldb().locate(hex_to_id(hex).to_borrowed()).and_then(Result::ok)
    }

    pub fn as_id(id: &[u8; 20]) -> borrowed::Id {
        id.into()
    }

    #[test]
    fn tree() {
        let mut o = locate("6ba2a0ded519f737fd5b8d5ccfb141125ef3176f");
        assert_eq!(o.kind, Kind::Tree);
        assert_eq!(o.size, 66);

        let expected = borrowed::Tree {
            entries: vec![
                tree::Entry {
                    mode: TreeMode::Tree,
                    filename: b"dir".as_bstr(),
                    oid: as_id(&[
                        150, 174, 134, 139, 53, 57, 245, 81, 200, 143, 213, 240, 35, 148, 208, 34, 88, 27, 17, 176,
                    ]),
                },
                tree::Entry {
                    mode: TreeMode::Blob,
                    filename: b"file.txt".as_bstr(),
                    oid: as_id(&[
                        55, 212, 230, 197, 196, 139, 160, 210, 69, 22, 76, 78, 16, 213, 244, 17, 64, 202, 185, 128,
                    ]),
                },
            ],
        };
        let tree = o.decode().unwrap();
        assert_eq!(tree.as_tree().unwrap(), &expected)
    }
}
