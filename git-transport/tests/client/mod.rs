mod git {
    use git_transport::Protocol;
    use std::path::PathBuf;

    pub fn fixture_path(path: &str) -> PathBuf {
        PathBuf::from("tests").join("fixtures").join(path)
    }
    fn fixture_bytes(path: &str) -> Vec<u8> {
        std::fs::read(fixture_path(path)).expect("fixture to be present and readable")
    }

    #[test]
    fn upload_pack_clone_v1() {
        let mut out = Vec::new();
        let input = fixture_bytes("v1/clone.response");
        git_transport::client::git::Connection::new(input.as_slice(), &mut out, Protocol::V1);
    }
    #[test]
    fn upload_pack_clone_v2() {
        // it lists the version in the first line
    }
    #[test]
    #[ignore]
    fn upload_pack_clone_version_unsupported() {
        // it replies with version 1, but doesn't list the version number, we can't test it actually, that's alright
    }
}
