use std::{io::BufRead, time::Duration};

use bstr::ByteSlice;

pub(crate) fn response_with_connection_close(response: &[u8]) -> Vec<u8> {
    let split = response
        .find(b"\n\n")
        .expect("response fixture with header/body separator");
    let (headers, body) = response.split_at(split);
    let body = &body[2..];
    let headers = std::str::from_utf8(headers).expect("fixture headers are UTF-8");
    // Text fixtures end with a newline which is not part of the final flush packet.
    let body = if body.ends_with(b"0000\n") {
        &body[..body.len() - 1]
    } else {
        body
    };

    let mut out = Vec::with_capacity(response.len() + 64);
    for line in headers.lines() {
        if line.to_ascii_lowercase().starts_with("content-length:") {
            out.extend_from_slice(format!("Content-Length: {}", body.len()).as_bytes());
        } else {
            out.extend_from_slice(line.as_bytes());
        }
        out.extend_from_slice(b"\r\n");
    }
    out.extend_from_slice(b"Connection: close\r\n\r\n");
    out.extend_from_slice(body);
    out
}

pub(crate) fn read_request_lines(reader: &mut dyn BufRead) -> Vec<String> {
    reader
        .lines()
        .map(Result::unwrap)
        .take_while(|line| !line.is_empty() && line != "\r")
        .map(|line| line.trim().to_string())
        .collect()
}

pub(crate) fn observe_connection_within_deadline(listener: std::net::TcpListener) -> std::thread::JoinHandle<bool> {
    listener
        .set_nonblocking(true)
        .expect("nonblocking listener can be configured");
    std::thread::spawn(move || {
        let deadline = std::time::Instant::now() + Duration::from_millis(250);
        loop {
            match listener.accept() {
                Ok((_stream, _)) => return true,
                Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                    if std::time::Instant::now() >= deadline {
                        return false;
                    }
                    std::thread::sleep(Duration::from_millis(10));
                }
                Err(err) => panic!("accept should work: {err}"),
            }
        }
    })
}
