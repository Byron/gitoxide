use git_hash::hex_to_id as oid;
use git_transport::{client, client::Capabilities};

use crate::handshake::{refs, refs::shared::InternalRef, Ref};

#[maybe_async::test(feature = "blocking-client", async(feature = "async-client", async_std::test))]
async fn extract_references_from_v2_refs() {
    let input = &mut Fixture(
        "808e50d724f604f69ab93c6da2919c014667bedb HEAD symref-target:refs/heads/main
808e50d724f604f69ab93c6da2919c014667bedb MISSING_NAMESPACE_TARGET symref-target:(null)
unborn HEAD symref-target:refs/heads/main
unborn refs/heads/symbolic symref-target:refs/heads/target
808e50d724f604f69ab93c6da2919c014667bedb refs/heads/main
7fe1b98b39423b71e14217aa299a03b7c937d656 refs/tags/foo peeled:808e50d724f604f69ab93c6da2919c014667bedb
7fe1b98b39423b71e14217aa299a03b7c937d6ff refs/tags/blaz
"
        .as_bytes(),
    );

    let out = refs::from_v2_refs(input).await.expect("no failure on valid input");

    assert_eq!(
        out,
        vec![
            Ref::Symbolic {
                full_ref_name: "HEAD".into(),
                target: "refs/heads/main".into(),
                object: oid("808e50d724f604f69ab93c6da2919c014667bedb")
            },
            Ref::Direct {
                full_ref_name: "MISSING_NAMESPACE_TARGET".into(),
                object: oid("808e50d724f604f69ab93c6da2919c014667bedb")
            },
            Ref::Unborn {
                full_ref_name: "HEAD".into(),
                target: "refs/heads/main".into(),
            },
            Ref::Unborn {
                full_ref_name: "refs/heads/symbolic".into(),
                target: "refs/heads/target".into(),
            },
            Ref::Direct {
                full_ref_name: "refs/heads/main".into(),
                object: oid("808e50d724f604f69ab93c6da2919c014667bedb")
            },
            Ref::Peeled {
                full_ref_name: "refs/tags/foo".into(),
                tag: oid("7fe1b98b39423b71e14217aa299a03b7c937d656"),
                object: oid("808e50d724f604f69ab93c6da2919c014667bedb")
            },
            Ref::Direct {
                full_ref_name: "refs/tags/blaz".into(),
                object: oid("7fe1b98b39423b71e14217aa299a03b7c937d6ff")
            },
        ]
    )
}

#[maybe_async::test(feature = "blocking-client", async(feature = "async-client", async_std::test))]
async fn extract_references_from_v1_refs() {
    let input = &mut Fixture(
        "73a6868963993a3328e7d8fe94e5a6ac5078a944 HEAD
21c9b7500cb144b3169a6537961ec2b9e865be81 MISSING_NAMESPACE_TARGET
73a6868963993a3328e7d8fe94e5a6ac5078a944 refs/heads/main
8e472f9ccc7d745927426cbb2d9d077de545aa4e refs/pull/13/head
dce0ea858eef7ff61ad345cc5cdac62203fb3c10 refs/tags/git-commitgraph-v0.0.0
21c9b7500cb144b3169a6537961ec2b9e865be81 refs/tags/git-commitgraph-v0.0.0^{}"
            .as_bytes(),
    );
    let out = refs::from_v1_refs_received_as_part_of_handshake_and_capabilities(
        input,
        Capabilities::from_bytes(b"\0symref=HEAD:refs/heads/main symref=MISSING_NAMESPACE_TARGET:(null)")
            .expect("valid capabilities")
            .0
            .iter(),
    )
    .await
    .expect("no failure from valid input");
    assert_eq!(
        out,
        vec![
            Ref::Symbolic {
                full_ref_name: "HEAD".into(),
                target: "refs/heads/main".into(),
                object: oid("73a6868963993a3328e7d8fe94e5a6ac5078a944")
            },
            Ref::Direct {
                full_ref_name: "MISSING_NAMESPACE_TARGET".into(),
                object: oid("21c9b7500cb144b3169a6537961ec2b9e865be81")
            },
            Ref::Direct {
                full_ref_name: "refs/heads/main".into(),
                object: oid("73a6868963993a3328e7d8fe94e5a6ac5078a944")
            },
            Ref::Direct {
                full_ref_name: "refs/pull/13/head".into(),
                object: oid("8e472f9ccc7d745927426cbb2d9d077de545aa4e")
            },
            Ref::Peeled {
                full_ref_name: "refs/tags/git-commitgraph-v0.0.0".into(),
                tag: oid("dce0ea858eef7ff61ad345cc5cdac62203fb3c10"),
                object: oid("21c9b7500cb144b3169a6537961ec2b9e865be81")
            },
        ]
    )
}

#[test]
fn extract_symbolic_references_from_capabilities() -> Result<(), client::Error> {
    let caps = client::Capabilities::from_bytes(
        b"\0unrelated symref=HEAD:refs/heads/main symref=ANOTHER:refs/heads/foo symref=MISSING_NAMESPACE_TARGET:(null) agent=git/2.28.0",
    )?
        .0;
    let out = refs::shared::from_capabilities(caps.iter()).expect("a working example");

    assert_eq!(
        out,
        vec![
            InternalRef::SymbolicForLookup {
                path: "HEAD".into(),
                target: Some("refs/heads/main".into())
            },
            InternalRef::SymbolicForLookup {
                path: "ANOTHER".into(),
                target: Some("refs/heads/foo".into())
            },
            InternalRef::SymbolicForLookup {
                path: "MISSING_NAMESPACE_TARGET".into(),
                target: None
            }
        ]
    );
    Ok(())
}

#[cfg(any(feature = "async-client", feature = "blocking-client"))]
struct Fixture<'a>(&'a [u8]);

#[cfg(feature = "blocking-client")]
impl<'a> std::io::Read for Fixture<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}

#[cfg(feature = "blocking-client")]
impl<'a> std::io::BufRead for Fixture<'a> {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        self.0.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.0.consume(amt)
    }
}

#[cfg(feature = "blocking-client")]
impl<'a> git_transport::client::ReadlineBufRead for Fixture<'a> {
    fn readline(
        &mut self,
    ) -> Option<std::io::Result<Result<git_packetline::PacketLineRef<'_>, git_packetline::decode::Error>>> {
        use bstr::{BStr, ByteSlice};
        let bytes: &BStr = self.0.into();
        let mut lines = bytes.lines();
        let res = lines.next()?;
        self.0 = lines.as_bytes();
        Some(Ok(Ok(git_packetline::PacketLineRef::Data(res))))
    }
}

#[cfg(feature = "async-client")]
impl<'a> Fixture<'a> {
    fn project_inner(self: std::pin::Pin<&mut Self>) -> std::pin::Pin<&mut &'a [u8]> {
        #[allow(unsafe_code)]
        unsafe {
            std::pin::Pin::new(&mut self.get_unchecked_mut().0)
        }
    }
}

#[cfg(feature = "async-client")]
impl<'a> futures_io::AsyncRead for Fixture<'a> {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        self.project_inner().poll_read(cx, buf)
    }
}

#[cfg(feature = "async-client")]
impl<'a> futures_io::AsyncBufRead for Fixture<'a> {
    fn poll_fill_buf(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<&[u8]>> {
        self.project_inner().poll_fill_buf(cx)
    }

    fn consume(self: std::pin::Pin<&mut Self>, amt: usize) {
        self.project_inner().consume(amt)
    }
}

#[cfg(feature = "async-client")]
#[async_trait::async_trait(?Send)]
impl<'a> git_transport::client::ReadlineBufRead for Fixture<'a> {
    async fn readline(
        &mut self,
    ) -> Option<std::io::Result<Result<git_packetline::PacketLineRef<'_>, git_packetline::decode::Error>>> {
        use bstr::{BStr, ByteSlice};
        let bytes: &BStr = self.0.into();
        let mut lines = bytes.lines();
        let res = lines.next()?;
        self.0 = lines.as_bytes();
        Some(Ok(Ok(git_packetline::PacketLineRef::Data(res))))
    }
}
