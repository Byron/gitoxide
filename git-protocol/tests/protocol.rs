type Result = std::result::Result<(), Box<dyn std::error::Error>>;
use std::path::PathBuf;

pub fn fixture_bytes(path: &str) -> Vec<u8> {
    std::fs::read(PathBuf::from("tests").join("fixtures").join(path)).expect("fixture to be present and readable")
}

mod credentials;
mod remote_progress;
mod fetch {
    struct CloneDelegate;
    use git_protocol::fetch;

    impl fetch::Delegate for CloneDelegate {}

    mod v1 {
        use crate::{fetch::CloneDelegate, fixture_bytes};
        use bstr::ByteSlice;

        #[test]
        #[ignore]
        fn clone() -> crate::Result {
            let mut out = Vec::new();
            let response = fixture_bytes("v1/clone.response");
            let transport = git_transport::client::git::Connection::new(
                response.as_slice(),
                &mut out,
                git_transport::Protocol::V1,
                b"does/not/matter".as_bstr().to_owned(),
                None::<(&str, _)>,
                git_transport::client::git::ConnectMode::Process,
            );
            git_protocol::fetch(transport, CloneDelegate, git_protocol::credentials::helper)?;
            Ok(())
        }
    }
}
