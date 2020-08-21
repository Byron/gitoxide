use crate::fixture_bytes;
use bstr::ByteVec;
use std::io::{Read, Write};
use std::net::SocketAddr;
use std::time::Duration;

struct MockServer {
    addr: String,
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
        let addr = listener.local_addr().expect("a local address").to_string();
        MockServer {
            addr,
            thread: Some(std::thread::spawn(move || {
                let (mut stream, _) = listener.accept().expect("accept to always work");
                stream
                    .set_read_timeout(Some(Duration::from_millis(50)))
                    .expect("timeout to always work");
                stream
                    .set_write_timeout(Some(Duration::from_millis(50)))
                    .expect("timeout to always work");
                let mut out = Vec::new();
                stream.read_to_end(&mut out).expect("reading to always work");
                stream.write_all(&fixture).expect("write to always work");
                out
            })),
        }
    }

    pub fn received(&mut self) -> Vec<u8> {
        self.thread
            .take()
            .and_then(|h| h.join().ok())
            .expect("join to be called only once")
    }

    pub fn addr(&self) -> &str {
        self.addr.as_str()
    }

    pub fn received_as_string(&mut self) -> String {
        self.received().into_string().expect("utf8 only")
    }
}

fn serve_once(name: &str) -> MockServer {
    MockServer::new(fixture_bytes(name))
}

mod curl {
    use crate::client::http::serve_once;

    #[test]
    #[ignore]
    fn http_get_with_headers() {
        let mut server = serve_once("v1/http-handshake.response");

        assert_eq!(&server.received_as_string(), "hello");
    }
}
