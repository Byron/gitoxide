use git_repository::Repository;

fn repo() -> Repository {
    let dir = git_testtools::scripted_fixture_repo_read_only("make_merge_repo.sh").unwrap();
    git_repository::open(dir).unwrap()
}

#[test]
fn full() -> crate::Result {
    let repo: Repository = repo();
    let expected = repo.head_id()?;
    let actual = repo.rev_parse(expected.to_hex().to_string())?;
    assert_eq!(actual, expected, "Parse full sha");
    Ok(())
}

#[test]
fn short() -> crate::Result {
    let repo: Repository = repo();
    let expected = repo.head_id()?;
    let actual = repo.rev_parse(expected.to_hex_with_len(8).to_string())?;
    assert_eq!(actual, expected, "Parse short sha");
    Ok(())
}

#[test]
fn head() -> crate::Result {
    let repo: Repository = repo();
    let expected = repo.head_id().expect("Current repo has a HEAD");
    let actual = repo.rev_parse("HEAD").expect("Resolved HEAD");
    assert_eq!(actual, expected, "Parse HEAD");
    Ok(())
}

#[test]
fn at_sign() -> crate::Result {
    let repo: Repository = repo();
    let expected = repo.head_id().expect("Current repo has a HEAD");
    let actual = repo.rev_parse("@").expect("Resolved @");
    assert_eq!(actual, expected, "Parse HEAD");
    Ok(())
}

#[test]
fn main() -> crate::Result {
    let repo: Repository = repo();
    let expected = repo.find_reference("main")?.peel_to_id_in_place()?;
    let actual = repo.rev_parse("main").expect("Resolved main branch");
    assert_eq!(actual, expected, "Parse main branch");
    Ok(())
}
