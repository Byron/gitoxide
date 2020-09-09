use std::io;

fn mock_reader(path: &str) -> git_packetline::Provider<std::io::Cursor<Vec<u8>>> {
    use crate::fixture_bytes;
    let buf = fixture_bytes(path);
    git_packetline::Provider::new(io::Cursor::new(buf), &[git_packetline::PacketLine::Flush])
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
        fn clone() -> crate::Result {
            let mut provider = mock_reader("v1/clone-only.response");
            let r = fetch::Response::from_line_reader(Protocol::V1, Box::new(provider.as_read_without_sidebands()))?;
            assert_eq!(r.acknowledgements(), &[Acknowledgement::NAK]);
            match r.try_into_pack() {
                Ok(mut pack_read) => {
                    let mut buf = Vec::new();
                    let bytes_read = pack_read.read_to_end(&mut buf)?;
                    assert_eq!(bytes_read, 1090, "should be able to read the whole pack");
                }
                Err(_) => panic!("We must get a pack out of a clone response"),
            }
            Ok(())
        }

        #[test]
        fn simple_fetch_acks_and_pack() -> crate::Result {
            let mut provider = mock_reader("v1/fetch.response");
            let r = fetch::Response::from_line_reader(Protocol::V1, Box::new(provider.as_read_without_sidebands()))?;
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
            Ok(())
        }
    }
}
mod v2 {
    mod from_line_reader {
        use crate::fetch::response::{id, mock_reader};
        use git_protocol::fetch::{self, response::Acknowledgement};
        use git_transport::Protocol;
        use std::io::Read;

        #[test]
        fn clone() -> crate::Result {
            let mut provider = mock_reader("v2/clone-only.response");
            let r = fetch::Response::from_line_reader(Protocol::V2, Box::new(provider.as_read_without_sidebands()))?;
            assert!(r.acknowledgements().is_empty(), "it should go straight to the packfile");
            match r.try_into_pack() {
                Ok(mut pack_read) => {
                    let mut buf = Vec::new();
                    let bytes_read = pack_read.read_to_end(&mut buf)?;
                    assert_eq!(bytes_read, 1089, "should be able to read the whole pack");
                }
                Err(_) => panic!("We must get a pack out of a clone response"),
            }
            Ok(())
        }

        #[test]
        fn simple_fetch_acks_and_pack() -> crate::Result {
            let mut provider = mock_reader("v2/fetch.response");
            let r = fetch::Response::from_line_reader(Protocol::V2, Box::new(provider.as_read_without_sidebands()))?;
            assert_eq!(
                r.acknowledgements(),
                &[
                    Acknowledgement::Common(id("190c3f6b2319c1f4ec854215533caf8623f8f870")),
                    Acknowledgement::Common(id("97c5a932b3940a09683e924ef6a92b31a6f7c6de")),
                    Acknowledgement::Ready,
                ]
            );
            Ok(())
        }
    }
}
