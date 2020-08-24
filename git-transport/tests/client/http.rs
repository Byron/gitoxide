use crate::fixture_bytes;
use bstr::ByteVec;
use git_transport::{client::TransportSketch, Protocol, Service};
use std::{
    error::Error,
    io::{Read, Write},
    net::SocketAddr,
    time::Duration,
};

struct MockServer {
    addr: SocketAddr,
    thread: Option<std::thread::JoinHandle<Vec<u8>>>,
}

impl MockServer {
    pub fn new(fixture: Vec<u8>) -> Self {
        let ports = (15411..).take(10);
        let listener = std::net::TcpListener::bind(
            ports
                .map(|port| SocketAddr::from(([127, 0, 0, 1], port)))
                .collect::<Vec<_>>()
                .as_slice(),
        )
        .expect("one of these ports to be free");
        let addr = listener.local_addr().expect("a local address");
        let (set_ready, is_ready) = std::sync::mpsc::sync_channel(0);
        let handle = std::thread::spawn(move || {
            set_ready.send(()).expect("someone listening");
            let (mut stream, _) = listener.accept().expect("accept to always work");
            stream
                .set_read_timeout(Some(Duration::from_millis(50)))
                .expect("timeout to always work");
            stream
                .set_write_timeout(Some(Duration::from_millis(50)))
                .expect("timeout to always work");
            let mut out = Vec::new();
            stream.read_to_end(&mut out).ok();
            stream.write_all(&fixture).expect("write to always work");
            stream.flush().expect("flush to work");
            out
        });
        is_ready.recv().expect("someone sending eventually");
        MockServer {
            addr,
            thread: Some(handle),
        }
    }

    pub fn received(&mut self) -> Vec<u8> {
        self.thread
            .take()
            .and_then(|h| h.join().ok())
            .expect("join to be called only once")
    }

    pub fn addr(&self) -> &SocketAddr {
        &self.addr
    }

    pub fn received_as_string(&mut self) -> String {
        self.received().into_string().expect("utf8 only")
    }
}

fn serve_once(name: &str) -> MockServer {
    MockServer::new(fixture_bytes(name))
}

fn serve_and_connect(
    name: &str,
    path: &str,
    version: Protocol,
) -> Result<(MockServer, git_transport::client::http::Transport), crate::Error> {
    let server = serve_once(name);
    let client = git_transport::client::http::connect(
        &format!(
            "http://{}:{}/{}",
            &server.addr().ip().to_string(),
            &server.addr().port(),
            path
        ),
        version,
    )?;
    Ok((server, client))
}

fn assert_error_status(status: usize, kind: std::io::ErrorKind) -> crate::Result {
    let (_server, mut client) =
        serve_and_connect(&format!("http-{}.response", status), "path/not-important", Protocol::V1)?;
    let error = client
        .set_service(Service::UploadPack)
        .err()
        .expect("non-200 status causes error");
    let error = error
        .source()
        .expect("source")
        .downcast_ref::<std::io::Error>()
        .expect("io error as source");
    assert_eq!(error.kind(), kind);
    assert_eq!(error.to_string(), format!("Received HTTP status {}", status));
    Ok(())
}

#[test]
fn http_authentication_error_can_be_differentiated() -> crate::Result {
    assert_error_status(401, std::io::ErrorKind::PermissionDenied)
}

#[test]
fn http_error_results_in_observable_error() -> crate::Result {
    assert_error_status(404, std::io::ErrorKind::Other)
}

mod upload_pack {
    use crate::client::http::serve_and_connect;
    use bstr::ByteSlice;
    use git_transport::{client::SetServiceResponse, client::TransportSketch, Protocol, Service};
    use std::io::BufRead;

    #[test]
    fn handshake_v1() -> crate::Result {
        let (mut server, mut c) = serve_and_connect(
            "v1/http-handshake.response",
            "path/not/important/due/to/mock",
            Protocol::V1,
        )?;
        let SetServiceResponse {
            actual_protocol,
            capabilities,
            refs,
        } = c.set_service(Service::UploadPack)?;
        assert_eq!(actual_protocol, Protocol::V1);
        assert_eq!(
            capabilities
                .iter()
                .filter_map(|c| c.value().map(ToOwned::to_owned))
                .collect::<Vec<_>>(),
            vec![b"HEAD:refs/heads/main".as_bstr(), b"git/github-gdf51a71f0236".as_bstr(),]
        );
        let refs = refs
            .expect("v1 protocol provides refs")
            .lines()
            .flat_map(Result::ok)
            .collect::<Vec<_>>();
        assert_eq!(
            refs,
            vec![
                "73a6868963993a3328e7d8fe94e5a6ac5078a944 HEAD",
                "73a6868963993a3328e7d8fe94e5a6ac5078a944 refs/heads/main",
                "8e472f9ccc7d745927426cbb2d9d077de545aa4e refs/pull/13/head",
                "1a33becbfa6aaf7661824ce40016acb8c179f13c refs/pull/14/head",
                "add2e3e8d155571154c8816cf57f473a6e4d8d31 refs/pull/2/head",
                "dce0ea858eef7ff61ad345cc5cdac62203fb3c10 refs/tags/git-commitgraph-v0.0.0",
                "21c9b7500cb144b3169a6537961ec2b9e865be81 refs/tags/git-commitgraph-v0.0.0^{}",
                "7ba6656568da186d153d66f26990b9b364ea9609 refs/tags/git-features-v0.1.0",
                "5688a3427ff3673e1422d43106f4d685fa837aed refs/tags/git-features-v0.1.0^{}",
                "92945a59059bf044744639673f1a0f5b314762ee refs/tags/git-features-v0.2.0",
                "0bb831480d8657e1bb29ee7009aeac673471403e refs/tags/git-features-v0.2.0^{}",
                "97e1d77270a8f9cbff19baf3803de8b4f5a339bf refs/tags/git-features-v0.3.0",
                "4351e2871c9dcf342b8471fffa74cae338a53269 refs/tags/git-features-v0.3.0^{}",
                "d5f78373a75de13ef3c08eedf03e616b2ec395f2 refs/tags/git-features-v0.4.0",
                "9d6b8790e2edd7fa01b3239adff86a7cd2393f10 refs/tags/git-features-v0.4.0^{}",
                "be64896ed543437b67e939c36ecd70945e100d6c refs/tags/git-object-v0.1.0",
                "5688a3427ff3673e1422d43106f4d685fa837aed refs/tags/git-object-v0.1.0^{}",
                "7b34dc75ac5010741c0675d8c3a9645adb9b2ee1 refs/tags/git-object-v0.3.0",
                "e8df6c1ffb7afa27aff9abbe11c7e4b80d19b61e refs/tags/git-object-v0.3.0^{}",
                "2249ae57005b7c5ff94409bbe0e3213cbfd1745f refs/tags/git-odb-v0.1.0",
                "2b80181ad428a9bf267a9660886f347a850fc76f refs/tags/git-odb-v0.1.0^{}",
                "a9bb4d08a8c159d2444615ce9f9bc68f40fe98b1 refs/tags/git-odb-v0.3.0",
                "e8df6c1ffb7afa27aff9abbe11c7e4b80d19b61e refs/tags/git-odb-v0.3.0^{}",
                "d5d9eabaa9f190e535771c8dcc9fd1bcf69b7947 refs/tags/git-packetline-v0.1.0",
                "9d6b8790e2edd7fa01b3239adff86a7cd2393f10 refs/tags/git-packetline-v0.1.0^{}",
                "defd2a7783ab4618f41c270477921aa2336693db refs/tags/git-protocol-v0.0.0",
                "14615143dc170217ca4acc80191f4e6725dc460a refs/tags/git-protocol-v0.0.0^{}",
                "7e168eef62b8ad6ddd49e4e50d500761b84cfb4f refs/tags/git-ref-v0.1.0",
                "e66c9ed041c7ebede869e899ecd4398fee47028b refs/tags/git-ref-v0.1.0^{}",
                "fde229329d5d4540d21a04dcaf8cfb13a1e8a8c5 refs/tags/git-ref-v0.2.0",
                "d350a13784685ea82b84646b18736986aeb68146 refs/tags/git-ref-v0.2.0^{}",
                "4f75945daf9e0a669b694b0652c5a7e8a6dd2246 refs/tags/git-ref-v0.3.0",
                "e8df6c1ffb7afa27aff9abbe11c7e4b80d19b61e refs/tags/git-ref-v0.3.0^{}",
                "058e7f3f554f37f05cc9aaf0c86b4bbe8bea9242 refs/tags/git-repository-v0.1.0",
                "2b80181ad428a9bf267a9660886f347a850fc76f refs/tags/git-repository-v0.1.0^{}",
                "74b85f2bc7a9bcdd59218ee54135d5dd3a8dbd72 refs/tags/git-repository-v0.3.0",
                "e8df6c1ffb7afa27aff9abbe11c7e4b80d19b61e refs/tags/git-repository-v0.3.0^{}",
                "40046d9f4ab51a8895e8de8a3ed4e213d87f042e refs/tags/git-transport-v0.0.0",
                "19e7fec7deb5a6419f36a2732c90006377414181 refs/tags/git-transport-v0.0.0^{}",
                "64bdbb4ef5415d4cfb088fbbdc8f5f6dca37aeca refs/tags/git-tui-v0.0.0",
                "a0b73afdd1df9b1096f0c6fe388f795a6dfe7f33 refs/tags/git-tui-v0.0.0^{}",
                "320c79b59068fc5f0fc11d331de7352bb1952f10 refs/tags/git-url-v0.0.0",
                "fd2e5bab97f09666c983634fa89947a4bed1c92d refs/tags/git-url-v0.0.0^{}",
                "58cbf2153987f6f4e91bd58074a1dd648f30f932 refs/tags/gitoxide-core-v0.1.0",
                "19e7fec7deb5a6419f36a2732c90006377414181 refs/tags/gitoxide-core-v0.1.0^{}",
                "640ce76991e36035af707ec4f9afc550cc33cb58 refs/tags/gitoxide-core-v0.3.0",
                "e8df6c1ffb7afa27aff9abbe11c7e4b80d19b61e refs/tags/gitoxide-core-v0.3.0^{}",
                "df1d23e4e6c489a74ab6c6845de49e54fe5a8f4d refs/tags/v0.1.0",
                "19e7fec7deb5a6419f36a2732c90006377414181 refs/tags/v0.1.0^{}",
                "7443892cb6b7925d98687903ab6d7ee0bdd1e9cf refs/tags/v0.3.0",
                "e8df6c1ffb7afa27aff9abbe11c7e4b80d19b61e refs/tags/v0.3.0^{}"
            ]
        );
        assert_eq!(
            server.received_as_string().lines().collect::<Vec<_>>(),
            format!(
                "GET /path/not/important/due/to/mock/info/refs?service=git-upload-pack HTTP/1.1
Host: 127.0.0.1:{}
Accept: */*
User-Agent: git/oxide-{}

",
                server.addr.port(),
                env!("CARGO_PKG_VERSION")
            )
            .lines()
            .collect::<Vec<_>>()
        );
        Ok(())
    }
    #[test]
    fn handshake_v2() -> crate::Result {
        let (mut server, mut c) = serve_and_connect(
            "v2/http-handshake.response",
            "path/not/important/due/to/mock",
            Protocol::V2,
        )?;
        let SetServiceResponse {
            actual_protocol,
            capabilities,
            refs,
        } = c.set_service(Service::UploadPack)?;
        assert_eq!(actual_protocol, Protocol::V2);
        assert!(
            refs.is_none(),
            "refs are only returned in V1, as V2 favors a separate command (with more options)"
        );
        assert_eq!(
            capabilities
                .iter()
                .map(|v| {
                    (
                        v.name().to_owned(),
                        v.values().map(|v| v.map(ToOwned::to_owned).collect::<Vec<_>>()),
                    )
                })
                .collect::<Vec<_>>(),
            [
                ("agent", Some(&["git/github-gdf51a71f0236"][..])),
                ("ls-refs", None),
                ("fetch", Some(&["shallow", "filter"])),
                ("server-option", None)
            ]
            .iter()
            .map(|(k, v)| (
                k.as_bytes().into(),
                v.map(|v| v.iter().map(|v| v.as_bytes().into()).collect::<Vec<_>>())
            ))
            .collect::<Vec<_>>()
        );

        assert_eq!(
            server.received_as_string().lines().collect::<Vec<_>>(),
            format!(
                "GET /path/not/important/due/to/mock/info/refs?service=git-upload-pack HTTP/1.1
Host: 127.0.0.1:{}
Accept: */*
User-Agent: git/oxide-{}
Git-Protocol: version=2

",
                server.addr.port(),
                env!("CARGO_PKG_VERSION")
            )
            .lines()
            .collect::<Vec<_>>()
        );
        Ok(())
    }
}
