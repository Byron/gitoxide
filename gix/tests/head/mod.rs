mod into_remote {

    use crate::remote;

    #[test]
    fn unborn_is_none() -> crate::Result {
        let repo = remote::repo("url-rewriting");
        assert_eq!(
            repo.head()?.into_remote(gix::remote::Direction::Fetch).transpose()?,
            None
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
        Ok(())
    }
}
