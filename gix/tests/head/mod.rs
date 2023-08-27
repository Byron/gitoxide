mod into_remote {

    use crate::remote;

    #[test]
    fn unborn_is_none() -> crate::Result {
        let repo = remote::repo("url-rewriting");
        assert_eq!(
            repo.head()?.into_remote(gix::remote::Direction::Fetch).transpose()?,
            None
        );
        assert_eq!(
            repo.find_fetch_remote(None)?.name().expect("present").as_ref(),
            "origin",
            "we can fallback to the only available remote"
        );
        Ok(())
    }

    #[test]
    fn detached_is_none() -> crate::Result {
        let repo = remote::repo("detached-head");
        assert_eq!(
            repo.head()?.into_remote(gix::remote::Direction::Fetch).transpose()?,
            None
        );
        assert_eq!(
            repo.find_fetch_remote(None)?.name().expect("present").as_ref(),
            "origin",
            "we can fallback to the only available remote"
        );
        Ok(())
    }
}
