type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

mod file;
mod transaction {
    mod refedit_ext {
        use git_ref::transaction::{Change, DeleteMode, RefEdit, RefEditsExt};
        use std::convert::TryInto;

        fn named_edit(name: &str) -> RefEdit {
            RefEdit {
                change: Change::Delete {
                    previous: None,
                    mode: DeleteMode::RefAndRefLog,
                    deref: false,
                },
                name: name.try_into().expect("valid name"),
            }
        }

        #[test]
        fn reject_duplicates() {
            assert!(
                vec![named_edit("HEAD")].assure_one_name_has_one_edit().is_ok(),
                "there are no duplicates"
            );
            assert!(
                vec![named_edit("refs/foo"), named_edit("HEAD")]
                    .assure_one_name_has_one_edit()
                    .is_ok(),
                "there are no duplicates"
            );
            assert_eq!(
                vec![named_edit("HEAD"), named_edit("refs/heads/main"), named_edit("HEAD")]
                    .assure_one_name_has_one_edit()
                    .expect_err("duplicate"),
                "HEAD",
                "a correctly named duplicate"
            );
        }

        mod splitting {
            #[test]
            #[ignore]
            fn non_symbolic_refs_are_ignored_but_derefs_are_fixed() {}

            #[test]
            #[ignore]
            fn symbolic_refs_are_split_into_referents_handling_the_reflog() {}
        }
    }
}
