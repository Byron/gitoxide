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
        use git_protocol::fetch::{
            self,
            response::{Acknowledgement, ShallowUpdate},
        };
        use git_transport::Protocol;
        use std::io::Read;

        #[test]
        fn clone() -> crate::Result {
            let mut provider = mock_reader("v1/clone-only.response");
            let mut reader = provider.as_read_without_sidebands();
            let r = fetch::Response::from_line_reader(Protocol::V1, &mut reader)?;
            assert_eq!(r.acknowledgements(), &[Acknowledgement::NAK]);
            assert!(r.has_pack());
            let mut buf = Vec::new();
            let bytes_read = reader.read_to_end(&mut buf)?;
            assert_eq!(bytes_read, 1090, "should be able to read the whole pack");
            Ok(())
        }

        #[test]
        fn shallow_clone() -> crate::Result {
            let mut provider = mock_reader("v1/clone-deepen-1.response");
            let mut reader = provider.as_read_without_sidebands();
            let r = fetch::Response::from_line_reader(Protocol::V1, &mut reader)?;
            assert_eq!(
                r.shallow_updates(),
                &[ShallowUpdate::Shallow(id("808e50d724f604f69ab93c6da2919c014667bedb"))]
            );
            assert_eq!(r.acknowledgements(), &[Acknowledgement::NAK]);
            assert!(r.has_pack());
            let mut buf = Vec::new();
            let bytes_read = reader.read_to_end(&mut buf)?;
            assert_eq!(bytes_read, 1989, "should be able to read the whole pack");
            Ok(())
        }

        #[test]
        fn empty_shallow_clone_due_to_depth_being_too_high() -> crate::Result {
            let mut provider = mock_reader("v1/clone-deepen-5.response");
            let mut reader = provider.as_read_without_sidebands();
            let r = fetch::Response::from_line_reader(Protocol::V1, &mut reader)?;
            assert!(r.shallow_updates().is_empty());
            assert_eq!(r.acknowledgements(), &[Acknowledgement::NAK]);
            assert!(r.has_pack());
            let mut buf = Vec::new();
            let bytes_read = reader.read_to_end(&mut buf)?;
            assert_eq!(bytes_read, 1988, "should be able to read the whole pack");
            Ok(())
        }

        #[test]
        fn fetch_acks_without_pack() -> crate::Result {
            let mut provider = mock_reader("v1/fetch-no-pack.response");
            let r = fetch::Response::from_line_reader(Protocol::V1, &mut provider.as_read_without_sidebands())?;
            assert_eq!(
                r.acknowledgements(),
                &[
                    Acknowledgement::Common(id("47ee0b7fe4f3a7d776c78794873e6467e1c47e59")),
                    Acknowledgement::Common(id("3f02c0ad360d96e8dbba92f97b42ebbaa4319db1")),
                    Acknowledgement::NAK,
                ]
            );
            Ok(())
        }

        #[test]
        fn fetch_acks_and_pack() -> crate::Result {
            let mut provider = mock_reader("v1/fetch.response");
            let mut reader = provider.as_read_without_sidebands();
            let r = fetch::Response::from_line_reader(Protocol::V1, &mut reader)?;
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
            assert!(r.has_pack());
            let mut buf = Vec::new();
            let bytes_read = reader.read_to_end(&mut buf)?;
            assert_eq!(bytes_read, 9703, "should be able to read the whole pack");
            Ok(())
        }
    }
}
mod v2 {
    mod from_line_reader {
        use crate::fetch::response::{id, mock_reader};
        use git_protocol::fetch::response::ShallowUpdate;
        use git_protocol::fetch::{self, response::Acknowledgement};
        use git_transport::Protocol;
        use std::io;
        use std::io::Read;

        #[test]
        fn clone() -> crate::Result {
            let mut provider = mock_reader("v2/clone-only.response");
            let mut reader = provider.as_read_without_sidebands();
            let r = fetch::Response::from_line_reader(Protocol::V2, &mut reader)?;
            assert!(r.acknowledgements().is_empty(), "it should go straight to the packfile");
            assert!(r.has_pack());
            let mut buf = Vec::new();
            let bytes_read = reader.read_to_end(&mut buf)?;
            assert_eq!(bytes_read, 1089, "should be able to read the whole pack");
            Ok(())
        }

        #[test]
        fn shallow_clone() -> crate::Result {
            let mut provider = mock_reader("v2/clone-deepen-1.response");
            let mut reader = provider.as_read_without_sidebands();
            let r = fetch::Response::from_line_reader(Protocol::V2, &mut reader)?;
            assert!(r.acknowledgements().is_empty(), "it should go straight to the packfile");
            assert_eq!(
                r.shallow_updates(),
                &[ShallowUpdate::Shallow(id("808e50d724f604f69ab93c6da2919c014667bedb"))]
            );
            assert!(r.has_pack());
            assert_eq!(
                io::copy(&mut reader, &mut io::sink())?,
                1991,
                "should be able to read the whole pack"
            );
            Ok(())
        }

        #[test]
        fn empty_shallow_clone() -> crate::Result {
            let mut provider = mock_reader("v2/clone-deepen-5.response");
            let mut reader = provider.as_read_without_sidebands();
            let r = fetch::Response::from_line_reader(Protocol::V2, &mut reader)?;
            assert!(r.acknowledgements().is_empty(), "it should go straight to the packfile");
            assert!(r.shallow_updates().is_empty(), "it should go straight to the packfile");
            assert!(r.has_pack());
            let mut buf = Vec::new();
            let bytes_read = reader.read_to_end(&mut buf)?;
            assert_eq!(bytes_read, 1989, "should be able to read the whole pack");
            Ok(())
        }

        #[test]
        fn clone_with_sidebands() -> crate::Result {
            let mut provider = mock_reader("v2/clone-only-2.response");
            let mut reader = provider.as_read_without_sidebands();
            let r = fetch::Response::from_line_reader(Protocol::V2, &mut reader)?;
            assert!(r.acknowledgements().is_empty(), "it should go straight to the packfile");
            assert!(r.has_pack());

            let mut buf = Vec::new();
            reader.set_progress_handler(Some(Box::new(|is_err: bool, _data: &[u8]| {
                assert!(!is_err, "fixture does not have an error");
            }) as git_transport::client::HandleProgress));
            let bytes_read = reader.read_to_end(&mut buf)?;
            assert_eq!(bytes_read, 1643, "should be able to read the whole pack");
            assert_eq!(&buf[..4], b"PACK");
            assert_eq!(
                git_object::owned::Id::from_20_bytes(&buf[buf.len() - 20..]).to_string(),
                "f34c9be7e0c3ef2c3ed7c62cc7791dbf6dc5ec9a"
            );
            Ok(())
        }

        #[test]
        fn fetch_acks_without_pack() -> crate::Result {
            let mut provider = mock_reader("v2/fetch-no-pack.response");
            let r = fetch::Response::from_line_reader(Protocol::V2, &mut provider.as_read_without_sidebands())?;
            assert_eq!(r.acknowledgements(), &[Acknowledgement::NAK,]);
            Ok(())
        }

        #[test]
        fn fetch_acks_and_pack() -> crate::Result {
            let mut provider = mock_reader("v2/fetch.response");
            let mut reader = provider.as_read_without_sidebands();
            let r = fetch::Response::from_line_reader(Protocol::V2, &mut reader)?;
            assert_eq!(
                r.acknowledgements(),
                &[
                    Acknowledgement::Common(id("190c3f6b2319c1f4ec854215533caf8623f8f870")),
                    Acknowledgement::Common(id("97c5a932b3940a09683e924ef6a92b31a6f7c6de")),
                    Acknowledgement::Ready,
                ]
            );
            assert!(r.has_pack());
            let mut buf = Vec::new();
            reader.set_progress_handler(Some(Box::new(|a: bool, b: &[u8]| {
                git_protocol::RemoteProgress::translate_to_progress(a, b, &mut git_features::progress::Discard)
            }) as git_transport::client::HandleProgress));
            let bytes_read = reader.read_to_end(&mut buf)?;
            assert_eq!(bytes_read, 5360, "should be able to read the whole pack");
            Ok(())
        }
    }
}
