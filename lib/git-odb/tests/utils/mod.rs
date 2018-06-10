use std::path::PathBuf;

pub fn fixture(path: &str) -> PathBuf {
    let mut b = PathBuf::from(file!());
    b.pop();
    b.pop();
    b.push("fixtures");
    b.push(path);
    b
}
