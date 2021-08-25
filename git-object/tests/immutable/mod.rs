use std::path::PathBuf;

use git_actor::{Sign, Time};

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

fn signature(time: u32) -> git_actor::SignatureRef<'static> {
    use git_object::bstr::ByteSlice;
    git_actor::SignatureRef {
        name: b"Sebastian Thiel".as_bstr(),
        email: b"sebastian.thiel@icloud.com".as_bstr(),
        time: Time {
            time,
            offset: 28800,
            sign: Sign::Plus,
        },
    }
}

fn linus_signature(time: u32) -> git_actor::SignatureRef<'static> {
    use git_object::bstr::ByteSlice;
    git_actor::SignatureRef {
        name: b"Linus Torvalds".as_bstr(),
        email: b"torvalds@linux-foundation.org".as_bstr(),
        time: Time {
            time,
            offset: -25200,
            sign: Sign::Minus,
        },
    }
}
