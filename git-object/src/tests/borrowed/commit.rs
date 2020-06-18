mod parse {
    use crate::{
        borrowed::{commit::parse, commit::Commit},
        tests::{borrowed::fixture_bytes, borrowed::signature},
    };
    use bstr::ByteSlice;

    #[test]
    fn unsigned() {
        assert_eq!(
            parse(&fixture_bytes("commit", "unsigned.txt")).unwrap().1,
            Commit {
                tree: b"1b2dfb4ac5e42080b682fc676e9738c94ce6d54d".as_bstr(),
                parents: vec![],
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
            parse(&fixture_bytes("commit", "whitespace.txt")).unwrap().1,
            Commit {
                tree: b"9bed6275068a0575243ba8409253e61af81ab2ff".as_bstr(),
                parents: vec![b"26b4df046d1776c123ac69d918f5aec247b58cc6".as_bstr()],
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
            parse(&fixture_bytes("commit", "signed-singleline.txt"))
                .unwrap()
                .1,
            Commit {
                tree: b"00fc39317701176e326974ce44f5bd545a32ec0b".as_bstr(),
                parents: vec![b"09d8d3a12e161a7f6afb522dbe8900a9c09bce06".as_bstr()],
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
            parse(&fixture_bytes("commit", "signed.txt")).unwrap().1,
            Commit {
                tree: b"00fc39317701176e326974ce44f5bd545a32ec0b".as_bstr(),
                parents: vec![b"09d8d3a12e161a7f6afb522dbe8900a9c09bce06".as_bstr()],
                author: signature(1592391367),
                committer: signature(1592391367),
                encoding: None,
                message: b"update tasks\n".as_bstr(),
                pgp_signature: Some(b"-----BEGIN PGP SIGNATURE-----\n \n iQEzBAABCAAdFiEEdjYp/sh4j8NRKLX27gKdHl60AwAFAl7p9tgACgkQ7gKdHl60\n AwBpegf+KQciv9AOIN7+yPmowecGxBnSfpKWTDzFxnyGR8dq63SpWT8WEKG5mf3a\n G6iUqpsDWaMHlzihaMKRvgRpZxFRbjnNPFBj6F4RRqfE+5R7k6DRSLUV5PqnsdSH\n uccfIDWi1imhsm7AaP5trwl1t+83U2JhHqPcPVFLMODYwWeO6NLR/JCzGSTQRa8t\n RgaVMKI19O/fge5OT5Ua8D47VKEhsJX0LfmkP5RfZQ8JJvNd40TupqKRdlv0sAzP\n ya7NXkSHXCavHNR6kA+KpWxn900UoGK8/IDlwU6MeOkpPVawb3NFMqnc7KJDaC2p\n SMzpuEG8LTrCx2YSpHNLqHyzvQ1CZA==\n =5ITV\n -----END PGP SIGNATURE-----".as_bstr())
            }
        );
    }

    #[test]
    fn with_encoding() {
        assert_eq!(
            parse(&fixture_bytes("commit", "with-encoding.txt"))
                .unwrap()
                .1,
            Commit {
                tree: b"4a1c03029e7407c0afe9fc0320b3258e188b115e".as_bstr(),
                parents: vec![b"7ca98aad461a5c302cb4c9e3acaaa6053cc67a62".as_bstr()],
                author: signature(1592438199),
                committer: signature(1592438199),
                encoding: Some("ISO-8859-1".into()),
                message: b"commit with encoding".as_bstr(),
                pgp_signature: None
            }
        );
    }
}
