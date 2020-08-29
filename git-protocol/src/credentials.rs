use quick_error::quick_error;
use std::io;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: io::Error) {
            display("An IO error occurred while communicating to the credentials helper")
            from()
            source(err)
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Action {
    Fill,
    Approve,
    Reject,
}

pub fn credential(_url: &str, _action: Action) -> Result<git_transport::client::Identity, Error> {
    unimplemented!("credential")
}

pub fn encode_message(url: &str, mut out: impl io::Write) -> io::Result<()> {
    validate(url)?;
    writeln!(out, "url={}\n", url)
}

fn validate(url: &str) -> io::Result<()> {
    if url.contains('\u{0}') || url.contains('\n') {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "token to encode must not contain newlines or null bytes",
        ));
    }
    Ok(())
}

pub fn decode_message(mut input: impl io::Read) -> io::Result<Vec<(String, String)>> {
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;
    buf.lines()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let mut iter = l.splitn(2, '=').map(|s| s.to_owned());
            match (iter.next(), iter.next()) {
                (Some(key), Some(value)) => validate(&key).and_then(|_| validate(&value)).map(|_| (key, value)),
                _ => Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Invalid format, expecting key=value",
                )),
            }
        })
        .collect::<io::Result<Vec<_>>>()
}
