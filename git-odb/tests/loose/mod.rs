use git_object::{borrowed, bstr::ByteSlice, Sign, Time};

pub mod db;

mod object {
    use git_odb::loose::Object;

    #[test]
    fn size_in_memory() {
        assert_eq!(
            std::mem::size_of::<Object>(),
            848,
            "Loose objects should not grow larger unexpectedly"
        )
    }
}

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
