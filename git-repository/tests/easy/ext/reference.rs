mod head {
    use git_ref::transaction::{Change, PreviousValue, RefEdit};
    use git_ref::Target;
    use git_repository::prelude::ReferenceAccessExt;
    use git_testtools::hex_to_id;
    use std::convert::TryInto;

    #[test]
    fn symbolic() -> crate::Result {
        let repo = crate::basic_repo()?;
        let head = repo.head()?.expect("HEAD is symbolic");
        assert_eq!(
            head.inner.target.into_id(),
            hex_to_id("3189cd3cb0af8586c39a838aa3e54fd72a872a41")
        );
        Ok(())
    }

    #[test]
    fn detached() {
        let (repo, _keep) = crate::basic_rw_repo().unwrap();
        repo.edit_reference(
            RefEdit {
                change: Change::Update {
                    log: Default::default(),
                    expected: PreviousValue::Any,
                    new: Target::Peeled(hex_to_id("3189cd3cb0af8586c39a838aa3e54fd72a872a41")),
                },
                name: "HEAD".try_into().unwrap(),
                deref: false,
            },
            git_lock::acquire::Fail::Immediately,
            None,
        )
        .unwrap();

        assert!(repo.head().unwrap().is_none(), "head is detached");
    }
}
