mod method {
    use crate::{borrowed::fixture_bytes, hex_to_id};
    use git_object::borrowed::Commit;
    use pretty_assertions::assert_eq;

    #[test]
    fn tree() {
        let fixture = fixture_bytes("commit", "unsigned.txt");
        let commit = Commit::from_bytes(&fixture).unwrap();
        assert_eq!(commit.tree(), hex_to_id("1b2dfb4ac5e42080b682fc676e9738c94ce6d54d"));
        assert_eq!(commit.tree, "1b2dfb4ac5e42080b682fc676e9738c94ce6d54d");
    }
}

mod from_bytes {
    use crate::{borrowed::fixture_bytes, borrowed::signature};
    use bstr::ByteSlice;
    use git_object::borrowed::Commit;
    use smallvec::SmallVec;

    #[test]
    fn unsigned() {
        assert_eq!(
            Commit::from_bytes(&fixture_bytes("commit", "unsigned.txt")).unwrap(),
            Commit {
                tree: b"1b2dfb4ac5e42080b682fc676e9738c94ce6d54d".as_bstr(),
                parents: SmallVec::default(),
                author: signature(1592437401),
                committer: signature(1592437401),
                encoding: None,
                message: b"without sig".as_bstr(),
                pgp_signature: None
            }
        );
    }

    #[test]
    fn whitespace() {
        assert_eq!(
            Commit::from_bytes(&fixture_bytes("commit", "whitespace.txt")).unwrap(),
            Commit {
                tree: b"9bed6275068a0575243ba8409253e61af81ab2ff".as_bstr(),
                parents: SmallVec::from(vec![b"26b4df046d1776c123ac69d918f5aec247b58cc6".as_bstr()]),
                author: signature(1592448450),
                committer: signature(1592448450),
                encoding: None,
                message: b" nl".as_bstr(), // this one had a \n trailing it, but git seems to trim that
                pgp_signature: None
            }
        );
    }

    #[test]
    fn signed_singleline() {
        assert_eq!(
            Commit::from_bytes(&fixture_bytes("commit", "signed-singleline.txt")).unwrap(),
            Commit {
                tree: b"00fc39317701176e326974ce44f5bd545a32ec0b".as_bstr(),
                parents: SmallVec::from(vec![b"09d8d3a12e161a7f6afb522dbe8900a9c09bce06".as_bstr()]),
                author: signature(1592391367),
                committer: signature(1592391367),
                encoding: None,
                message: b"update tasks\n".as_bstr(),
                pgp_signature: Some(b"magic:signature".as_bstr())
            }
        );
    }

    #[test]
    fn signed() {
        assert_eq!(
            Commit::from_bytes(&fixture_bytes("commit", "signed.txt")).unwrap(),
            Commit {
                tree: b"00fc39317701176e326974ce44f5bd545a32ec0b".as_bstr(),
                parents: SmallVec::from(vec![b"09d8d3a12e161a7f6afb522dbe8900a9c09bce06".as_bstr()]),
                author: signature(1592391367),
                committer: signature(1592391367),
                encoding: None,
                message: b"update tasks\n".as_bstr(),
                pgp_signature: Some(b"-----BEGIN PGP SIGNATURE-----\n \n iQEzBAABCAAdFiEEdjYp/sh4j8NRKLX27gKdHl60AwAFAl7p9tgACgkQ7gKdHl60\n AwBpegf+KQciv9AOIN7+yPmowecGxBnSfpKWTDzFxnyGR8dq63SpWT8WEKG5mf3a\n G6iUqpsDWaMHlzihaMKRvgRpZxFRbjnNPFBj6F4RRqfE+5R7k6DRSLUV5PqnsdSH\n uccfIDWi1imhsm7AaP5trwl1t+83U2JhHqPcPVFLMODYwWeO6NLR/JCzGSTQRa8t\n RgaVMKI19O/fge5OT5Ua8D47VKEhsJX0LfmkP5RfZQ8JJvNd40TupqKRdlv0sAzP\n ya7NXkSHXCavHNR6kA+KpWxn900UoGK8/IDlwU6MeOkpPVawb3NFMqnc7KJDaC2p\n SMzpuEG8LTrCx2YSpHNLqHyzvQ1CZA==\n =5ITV\n -----END PGP SIGNATURE-----".as_bstr())
            }
        );
    }

    #[test]
    fn signed_with_encoding() {
        assert_eq!(
            Commit::from_bytes(&fixture_bytes("commit", "signed-with-encoding.txt")).unwrap(),
            Commit {
                tree: b"1973afa74d87b2bb73fa884aaaa8752aec43ea88".as_bstr(),
                parents: SmallVec::from(vec![b"79c51cc86923e2b8ca0ee5c4eb75e48027133f9a".as_bstr()]),
                author: signature(1592448995),
                committer: signature(1592449083),
                encoding: Some(b"ISO-8859-1".as_bstr()),
                message: b"encoding & sig".as_bstr(),
                pgp_signature: Some(b"-----BEGIN PGP SIGNATURE-----\n \n iQEzBAABCAAdFiEEdjYp/sh4j8NRKLX27gKdHl60AwAFAl7q2DsACgkQ7gKdHl60\n AwDvewgAkL5UjEztzeVXlzceom0uCrAkCw9wSGLTmYcMKW3JwEaTRgQ4FX+sDuFT\n LZ8DoPu3UHUP0QnKrUwHulTTlKcOAvsczHbVPIKtXCxo6QpUfhsJQwz/J29kiE4L\n sOd+lqKGnn4oati/de2xwqNGi081fO5KILX75z6KfsAe7Qz7R3jxRF4uzHI033O+\n Jc2Y827XeaELxW40SmzoLanWgEcdreXf3PstXEWW77CAu0ozXmvYj56vTviVybxx\n G7bc8lwc+SSKVe2VVB+CCfVbs0i541gmghUpZfMhUgaqttcCH8ysrUJDhne1BLG8\n CrOJIWTwAeEDtomV1p76qrMeqr1GFg==\n =qlSN\n -----END PGP SIGNATURE-----".as_bstr())
            }
        );
    }

    #[test]
    fn with_encoding() {
        assert_eq!(
            Commit::from_bytes(&fixture_bytes("commit", "with-encoding.txt")).unwrap(),
            Commit {
                tree: b"4a1c03029e7407c0afe9fc0320b3258e188b115e".as_bstr(),
                parents: SmallVec::from(vec![b"7ca98aad461a5c302cb4c9e3acaaa6053cc67a62".as_bstr()]),
                author: signature(1592438199),
                committer: signature(1592438199),
                encoding: Some("ISO-8859-1".into()),
                message: b"commit with encoding".as_bstr(),
                pgp_signature: None
            }
        );
    }

    #[test]
    fn merge() {
        assert_eq!(
            Commit::from_bytes(&fixture_bytes("commit", "merge.txt")).unwrap(),
            Commit {
                tree: b"0cf16ce8e229b59a761198975f0c0263229faf82".as_bstr(),
                parents: SmallVec::from(vec![
                    b"6a6054db4ce3c1e4e6a37f8c4d7acb63a4d6ad71".as_bstr(),
                    b"c91d592913d47ac4e4a76daf16fd649b276e211e".as_bstr()
                ]),
                author: signature(1592454703),
                committer: signature(1592454738),
                encoding: Some("ISO-8859-1".into()),
                message: b"Merge branch 'branch'".as_bstr(),
                pgp_signature: None
            }
        );
    }
}
