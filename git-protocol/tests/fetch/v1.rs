use crate::fetch::{oid, transport, CloneDelegate, LsRemoteDelegate};
use bstr::ByteSlice;
use git_features::progress;
use git_protocol::fetch;
use git_transport::Protocol;

#[maybe_async::test(feature = "blocking-client", async(feature = "async-client", async_std::test))]
async fn clone() -> crate::Result {
    let out = Vec::new();
    let dlg = CloneDelegate::default();
    let dlg = git_protocol::fetch(
        transport(out, "v1/clone.response", Protocol::V1),
        dlg,
        git_protocol::credentials::helper,
        progress::Discard,
    )
    .await?
    .0;
    assert_eq!(dlg.pack_bytes, 876, "It be able to read pack bytes");
    Ok(())
}

#[maybe_async::test(feature = "blocking-client", async(feature = "async-client", async_std::test))]
async fn ls_remote() -> crate::Result {
    let out = Vec::new();
    let delegate = LsRemoteDelegate::default();
    let (delegate, out) = git_protocol::fetch(
        transport(out, "v1/clone.response", Protocol::V1),
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
        b"0000".as_bstr(),
        "we dont have to send anything in V1, except for the final flush byte to indicate we are done"
    );
    Ok(())
}
