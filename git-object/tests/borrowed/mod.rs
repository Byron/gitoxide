use git_object::{borrowed::Signature, Sign, Time};
use std::path::PathBuf;

mod commit;
mod tag;
mod tree;

#[cfg(not(windows))]
fn fixup(v: Vec<u8>) -> Vec<u8> {
    v
}

#[cfg(windows)]
fn fixup(v: Vec<u8>) -> Vec<u8> {
    // Git checks out text files with line ending conversions, git itself will of course not put '\r\n' anywhere,
    // so that wouldn't be expected in an object and doesn't have to be parsed.
    use bstr::ByteSlice;
    v.replace(b"\r\n", "\n")
}

fn fixture_bytes(kind: &str, path: &str) -> Vec<u8> {
    fixup(super::fixture_bytes(PathBuf::from(kind).join(path).to_str().unwrap()))
}

fn signature(time: u32) -> Signature<'static> {
    use git_object::bstr::ByteSlice;
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
