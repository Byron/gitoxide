use bstr::ByteSlice;
use git_object::commit::MessageRef;

#[test]
fn only_title_no_trailing_newline() {
    let msg = MessageRef::from_bytes(b"hello there");
    assert_eq!(
        msg,
        MessageRef {
            title: b"hello there".as_bstr(),
            body: None
        }
    );
    assert_eq!(msg.summary().as_ref(), "hello there");
}

#[test]
fn title_and_body() {
    let msg = MessageRef::from_bytes(b"hello\n\nthere");
    assert_eq!(
        msg,
        MessageRef {
            title: b"hello".as_bstr(),
            body: Some("there".into())
        }
    );
    assert_eq!(msg.summary().as_ref(), "hello");
}

#[test]
fn only_title_trailing_newline_is_retained() {
    let msg = MessageRef::from_bytes(b"hello there\n");
    assert_eq!(
        msg,
        MessageRef {
            title: b"hello there\n".as_bstr(),
            body: None
        }
    );
    assert_eq!(msg.summary().as_ref(), "hello there");
}

#[test]
fn only_title_trailing_windows_newline_is_retained() {
    let msg = MessageRef::from_bytes(b"hello there\r\n");
    assert_eq!(
        msg,
        MessageRef {
            title: b"hello there\r\n".as_bstr(),
            body: None
        }
    );
    assert_eq!(msg.summary().as_ref(), "hello there");
}

#[test]
fn title_with_whitespace_and_body() {
    let msg = MessageRef::from_bytes(b"hello \t \r\n there\nanother line\n\nthe body\n\n");
    assert_eq!(msg.summary().as_ref(), "hello  there another line");
    assert_eq!(
        msg,
        MessageRef {
            title: b"hello \t \r\n there\nanother line".as_bstr(),
            body: Some(b"the body\n\n".as_bstr())
        }
    );
}

#[test]
fn title_with_more_whitespace_and_body() {
    let msg = MessageRef::from_bytes(b"hello \r\r\r\n there\nanother line\n\nthe body\n\n");
    assert_eq!(msg.summary().as_ref(), "hello  there another line");
    assert_eq!(
        msg,
        MessageRef {
            title: b"hello \r\r\r\n there\nanother line".as_bstr(),
            body: Some(b"the body\n\n".as_bstr())
        }
    );
}

#[test]
fn title_with_whitespace_and_body_windows_lineending() {
    let msg = MessageRef::from_bytes(b"hello \r\n \r\n there\nanother line\r\n\r\nthe body\n\r\n");
    assert_eq!(msg.summary().as_ref(), "hello   there another line");
    assert_eq!(
        msg,
        MessageRef {
            title: b"hello \r\n \r\n there\nanother line".as_bstr(),
            body: Some(b"the body\n\r\n".as_bstr())
        }
    );
}

#[test]
fn title_with_separator_and_empty_body() {
    let msg = MessageRef::from_bytes(b"hello\n\n");
    assert_eq!(msg.summary().as_ref(), "hello");
    assert_eq!(
        msg,
        MessageRef {
            title: b"hello".as_bstr(),
            body: None
        }
    );
}

#[test]
fn title_with_windows_separator_and_empty_body() {
    let msg = MessageRef::from_bytes(b"hello\r\n\r\n");
    assert_eq!(msg.summary().as_ref(), "hello");
    assert_eq!(
        msg,
        MessageRef {
            title: b"hello".as_bstr(),
            body: None
        }
    );
}

pub mod summary {
    use std::borrow::Cow;

    use git_object::{
        bstr::{BStr, ByteSlice},
        commit::MessageRef,
    };

    fn summary(input: &[u8]) -> Cow<'_, BStr> {
        MessageRef::from_bytes(input).summary()
    }

    #[test]
    fn no_newline_yields_the_message_itself() {
        let input = b"hello world".as_bstr();
        assert_eq!(summary(input), Cow::Borrowed(input));
    }

    #[test]
    fn trailing_newlines_and_whitespace_are_trimmed() {
        let input = b"hello world \t\r\n \n";
        assert_eq!(summary(input), Cow::Borrowed(b"hello world".as_bstr()));
    }

    #[test]
    fn prefixed_newlines_and_whitespace_are_trimmed() {
        let input = b" \t\r\n \nhello world";
        assert_eq!(summary(input), Cow::Borrowed(b"hello world".as_bstr()));
    }

    #[test]
    fn whitespace_up_to_a_newline_is_collapsed_into_a_space() {
        let input = b" \t\r\n \nhello\r\nworld \t\r\n \n";
        assert_eq!(summary(input), Cow::Borrowed(b"hello world".as_bstr()));
    }

    #[test]
    fn whitespace_without_newlines_is_ignored_except_for_leading_and_trailing_whitespace() {
        let input = b" \t\r\n \nhello \t \rworld \t\r\n \n";
        assert_eq!(summary(input), Cow::Borrowed(b"hello \t \rworld".as_bstr()));
    }

    #[test]
    fn lines_separated_by_double_newlines_are_subjects() {
        let input = b" \t\r\n \nhello\t \r\nworld \t\r \nfoo\n\nsomething else we ignore";
        assert_eq!(summary(input), Cow::Borrowed(b"hello world foo".as_bstr()));
    }
}
