use git_refspec::instruction::Fetch;

#[test]
fn fetch_only() {
    baseline::parse().unwrap();
    let _spec = Fetch::Only {
        src: "refs/heads/main".into(),
    };
}

mod baseline {
    pub fn parse() -> crate::Result {
        let _ = git_testtools::scripted_fixture_repo_read_only("match_baseline.sh")?;
        Ok(())
    }
}
