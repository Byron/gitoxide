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
    #[ignore]
    fn with_encoding() {
        assert_eq!(
            parse(&fixture_bytes("commit", "with-encoding.txt"))
                .unwrap()
                .1,
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
}
