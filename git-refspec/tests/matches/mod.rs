use git_refspec::instruction::Fetch;

#[test]
fn fetch_only() {
    let _spec = Fetch::Only {
        src: "refs/heads/main".into(),
    };
}
