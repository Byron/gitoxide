use std::io;

#[derive(Clone, Copy, Debug)]
pub enum Action {
    Fill,
    Approve,
    Reject,
}

pub fn encode_message(url: &str, mut out: impl io::Write) -> io::Result<()> {
    if url.contains('\u{0}') || url.contains('\n') {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "token to encode must not contain newlines or null bytes",
        ));
    }
    writeln!(out, "url={}\n", url)
}
