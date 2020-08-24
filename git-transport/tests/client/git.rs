mod connect_message {
    use git_transport::{client::git, Protocol, Service};

    #[test]
    fn version_1_without_host_and_version() {
        assert_eq!(
            git::message::connect(Service::UploadPack, Protocol::V1, b"hello/world", None),
            "git-upload-pack hello/world\0"
        )
    }
    #[test]
    fn version_2_without_host_and_version() {
        assert_eq!(
            git::message::connect(Service::UploadPack, Protocol::V2, b"hello\\world", None),
            "git-upload-pack hello\\world\0\0version=2\0"
        )
    }
    #[test]
    fn with_host_without_port() {
        assert_eq!(
            git::message::connect(
                Service::UploadPack,
                Protocol::V1,
                b"hello\\world",
                Some(&("host".into(), None))
            ),
            "git-upload-pack hello\\world\0host=host\0"
        )
    }
    #[test]
    fn with_host_with_port() {
        assert_eq!(
            git::message::connect(
                Service::UploadPack,
                Protocol::V1,
                b"hello\\world",
                Some(&("host".into(), Some(404)))
            ),
            "git-upload-pack hello\\world\0host=host:404\0"
        )
    }
}

mod upload_pack {
    use crate::fixture_bytes;
    use bstr::ByteSlice;
    use git_transport::{client::TransportSketch, Protocol, Service};
    use std::io::BufRead;

    #[test]
    fn handshake_v1() -> crate::Result {
        let mut out = Vec::new();
        let input = fixture_bytes("v1/clone.response");
        let mut c = git_transport::client::git::Connection::new(
            input.as_slice(),
            &mut out,
            Protocol::V1,
            "/foo.git",
            Some(("example.org", None)),
        );
        let mut res = c.set_service(Service::UploadPack)?;
        assert_eq!(res.actual_protocol, Protocol::V1);
        assert_eq!(
            res.capabilities
                .iter()
                .flat_map(|c| c.value().map(ToOwned::to_owned))
                .collect::<Vec<_>>(),
            vec![
                b"HEAD:refs/heads/master".as_bstr(),
                b"sha1".as_bstr(),
                b"git/2.28.0".as_bstr()
            ]
        );
        let refs = res
            .refs
            .as_mut()
            .expect("v1 protocol provides refs")
            .lines()
            .flat_map(Result::ok)
            .collect::<Vec<_>>();
        assert_eq!(
            refs,
            vec![
                "808e50d724f604f69ab93c6da2919c014667bedb HEAD",
                "808e50d724f604f69ab93c6da2919c014667bedb refs/heads/master"
            ]
        );
        drop(res);
        assert_eq!(
            out.as_slice().as_bstr(),
            b"002egit-upload-pack /foo.git\0host=example.org\0".as_bstr(),
            "it sends the correct request"
        );
        Ok(())
    }

    #[test]
    fn handshake_v2() -> crate::Result {
        let mut out = Vec::new();
        let input = fixture_bytes("v2/clone.response");
        let mut c = git_transport::client::git::Connection::new(
            input.as_slice(),
            &mut out,
            Protocol::V2,
            "/bar.git",
            Some(("example.org", None)),
        );
        let mut res = c.set_service(Service::UploadPack)?;
        assert_eq!(res.actual_protocol, Protocol::V1);

        drop(res);
        assert_eq!(
            out.as_slice().as_bstr(),
            b"0039git-upload-pack /bar.git\0host=example.org\0\0version=2\0".as_bstr(),
            "it sends the correct request, including the adjusted version"
        );
        Ok(())
    }
}
