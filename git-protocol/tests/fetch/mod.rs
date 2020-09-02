use git_protocol::fetch;

struct CloneDelegate;
impl fetch::Delegate for CloneDelegate {}

#[derive(Default)]
struct LsRemoteDelegate {
    refs: Vec<fetch::Ref>,
}

impl fetch::Delegate for LsRemoteDelegate {
    fn prepare_fetch(
        &mut self,
        _version: git_transport::Protocol,
        _server: &fetch::Capabilities,
        _features: &mut Vec<&str>,
        refs: &[fetch::Ref],
    ) -> fetch::Action {
        self.refs = refs.to_owned();
        fetch::Action::Close
    }
}

mod v1 {
    use crate::{
        fetch::{CloneDelegate, LsRemoteDelegate},
        fixture_bytes,
    };
    use bstr::ByteSlice;
    use git_object::owned;
    use git_protocol::fetch;

    #[test]
    #[ignore]
    fn clone() -> crate::Result {
        let mut out = Vec::new();
        git_protocol::fetch(
            transport(&mut out, "v1/clone.response"),
            &mut CloneDelegate,
            git_protocol::credentials::helper,
        )?;
        Ok(())
    }

    #[test]
    fn ls_remote() -> crate::Result {
        let mut out = Vec::new();
        let mut delegate = LsRemoteDelegate::default();
        git_protocol::fetch(
            transport(&mut out, "v1/clone.response"),
            &mut delegate,
            git_protocol::credentials::helper,
        )?;

        assert_eq!(
            delegate.refs,
            vec![
                fetch::Ref::Symbolic {
                    path: "HEAD".into(),
                    object: oid("808e50d724f604f69ab93c6da2919c014667bedb"),
                    target: "refs/heads/master".into()
                },
                fetch::Ref::Direct {
                    path: "refs/heads/master".into(),
                    object: oid("808e50d724f604f69ab93c6da2919c014667bedb")
                }
            ]
        );
        // multi_ack thin-pack side-band side-band-64k ofs-delta shallow deepen-since deepen-not deepen-relative no-progress include-tag multi_ack_detailed symref=HEAD:refs/heads/master object-format=sha1 agent=git/2.28.0
        assert!(
            out.is_empty(),
            "we dont have to send anything in V1, everything happens 'by default'"
        );
        Ok(())
    }

    fn oid(hex_sha: &str) -> owned::Id {
        owned::Id::from_40_bytes_in_hex(hex_sha.as_bytes()).expect("valid input")
    }

    fn transport<'a>(
        out: &'a mut Vec<u8>,
        path: &str,
    ) -> git_transport::client::git::Connection<std::io::Cursor<Vec<u8>>, &'a mut Vec<u8>> {
        let response = fixture_bytes(path);
        git_transport::client::git::Connection::new(
            std::io::Cursor::new(response),
            out,
            git_transport::Protocol::V1,
            b"does/not/matter".as_bstr().to_owned(),
            None::<(&str, _)>,
            git_transport::client::git::ConnectMode::Process,
        )
    }
}
