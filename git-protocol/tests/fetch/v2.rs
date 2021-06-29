use crate::fetch::{oid, transport, CloneRefInWantDelegate, LsRemoteDelegate};
use bstr::ByteSlice;
use git_features::progress;
use git_protocol::fetch;
use git_transport::Protocol;

#[maybe_async::test(feature = "blocking-client", async(feature = "async-client", async_std::test))]
async fn ls_remote() -> crate::Result {
    let out = Vec::new();
    let delegate = LsRemoteDelegate::default();
    let (delegate, out) = git_protocol::fetch(
        transport(
            out,
            "v2/clone.response",
            Protocol::V2,
            git_transport::client::git::ConnectMode::Daemon,
        ),
        delegate,
        git_protocol::credentials::helper,
        progress::Discard,
    )
    .await?;

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
        out.into_inner().1.as_bstr(),
        format!(
            "0044git-upload-pack does/not/matter\0\0version=2\0value-only\0key=value\00014command=ls-refs
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

#[maybe_async::test(feature = "blocking-client", async(feature = "async-client", async_std::test))]
async fn ref_in_want() -> crate::Result {
    let out = Vec::new();
    let delegate = CloneRefInWantDelegate {
        want_refs: vec!["refs/heads/main".into()],
        ..CloneRefInWantDelegate::default()
    };
    let mut transport = transport(
        out,
        "v2/clone-ref-in-want.response",
        Protocol::V2,
        git_transport::client::git::ConnectMode::Daemon,
    );

    let (delegate, _) = git_protocol::fetch(
        &mut transport,
        delegate,
        git_protocol::credentials::helper,
        progress::Discard,
    )
    .await?;

    assert!(delegate.refs.is_empty(), "Should not receive any ref advertisement");
    assert_eq!(
        delegate.wanted_refs,
        vec![fetch::Ref::Direct {
            path: "refs/heads/main".into(),
            object: oid("9e320b9180e0b5580af68fa3255b7f3d9ecd5af0"),
        }]
    );
    assert_eq!(delegate.pack_bytes, 641, "Should get packfile");
    assert_eq!(
        transport.into_inner().1.as_bstr(),
        format!(
            "002fgit-upload-pack does/not/matter\0\0version=2\00012command=fetch
001aagent={}
0001000ethin-pack
0010include-tag
000eofs-delta
001dwant-ref refs/heads/main
0009done
0000",
            fetch::agent().1.expect("value set")
        )
        .as_bytes()
        .as_bstr()
    );

    Ok(())
}
