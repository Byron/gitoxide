use git_object::{borrowed::Signature, Sign, Time};
use std::path::PathBuf;

mod commit;
mod tag;
mod tree;

fn fixture_bytes(kind: &str, path: &str) -> Vec<u8> {
    super::fixture_bytes(PathBuf::from(kind).join(path).to_str().unwrap())
}

fn signature(time: u32) -> Signature<'static> {
    use bstr::ByteSlice;
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
