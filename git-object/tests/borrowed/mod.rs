use git_object::{borrowed::Signature, Sign, Time};
use std::path::PathBuf;

mod commit;
mod tag;
mod tree;

fn fixture_bytes(kind: &str, path: &str) -> Vec<u8> {
    super::fixture_bytes(PathBuf::from(kind).join(path).to_str().unwrap())
}

fn signature(time: u32) -> Signature<'static> {
    use git_object::ByteSlice;
    Signature {
        name: b"Sebastian Thiel".as_bstr(),
        email: b"sebastian.thiel@icloud.com".as_bstr(),
        time: Time {
            time,
            offset: 28800,
            sign: Sign::Plus,
        },
    }
}

mod object {
    use git_object::borrowed::Object;

    #[test]
    fn size_in_memory() {
        assert_eq!(
            std::mem::size_of::<Object>(),
            200,
            "Prevent unexpected growth of what should be lightweight objects"
        )
    }
}
