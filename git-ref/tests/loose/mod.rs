mod reference {
    mod parse {
        use git_ref::loose::Store;

        fn store() -> Store {
            Store::new("base doesnt matter")
        }

        mod valid {
            use crate::loose::reference::parse::store;
            use bstr::ByteSlice;
            use git_ref::loose::Reference;
            use git_testtools::hex_to_id;

            macro_rules! mktest {
                ($name:ident, $input:literal, $kind:path, $id:expr, $ref:expr) => {
                    #[test]
                    fn $name() {
                        let store = store();
                        let reference = Reference::from_path(&store, "name", $input).unwrap();
                        assert_eq!(reference.kind(), $kind);
                        assert_eq!(reference.target().as_id(), $id);
                        assert_eq!(reference.target().as_ref(), $ref);
                    }
                };
            }

            mktest!(
                peeled,
                b"c5241b835b93af497cda80ce0dceb8f49800df1c\n",
                git_ref::Kind::Peeled,
                Some(hex_to_id("c5241b835b93af497cda80ce0dceb8f49800df1c").as_ref()),
                None
            );

            mktest!(
                symbolic,
                b"ref: refs/heads/main\n",
                git_ref::Kind::Symbolic,
                None,
                Some(b"refs/heads/main".as_bstr())
            );
        }
    }
}
