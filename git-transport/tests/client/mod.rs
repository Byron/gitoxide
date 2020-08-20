mod git {
    mod upload_pack {
        use crate::fixture_bytes;
        use git_transport::{client::TransportSketch, Protocol, Service};
        use std::io::BufRead;

        #[test]
        #[ignore]
        fn clone_v1() -> crate::Result {
            let mut out = Vec::new();
            let input = fixture_bytes("v1/clone.response");
            let mut c = git_transport::client::git::Connection::new(
                input.as_slice(),
                &mut out,
                Protocol::V1,
                "/foo.git",
                Some(("example.org", None)),
            );
            let res = c.set_service(Service::UploadPack)?;
            assert_eq!(res.actual_protocol, Protocol::V1);
            // assert_eq!(res.capabilities, vec!["hello"].into());
            let refs = res
                .refs
                .expect("v1 protocol provides refs")
                .lines()
                .flat_map(Result::ok)
                .collect::<Vec<_>>();
            assert_eq!(refs, vec!["HEAD"]);
            Ok(())
        }

        #[test]
        fn upload_pack_clone_v2() {
            // With port
            // it lists the version in the first line
        }
        #[test]
        #[ignore]
        fn upload_pack_clone_version_unsupported() {
            // it replies with version 1, but doesn't list the version number, we can't test it actually, that's alright
        }
    }
}
