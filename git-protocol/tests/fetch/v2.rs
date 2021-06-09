use crate::fetch::{oid, transport, LsRemoteDelegate};
use bstr::ByteSlice;
use git_features::progress;
use git_protocol::fetch;
use git_transport::Protocol;

#[maybe_async::test(feature = "blocking-client", async(feature = "async-client", async_std::test))]
async fn ls_remote() -> crate::Result {
    let out = Vec::new();
    let delegate = LsRemoteDelegate::default();
    let (delegate, out) = git_protocol::fetch(
        transport(out, "v2/clone.response", Protocol::V2),
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
