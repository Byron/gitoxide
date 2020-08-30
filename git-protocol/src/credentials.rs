use git_transport::client;
use quick_error::quick_error;
use std::{
    io::{self, Write},
    process::{Command, Stdio},
};

pub type Result = std::result::Result<Outcome, Error>;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: io::Error) {
            display("An IO error occurred while communicating to the credentials helper")
            from()
            source(err)
        }
        KeyNotFound(name: String) {
            display("Could not find '{}' in output of git credentials helper", name)
        }
        CredentialsHelperFailed(code: Option<i32>) {
            display("Credentials helper program failed with status code {:?}", code)
        }
    }
}

#[derive(Clone, Debug)]
pub enum Action {
    /// Provide credentials using the given URL (as String) as context
    Fill(String),
    Approve(Vec<u8>),
    Reject(Vec<u8>),
}

impl Action {
    pub fn as_str(&self) -> &str {
        match self {
            Action::Approve(_) => "approve",
            Action::Fill(_) => "fill",
            Action::Reject(_) => "reject",
        }
    }
}

#[derive(Clone, Debug)]
pub struct NextAction {
    previous_output: Vec<u8>,
}

impl NextAction {
    pub fn approve(self) -> Action {
        Action::Approve(self.previous_output)
    }
    pub fn reject(self) -> Action {
        Action::Reject(self.previous_output)
    }
}

pub struct Outcome {
    pub identity: client::Identity,
    pub next: NextAction,
}

#[cfg(windows)]
fn git_program() -> &'static str {
    "git.exe"
}

#[cfg(not(windows))]
fn git_program() -> &'static str {
    "git"
}

pub fn helper(action: Action) -> Result {
    let mut cmd = Command::new(git_program());
    cmd.arg("credential")
        .arg(action.as_str())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped());
    let mut child = cmd.spawn()?;
    let mut stdin = child.stdin.take().expect("stdin to be configured");

    match action {
        Action::Fill(url) => encode_message(&url, stdin)?,
        Action::Approve(last) | Action::Reject(last) => stdin.write_all(&last)?,
    }

    let output = child.wait_with_output()?;
    if !output.status.success() {
        return Err(Error::CredentialsHelperFailed(output.status.code()));
    }
    let stdout = output.stdout;
    let kvs = decode_message(stdout.as_slice())?;
    let find = |name: &str| {
        kvs.iter()
            .find(|(k, _)| k == name)
            .ok_or_else(|| Error::KeyNotFound(name.into()))
            .map(|(_, n)| n.to_owned())
    };
    Ok(Outcome {
        identity: client::Identity::Account {
            username: find("username")?,
            password: find("password")?,
        },
        next: NextAction {
            previous_output: stdout,
        },
    })
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
