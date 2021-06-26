use git_actor::{Sign, Time};
use git_object::bstr::ByteSlice;

pub mod backend;

fn signature(time: u32) -> git_actor::immutable::Signature<'static> {
    git_actor::immutable::Signature {
        name: b"Sebastian Thiel".as_bstr(),
        email: b"byronimo@gmail.com".as_bstr(),
        time: Time {
            time,
            offset: 7200,
            sign: Sign::Plus,
        },
    }
}
