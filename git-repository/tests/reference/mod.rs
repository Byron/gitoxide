use git_repository::Repository;

fn repo() -> crate::Result<git_repository::Shared> {
    let repo_path = git_testtools::scripted_fixture_repo_read_only("make_references_repo.sh")?;
    Ok(Repository::discover(repo_path)?.into())
}

mod find {
    use git_repository::prelude::*;

    use crate::reference::repo;

    #[test]
    fn find_and_peel() {
        let mut repo = repo().unwrap();
        let tag_ref = repo.find_reference("dt1").unwrap().expect("tag to exist");
    }
}
