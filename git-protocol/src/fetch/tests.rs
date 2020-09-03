mod command {
    mod v1 {
        fn capabilities_from_v1(input: &str) -> git_transport::client::Capabilities {
            git_transport::client::Capabilities::from_bytes(format!("\0{}", input).as_bytes())
                .expect("valid input capabilities")
                .0
        }

        const ALL_FEATURES: &str = "symref=HEAD:refs/heads/master object-format=sha1 agent=git/2.28.0";

        mod ls_refs {
            use crate::fetch::{
                tests::command::v1::{capabilities_from_v1, ALL_FEATURES},
                Command,
            };

            #[test]
            fn collect_initial_features() {
                assert_eq!(
                    Command::LsRefs
                        .collect_initial_features(git_transport::Protocol::V1, &capabilities_from_v1(ALL_FEATURES)),
                    &[
                        ("symrefs", None),
                        ("peel", None),
                        ("agent", Some(concat!("git/oxide-", env!("CARGO_PKG_VERSION"))))
                    ]
                );
            }
        }
    }
}

mod refs {
    use super::super::{refs, Ref};
    use git_object::owned;
    use git_transport::client;
    use std::io;

    fn oid(hex_sha: &str) -> owned::Id {
        owned::Id::from_40_bytes_in_hex(hex_sha.as_bytes()).expect("valid input")
    }

    #[test]
    fn extract_references_from_v2_refs() {
        let input: &mut dyn io::BufRead =
            &mut "808e50d724f604f69ab93c6da2919c014667bedb HEAD symref-target:refs/heads/main
808e50d724f604f69ab93c6da2919c014667bedb refs/heads/main
7fe1b98b39423b71e14217aa299a03b7c937d656 refs/tags/foo peeled:808e50d724f604f69ab93c6da2919c014667bedb
7fe1b98b39423b71e14217aa299a03b7c937d6ff refs/tags/blaz
"
            .as_bytes();
        let mut out = Vec::new();
        refs::from_v2_refs(&mut out, input).expect("no failure on valid input");

        assert_eq!(
            out,
            vec![
                Ref::Symbolic {
                    path: "HEAD".into(),
                    target: "refs/heads/main".into(),
                    object: oid("808e50d724f604f69ab93c6da2919c014667bedb")
                },
                Ref::Direct {
                    path: "refs/heads/main".into(),
                    object: oid("808e50d724f604f69ab93c6da2919c014667bedb")
                },
                Ref::Peeled {
                    path: "refs/tags/foo".into(),
                    tag: oid("7fe1b98b39423b71e14217aa299a03b7c937d656"),
                    object: oid("808e50d724f604f69ab93c6da2919c014667bedb")
                },
                Ref::Direct {
                    path: "refs/tags/blaz".into(),
                    object: oid("7fe1b98b39423b71e14217aa299a03b7c937d6ff")
                },
            ]
        )
    }

    #[test]
    fn extract_references_from_v1_refs() {
        let input: &mut dyn io::BufRead = &mut "73a6868963993a3328e7d8fe94e5a6ac5078a944 HEAD
73a6868963993a3328e7d8fe94e5a6ac5078a944 refs/heads/main
8e472f9ccc7d745927426cbb2d9d077de545aa4e refs/pull/13/head
dce0ea858eef7ff61ad345cc5cdac62203fb3c10 refs/tags/git-commitgraph-v0.0.0
21c9b7500cb144b3169a6537961ec2b9e865be81 refs/tags/git-commitgraph-v0.0.0^{}"
            .as_bytes();
        let mut out = vec![Ref::SymbolicForLookup {
            path: "HEAD".into(),
            target: "refs/heads/main".into(),
        }];
        refs::from_v1_refs_received_as_part_of_handshake(&mut out, input).expect("no failure from valid input");
        assert_eq!(
            out,
            vec![
                Ref::Symbolic {
                    path: "HEAD".into(),
                    target: "refs/heads/main".into(),
                    object: oid("73a6868963993a3328e7d8fe94e5a6ac5078a944")
                },
                Ref::Direct {
                    path: "refs/heads/main".into(),
                    object: oid("73a6868963993a3328e7d8fe94e5a6ac5078a944")
                },
                Ref::Direct {
                    path: "refs/pull/13/head".into(),
                    object: oid("8e472f9ccc7d745927426cbb2d9d077de545aa4e")
                },
                Ref::Peeled {
                    path: "refs/tags/git-commitgraph-v0.0.0".into(),
                    tag: oid("dce0ea858eef7ff61ad345cc5cdac62203fb3c10"),
                    object: oid("21c9b7500cb144b3169a6537961ec2b9e865be81")
                }
            ]
        )
    }

    #[test]
    fn extract_symbolic_references_from_capabilities() -> Result<(), client::Error> {
        let caps = client::Capabilities::from_bytes(
            b"\0unrelated symref=HEAD:refs/heads/main symref=ANOTHER:refs/heads/foo agent=git/2.28.0",
        )?
        .0;
        let mut out = Vec::new();
        refs::from_capabilities(&mut out, caps.iter()).expect("a working example");

        assert_eq!(
            out,
            vec![
                Ref::SymbolicForLookup {
                    path: "HEAD".into(),
                    target: "refs/heads/main".into()
                },
                Ref::SymbolicForLookup {
                    path: "ANOTHER".into(),
                    target: "refs/heads/foo".into()
                }
            ]
        );
        Ok(())
    }
}
