#[test]
#[ignore]
fn byte_order_marks_are_no_patterns() {
    git_attributes::parse("\u{feff}hello".as_bytes()).next();
    todo!();
}

#[test]
#[ignore]
fn line_numbers_are_counted_correctly() {
    todo!()
}

#[test]
#[ignore]
fn line_endings_can_be_windows_or_unix() {
    let _ = git_attributes::parse(b"unix\nwindows\r\nlast").collect::<Vec<_>>();
    todo!()
}

#[test]
fn comments_are_ignored() {
    assert!(git_attributes::parse(b"# hello world").next().is_none());
}

#[test]
#[ignore]
fn backslashes_before_hashes_are_part_of_the_path() {
    git_attributes::parse(br"\#hello").next();
    todo!();
}
