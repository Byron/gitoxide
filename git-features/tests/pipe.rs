use std::io::{Read, Write};
use std::thread::spawn;

#[test]
fn threaded() {
    let (mut write, mut read) = git_features::pipe::unidirectional();

    let message = "Hello, world!";
    spawn(move || write.write_all(message.as_bytes()).unwrap());

    let mut s = String::new();
    read.read_to_string(&mut s).unwrap();

    assert_eq!(&s, message);
}
