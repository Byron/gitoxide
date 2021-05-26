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

            #[test]
            fn peeled() {
                let store = store();
                let reference =
                    Reference::from_path(&store, "name", b"c5241b835b93af497cda80ce0dceb8f49800df1c\n").unwrap();
                assert_eq!(reference.kind(), git_ref::Kind::Peeled);
                assert_eq!(
                    reference.target().as_id(),
                    Some(hex_to_id("c5241b835b93af497cda80ce0dceb8f49800df1c").as_ref())
                );
                assert_eq!(reference.target().as_ref(), None);
            }

            #[test]
            fn symbolic() {
                let store = store();
                let reference = Reference::from_path(&store, "name", b"ref: refs/heads/main\n").unwrap();
                assert_eq!(reference.kind(), git_ref::Kind::Symbolic);
                assert_eq!(reference.target().as_id(), None);
                assert_eq!(reference.target().as_ref(), Some(b"refs/heads/main".as_bstr()));
            }
        }
    }
}
