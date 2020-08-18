mod parse {
    use bstr::ByteSlice;
    use git_packetline::RemoteProgress;
    use git_protocol::parse_remote_progress;

    #[test]
    fn a_message_we_dont_understand() {
        assert_eq!(
            parse_remote_progress(b"something that might be progress: but is not."),
            None
        )
    }

    #[test]
    fn enumerating_just_with_count() {
        assert_eq!(
            parse_remote_progress(b"Enumerating objects: 10, done."),
            Some(RemoteProgress {
                action: b"Enumerating objects".as_bstr(),
                percent: None,
                step: Some(10),
                max: None
            })
        )
    }

    #[test]
    fn counting_objects_with_percentage() {
        assert_eq!(
            parse_remote_progress(b"Counting objects: 50% (5/10), done."),
            Some(RemoteProgress {
                action: b"Counting objects".as_bstr(),
                percent: Some(50),
                step: Some(5),
                max: Some(10)
            })
        )
    }
}
