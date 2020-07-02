use git_object::{
    borrowed::{Signature, Tag},
    ByteSlice, Kind, Sign, Time,
};

mod method {
    use crate::{borrowed::fixture_bytes, hex_to_id};
    use git_object::borrowed::Tag;
    use pretty_assertions::assert_eq;

    #[test]
    fn target() {
        let fixture = fixture_bytes("tag", "signed.txt");
        let tag = Tag::from_bytes(&fixture).unwrap();
        assert_eq!(tag.target(), hex_to_id("ffa700b4aca13b80cb6b98a078e7c96804f8e0ec"));
        assert_eq!(tag.target, "ffa700b4aca13b80cb6b98a078e7c96804f8e0ec".as_bytes())
    }
}

mod from_bytes {
    use crate::{borrowed::fixture_bytes, borrowed::signature, borrowed::tag::tag_fixture};
    use git_object::{borrowed::Tag, ByteSlice, Kind};

    #[test]
    fn signed() {
        assert_eq!(
            Tag::from_bytes(&fixture_bytes("tag", "signed.txt")).unwrap(),
            tag_fixture(9000)
        );
    }

    #[test]
    fn empty() {
        assert_eq!(
            Tag::from_bytes(&fixture_bytes("tag", "empty.txt")).unwrap(),
            Tag {
                target: b"01dd4e2a978a9f5bd773dae6da7aa4a5ac1cdbbc".as_bstr(),
                name: b"empty".as_bstr(),
                target_kind: Kind::Commit,
                message: b"".as_bstr(),
                signature: signature(1592381636),
                pgp_signature: None
            }
        );
    }

    #[test]
    fn with_newlines() {
        assert_eq!(
            Tag::from_bytes(&fixture_bytes("tag", "with-newlines.txt")).unwrap(),
            Tag {
                target: b"ebdf205038b66108c0331aa590388431427493b7".as_bstr(),
                name: b"baz".as_bstr(),
                target_kind: Kind::Commit,
                message: b"hello\n\nworld".as_bstr(),
                signature: signature(1592311808),
                pgp_signature: None
            }
        );
    }

    #[test]
    fn whitespace() {
        assert_eq!(
            Tag::from_bytes(&fixture_bytes("tag", "whitespace.txt")).unwrap(),
            Tag {
                target: b"01dd4e2a978a9f5bd773dae6da7aa4a5ac1cdbbc".as_bstr(),
                name: b"whitespace".as_bstr(),
                target_kind: Kind::Commit,
                message: b" \ttab\nnewline\n\nlast-with-trailer\n".as_bstr(), // odd, was created with \n\n actually
                signature: signature(1592382888),
                pgp_signature: None
            }
        );
    }
}

fn tag_fixture(offset: i32) -> Tag<'static> {
    Tag {
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
        signature: Signature {
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
