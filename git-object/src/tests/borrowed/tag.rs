use crate::{
    borrowed::{Signature, Tag},
    Kind, Sign, Time,
};
use bstr::ByteSlice;

mod method {
    use crate::tests::bin;
    use crate::tests::borrowed::tag::tag_fixture;
    use pretty_assertions::assert_eq;

    #[test]
    fn target() {
        assert_eq!(
            tag_fixture(9000).target(),
            bin("ffa700b4aca13b80cb6b98a078e7c96804f8e0ec")
        )
    }
}

mod parse_tag {
    use crate::borrowed::tag::parse_tag;
    use crate::tests::borrowed::fixture_bytes;
    use crate::tests::borrowed::tag::tag_fixture;

    #[test]
    fn signed() {
        let fixture = fixture_bytes("signed.txt");
        assert_eq!(parse_tag(&fixture).unwrap().1, tag_fixture(9000));
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
