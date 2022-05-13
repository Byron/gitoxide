fn test_repo() -> crate::Repository {
    let cwd = std::env::current_dir().expect("CWD");
    git_repository::discover(cwd).expect("Repository")
}

mod simple {
    use crate::repository::rev_parse::test_repo;

    #[test]
    fn full() {
        let repo = test_repo();
        let expected = "e058bdabf8449b6a6fdff851e3929137d9b71568";
        let actual = repo
            .rev_parse("e058bdabf8449b6a6fdff851e3929137d9b71568")
            .expect("Found full SHA hash")
            .to_hex()
            .to_string();
        assert_eq!(actual, expected, "Parse full sha");
    }

    #[test]
    fn short() {
        let repo = test_repo();
        let expected = "e058bdabf8449b6a6fdff851e3929137d9b71568";
        let actual = repo
            .rev_parse("e058bdab")
            .expect("Found full 8 chars hash")
            .to_hex()
            .to_string();
        assert_eq!(actual, expected, "Parse full sha");
    }

    #[test]
    fn head() {
        let repo = test_repo();
        let expected = repo.head_id().expect("Current repo has a HEAD");
        let actual = repo.rev_parse("HEAD").expect("Resolved HEAD");
        assert_eq!(actual, expected, "Parse HEAD");
    }

    #[test]
    fn at_sign() {
        let repo = test_repo();
        let expected = repo.head_id().expect("Current repo has a HEAD");
        let actual = repo.rev_parse("@").expect("Resolved @");
        assert_eq!(actual, expected, "Parse HEAD");
    }

    #[test]
    fn main() {
        let repo = test_repo();
        repo.rev_parse("main").expect("Resolved main branch");
    }
}

mod advanced {
    use crate::repository::rev_parse::test_repo;

    #[test]
    fn at_previous() {
        let repo = test_repo();
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
    }

    #[test]
    fn head_previous() {
        let repo = test_repo();
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
    }

    #[test]
    fn at_first_parent() {
        let repo = test_repo();
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
    }

    #[test]
    fn head_first_parent() {
        let repo = test_repo();
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
    }

    #[test]
    fn branch_previous() {
        let repo = test_repo();
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
    }
}
