use crate::fixture_bytes;
use bstr::ByteSlice;
use git_object::owned;
use git_protocol::fetch::{self, Action, Arguments, Ref, Response};
use git_transport::client::Capabilities;

mod response;

struct CloneDelegate;
impl fetch::Delegate for CloneDelegate {
    fn negotiate(&mut self, refs: &[Ref], arguments: &mut Arguments, _previous_result: Option<&Response>) -> Action {
        for r in refs {
            arguments.want(r.unpack_common().1.to_borrowed());
        }
        Action::Close
    }
}

#[derive(Default)]
struct LsRemoteDelegate {
    refs: Vec<fetch::Ref>,
}

impl fetch::Delegate for LsRemoteDelegate {
    fn prepare_fetch(
        &mut self,
        _version: git_transport::Protocol,
        _server: &Capabilities,
        _features: &mut Vec<(&str, Option<&str>)>,
        refs: &[fetch::Ref],
    ) -> fetch::Action {
        self.refs = refs.to_owned();
        fetch::Action::Close
    }

    fn negotiate(&mut self, _refs: &[Ref], _arguments: &mut Arguments, _previous_result: Option<&Response>) -> Action {
        unreachable!("this must not be called after closing the connection in `prepare_fetch(…)`")
    }
}

fn oid(hex_sha: &str) -> owned::Id {
    owned::Id::from_40_bytes_in_hex(hex_sha.as_bytes()).expect("valid input")
}

fn transport<'a>(
    out: &'a mut Vec<u8>,
    path: &str,
    version: git_transport::Protocol,
) -> git_transport::client::git::Connection<std::io::Cursor<Vec<u8>>, &'a mut Vec<u8>> {
    let response = fixture_bytes(path);
    git_transport::client::git::Connection::new(
        std::io::Cursor::new(response),
        out,
        version,
        b"does/not/matter".as_bstr().to_owned(),
        None::<(&str, _)>,
        git_transport::client::git::ConnectMode::Process,
    )
}

mod v1 {
    use crate::fetch::{oid, transport, CloneDelegate, LsRemoteDelegate};
    use bstr::ByteSlice;
    use git_features::progress;
    use git_protocol::fetch;
    use git_transport::Protocol;

    #[test]
    fn clone() -> crate::Result {
        let mut out = Vec::new();
        git_protocol::fetch(
            transport(&mut out, "v1/clone.response", Protocol::V1),
            &mut CloneDelegate,
            git_protocol::credentials::helper,
            progress::Discard,
        )?;
        Ok(())
    }

    #[test]
    fn ls_remote() -> crate::Result {
        let mut out = Vec::new();
        let mut delegate = LsRemoteDelegate::default();
        git_protocol::fetch(
            transport(&mut out, "v1/clone.response", Protocol::V1),
            &mut delegate,
            git_protocol::credentials::helper,
            progress::Discard,
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
        assert_eq!(
            out.as_bstr(),
            b"0000".as_bstr(),
            "we dont have to send anything in V1, except for the final flush byte to indicate we are done"
        );
        Ok(())
    }
}

mod v2 {
    use crate::fetch::{oid, transport, LsRemoteDelegate};
    use bstr::ByteSlice;
    use git_features::progress;
    use git_protocol::fetch;
    use git_transport::Protocol;

    #[test]
    fn ls_remote() -> crate::Result {
        let mut out = Vec::new();
        let mut delegate = LsRemoteDelegate::default();
        git_protocol::fetch(
            transport(&mut out, "v2/clone.response", Protocol::V2),
            &mut delegate,
            git_protocol::credentials::helper,
            progress::Discard,
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
        assert_eq!(
            out.as_bstr(),
            format!(
                "0014command=ls-refs
001aagent={}
0001000csymrefs
0009peel
00000000",
                fetch::agent().1.expect("value set")
            )
            .as_bytes()
            .as_bstr()
        );
        Ok(())
    }
}
