mod parse {
    use crate::borrowed::commit::Commit;
    use crate::tests::bin;
    use crate::tests::borrowed::signature;
    use crate::{borrowed::commit::parse, tests::borrowed::fixture_bytes};
    use bstr::ByteSlice;

    #[test]
    fn signed() {
        assert_eq!(
            parse(&fixture_bytes("commit", "unsigned.txt")).unwrap().1,
            Commit {
                tree: bin("1b2dfb4ac5e42080b682fc676e9738c94ce6d54d"),
                parents: vec![],
                author: signature(1592437401),
                committer: signature(1592437401),
                encoding: None,
                message: b"without sig".as_bstr(),
                pgp_signature: None
            }
        );
    }
}
