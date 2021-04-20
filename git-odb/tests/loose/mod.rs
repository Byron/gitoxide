use git_object::{borrowed, bstr::ByteSlice, Sign, Time};

pub mod db;

fn signature(time: u32) -> borrowed::Signature<'static> {
    borrowed::Signature {
        name: b"Sebastian Thiel".as_bstr(),
        email: b"byronimo@gmail.com".as_bstr(),
        time: Time {
            time,
            offset: 7200,
            sign: Sign::Plus,
        },
    }
}
