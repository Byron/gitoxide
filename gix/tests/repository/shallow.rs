use crate::util::{hex_to_id, named_subrepo_opts};

#[test]
fn no() -> crate::Result {
    for name in ["base", "empty"] {
        let repo = named_subrepo_opts("make_shallow_repo.sh", name, gix::open::Options::isolated())?;
        assert!(!repo.is_shallow());
        assert!(repo.shallow_commits()?.is_none());
    }
    Ok(())
}

#[test]
fn yes() -> crate::Result {
    for name in ["shallow.git", "shallow"] {
        let repo = named_subrepo_opts("make_shallow_repo.sh", name, gix::open::Options::isolated())?;
        assert!(repo.is_shallow());
        assert_eq!(
            repo.shallow_commits()?.expect("present").as_slice(),
            [hex_to_id("30887839de28edf7ab66c860e5c58b4d445f6b12")]
        );
    }
    Ok(())
}
