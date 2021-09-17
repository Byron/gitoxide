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
