use git_repository::Repository;

fn repo() -> Repository {
    let dir = git_testtools::scripted_fixture_repo_read_only("make_merge_repo.sh").unwrap();
    git_repository::open(dir).unwrap()
}

#[test]
fn at_previous() -> crate::Result {
    let repo: Repository = repo();
    let input = "@~";
    let expected = {
        let head_id = repo.head_id().expect("Current repo has a HEAD");
        head_id
            .object()
            .unwrap()
            .try_into_commit()
            .unwrap()
            .parent_ids()
            .next()
            .unwrap()
    };
    let msg = format!("Parsed {}", input);
    let actual = repo.rev_parse(input).expect(&msg);
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn head_previous() -> crate::Result {
    let repo: Repository = repo();
    let input = "HEAD~";
    let expected = {
        let head_id = repo.head_id().expect("Current repo has a HEAD");
        head_id
            .object()
            .unwrap()
            .try_into_commit()
            .unwrap()
            .parent_ids()
            .next()
            .unwrap()
    };
    let msg = format!("Parsed {}", input);
    let actual = repo.rev_parse(input).expect(&msg);
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn at_first_parent() -> crate::Result {
    let repo: Repository = crate::repo("make_merge_repo.sh").map(Into::into)?;
    let input = "@^";
    let expected = {
        let head_id = repo.head_id().expect("Current repo has a HEAD");
        head_id
            .object()
            .unwrap()
            .try_into_commit()
            .unwrap()
            .parent_ids()
            .next()
            .unwrap()
    };
    let msg = format!("Parsed {}", input);
    let actual = repo.rev_parse(input).expect(&msg);
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn head_first_parent() -> crate::Result {
    let repo: Repository = repo();
    let input = "HEAD^";
    let expected = {
        let head_id = repo.head_id().expect("Current repo has a HEAD");
        head_id
            .object()
            .unwrap()
            .try_into_commit()
            .unwrap()
            .parent_ids()
            .next()
            .unwrap()
    };
    let msg = format!("Parsed {}", input);
    let actual = repo.rev_parse(input).expect(&msg);
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn branch_previous() -> crate::Result {
    let repo: Repository = repo();
    let input = "main^";
    let expected = {
        let id = repo.find_reference("main").unwrap().into_fully_peeled_id().unwrap();
        id.object()
            .unwrap()
            .try_into_commit()
            .unwrap()
            .parent_ids()
            .next()
            .unwrap()
    };
    let msg = format!("Parsed {}", input);
    let actual = repo.rev_parse(input).expect(&msg);
    assert_eq!(actual, expected);
    Ok(())
}
