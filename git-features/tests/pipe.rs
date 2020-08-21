use git_features::pipe;
use std::io::{ErrorKind, Read, Write};

#[test]
fn threaded_read_to_end() {
    let (mut writer, mut reader) = git_features::pipe::unidirectional(0);

    let message = "Hello, world!";
    std::thread::spawn(move || {
        writer
            .write_all(message.as_bytes())
            .expect("writes to work if reader is present")
    });

    let mut received = String::new();
    reader.read_to_string(&mut received).unwrap();

    assert_eq!(&received, message);
}

#[test]
fn write_failure_propagates() {
    let (mut writer, _) = pipe::unidirectional(None);
    assert_eq!(
        writer.write_all(b"must fail").unwrap_err().kind(),
        ErrorKind::BrokenPipe
    );
}

#[test]
fn continue_on_empty_writes() {
    let (mut writer, mut reader) = pipe::unidirectional(2);
    writer.write(&[]).expect("write successful and non-blocking");
    let input = b"hello";
    writer
        .write(input)
        .expect("second write works as well as there is capacity");
    let mut buf = vec![0u8; input.len()];
    assert_eq!(reader.read(&mut buf).expect("read succeeds"), input.len());
    assert_eq!(buf, &input[..]);
}

#[test]
fn small_reads() {
    const BLOCK_SIZE: usize = 20;
    let block_count = 20;
    let (mut writer, mut reader) = pipe::unidirectional(Some(4));
    std::thread::spawn(move || {
        for _ in 0..block_count {
            let data = &[0; BLOCK_SIZE];
            writer.write_all(data).unwrap();
        }
    });

    let mut small_read_buf = [0; BLOCK_SIZE / 2];
    let mut bytes_read = 0;
    while let Ok(size) = reader.read(&mut small_read_buf) {
        if size == 0 {
            break;
        }
        bytes_read += size;
    }
    assert_eq!(block_count * BLOCK_SIZE, bytes_read);
}
