mod reference {
    mod parse {
        mod valid {
            use git_ref::loose::{Reference, Store};
            use git_testtools::hex_to_id;

            #[test]
            fn peeled() {
                let store = Store::new("doesnt matter");
                let reference =
                    Reference::from_path(&store, "name", b"c5241b835b93af497cda80ce0dceb8f49800df1c\n").unwrap();
                assert_eq!(reference.kind(), git_ref::Kind::Peeled);
                assert_eq!(
                    reference.target(),
                    Some(hex_to_id("c5241b835b93af497cda80ce0dceb8f49800df1c").as_ref())
                );
            }
        }
    }
}
