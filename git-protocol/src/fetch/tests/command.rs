mod v1 {
    fn capabilities_from_v1(input: &str) -> git_transport::client::Capabilities {
        git_transport::client::Capabilities::from_bytes(format!("\0{}", input).as_bytes())
            .expect("valid input capabilities")
            .0
    }

    mod ls_refs {
        mod collect_initial_features {
            use crate::{
                fetch,
                fetch::{tests::command::v1::capabilities_from_v1, Command},
            };

            #[test]
            fn with_simrefs_in_capabilities() {
                assert_eq!(
                    Command::LsRefs
                        .collect_initial_features(
                            git_transport::Protocol::V1,
                            &capabilities_from_v1("symref=HEAD:refs/heads/master object-format=sha1 agent=git/2.28.0")
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
                            &capabilities_from_v1("object-format=sha1 agent=git/2.28.0")
                        )
                        .collect::<Vec<_>>(),
                    &[("peel", None), fetch::agent()]
                );
            }
        }
    }
}
