#[test]
fn can_parse() {
    let buf = b"";
    git_pathspec::parse(buf);
}

#[test]
fn can_match() {
    let buf = b"git-pathspec/tests/pathspec.rs";
    git_pathspec::matches(buf);
}
