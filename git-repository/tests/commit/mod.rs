pub mod summary {
    use bstr::ByteSlice;
    use git_repository as git;
    use std::borrow::Cow;

    #[test]
    fn no_newline_yields_the_message_itself() {
        let input = b"hello world".as_bstr();
        assert_eq!(git::commit::summary(input), Cow::Borrowed(input));
    }

    #[test]
    fn trailing_newlines_and_whitespace_are_trimmed() {
        let input = b"hello world \t\r\n \n".as_bstr();
        assert_eq!(git::commit::summary(input), Cow::Borrowed(b"hello world".as_bstr()));
    }

    #[test]
    fn prefixed_newlines_and_whitespace_are_trimmed() {
        let input = b" \t\r\n \nhello world".as_bstr();
        assert_eq!(git::commit::summary(input), Cow::Borrowed(b"hello world".as_bstr()));
    }

    #[test]
    fn whitespace_up_to_a_newline_is_collapsed_into_a_space() {
        let input = b" \t\r\n \nhello\r\nworld \t\r\n \n".as_bstr();
        assert_eq!(git::commit::summary(input), Cow::Borrowed(b"hello world".as_bstr()));
    }

    #[test]
    fn lines_separated_by_double_newlines_are_subjects() {
        let input = b" \t\r\n \nhello\t \r\nworld \t\r \nfoo\n\nsomething else we ignore".as_bstr();
        assert_eq!(git::commit::summary(input), Cow::Borrowed(b"hello world foo".as_bstr()));
    }
}
