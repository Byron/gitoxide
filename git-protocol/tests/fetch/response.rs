use std::io;

fn mock_reader(path: &str) -> git_packetline::Provider<std::io::Cursor<Vec<u8>>> {
    use crate::fixture_bytes;
    let buf = fixture_bytes(path);
    git_packetline::Provider::new(io::Cursor::new(buf), git_packetline::PacketLine::Flush)
}

fn id(hex: &str) -> git_object::owned::Id {
    git_object::owned::Id::from_40_bytes_in_hex(hex.as_bytes()).expect("expect valid hex id")
}

mod v1 {
    mod from_line_reader {
        use crate::fetch::response::{id, mock_reader};
        use git_protocol::fetch::{self, response::Acknowledgement};
        use git_transport::Protocol;

        #[test]
        #[ignore]
        fn simple_fetch_acks_and_pack() {
            let mut provider = mock_reader("v1/fetch.response");
            let r = fetch::Response::from_line_reader(Protocol::V1, Box::new(provider.as_read_without_sidebands()))
                .expect("reading to succeed");
            assert_eq!(
                r.acknowledgements(),
                &[
                    Acknowledgement::Common(id("6504930888c9c5337e7e065c964f87b60d16a7d7")),
                    Acknowledgement::Common(id("fe17165c392110d1305674c06e4aec35728bfab7")),
                    Acknowledgement::Common(id("f22743895a3024bb0c958335981439f1fa747d57")),
                    Acknowledgement::Ready,
                    Acknowledgement::NAK,
                ]
            );
        }
    }
}
