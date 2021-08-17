use git_repository::Repository;

fn repo() -> crate::Result<git_repository::Shared> {
    let repo_path = git_testtools::scripted_fixture_repo_read_only("make_references_repo.sh")?;
    Ok(Repository::discover(repo_path)?.into())
}

mod find {
    use git_repository::prelude::*;

    use crate::reference::repo;

    #[test]
    #[ignore]
    fn find_and_peel() {
        let repo = repo().unwrap();
        let mut tag_ref = repo.find_reference("dt1").unwrap().expect("tag to exist");
        let _id = tag_ref.peel_to_id_in_place().unwrap();
    }
}
