mod remote {
    use git_repository as git;

    use crate::remote;

    #[test]
    fn unborn_is_none() -> crate::Result {
        let repo = remote::repo("url-rewriting");
        assert_eq!(
            repo.head()?.into_remote(git::remote::Direction::Fetch).transpose()?,
            None
        );
        Ok(())
    }
}
