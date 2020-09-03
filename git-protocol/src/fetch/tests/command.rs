fn capabilities(input: &str) -> git_transport::client::Capabilities {
    git_transport::client::Capabilities::from_bytes(format!("\0{}", input).as_bytes())
        .expect("valid input capabilities")
        .0
}

mod v1 {
    mod ls_refs {
        mod validate_arguments {
            use crate::{fetch::tests::command::capabilities, fetch::Command};
            use bstr::ByteSlice;

            #[test]
            #[should_panic]
            fn ref_prefixes_cannot_be_used() {
                Command::LsRefs.validate_argument_prefixes_or_panic(
                    git_transport::Protocol::V1,
                    &capabilities("do-not-matter"),
                    &[b"ref-prefix hello/".as_bstr().into()],
                    &[],
                );
            }
        }
        mod collect_initial_features {
            use crate::{
                fetch,
                fetch::{tests::command::capabilities, Command},
            };

            #[test]
            fn with_simrefs_in_capabilities() {
                assert_eq!(
                    Command::LsRefs
                        .collect_initial_features(
                            git_transport::Protocol::V1,
                            &capabilities("symref=HEAD:refs/heads/master object-format=sha1 agent=git/2.28.0")
                        )
                        .collect::<Vec<_>>(),
                    &[("symrefs", None), ("peel", None), fetch::agent()]
                );
            }

            #[test]
            fn without_simrefs_in_capabilities() {
                assert_eq!(
                    Command::LsRefs
                        .collect_initial_features(
                            git_transport::Protocol::V1,
                            &capabilities("object-format=sha1 agent=git/2.28.0")
                        )
                        .collect::<Vec<_>>(),
                    &[("peel", None), fetch::agent()]
                );
            }
        }
    }
}

mod v2 {
    mod ls_refs {
        mod validate_arguments {
            use crate::{fetch::tests::command::capabilities, fetch::Command};
            use bstr::ByteSlice;

            #[test]
            fn ref_prefixes_always_be_used() {
                Command::LsRefs.validate_argument_prefixes_or_panic(
                    git_transport::Protocol::V2,
                    &capabilities("do-not-matter"),
                    &[b"ref-prefix hello/".as_bstr().into()],
                    &[],
                );
            }
        }
    }
}
