mod refs {
    use super::super::{refs, Capabilities, Ref};
    use git_object::owned;
    use git_transport::client;
    use std::{convert::TryInto, io};

    fn oid(hex_sha: &str) -> owned::Id {
        owned::Id::from_40_bytes_in_hex(hex_sha.as_bytes()).expect("valid input")
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
        let (caps, _) = client::Capabilities::from_bytes(
            b"\0unrelated symref=HEAD:refs/heads/main symref=ANOTHER:refs/heads/foo agent=git/2.28.0",
        )?;
        let mut caps: Capabilities = caps.try_into().expect("this is a working example");
        let mut out = Vec::new();
        refs::from_capabilities(&mut out, std::mem::take(&mut caps.symrefs)).expect("a working example");

        assert_eq!(
            caps.available.into_iter().collect::<Vec<_>>(),
            vec![("agent".into(), Some("git/2.28.0".into())), ("unrelated".into(), None)]
        );
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
