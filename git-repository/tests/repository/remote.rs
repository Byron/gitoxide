use crate::remote;

#[test]
#[ignore]
fn find_remote() {
    let repo = remote::repo("clone");
    let mut count = 0;
    for name in repo.remote_names() {
        count += 1;
        assert_eq!(repo.find_remote(name).expect("no error").name(), Some(name));
    }
    assert!(count > 0, "should have seen more than one commit")
}
