use git_repository::{Easy, Repository};

fn repo() -> crate::Result<Easy> {
    let repo_path = git_testtools::scripted_fixture_repo_read_only("make_references_repo.sh")?;
    Ok(Repository::discover(repo_path)?.into())
}

mod find {
    use crate::reference::repo;

    #[test]
    fn find_and_peel() {
        let mut repo = repo().unwrap();
        let _tag_ref = repo.find_reference("dt1").unwrap().expect("tag to exist");
    }
}
