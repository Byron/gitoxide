use std::{
    io::{self, Write},
    process::{Command, Stdio},
};

use quick_error::quick_error;

/// The result used in [`action()`].
pub type Result = std::result::Result<Option<Outcome>, Error>;

quick_error! {
    /// The error used in the [credentials helper][action()].
    #[derive(Debug)]
    #[allow(missing_docs)]
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

/// The action to perform by the credentials [`action()`].
#[derive(Clone, Debug)]
pub enum Action<'a> {
    /// Provide credentials using the given repository URL (as &str) as context.
    Fill(&'a str),
    /// Approve the credentials as identified by the previous input as `Vec<u8>`.
    Approve(Vec<u8>),
    /// Reject the credentials as identified by the previous input as `Vec<u8>`.
    Reject(Vec<u8>),
}

impl<'a> Action<'a> {
    fn is_fill(&self) -> bool {
        matches!(self, Action::Fill(_))
    }
    fn as_str(&self) -> &str {
        match self {
            Action::Approve(_) => "approve",
            Action::Fill(_) => "fill",
            Action::Reject(_) => "reject",
        }
    }
}

/// A handle to [approve][NextAction::approve()] or [reject][NextAction::reject()] the outcome of the initial action.
#[derive(Clone, Debug)]
pub struct NextAction {
    previous_output: Vec<u8>,
}

impl NextAction {
    /// Approve the result of the previous [Action].
    pub fn approve(self) -> Action<'static> {
        Action::Approve(self.previous_output)
    }
    /// Reject the result of the previous [Action].
    pub fn reject(self) -> Action<'static> {
        Action::Reject(self.previous_output)
    }
}

/// The outcome of [`action()`].
pub struct Outcome {
    /// The obtained identity.
    pub identity: git_sec::identity::Account,
    /// A handle to the action to perform next using another call to [`action()`].
    pub next: NextAction,
}

// TODO(sec): reimplement helper execution so it won't use the `git credential` anymore to allow enforcing our own security model.
//            Currently we support more flexible configuration than downright not working at all.
/// Call the `git` credentials helper program performing the given `action`.
///
/// Usually the first call is performed with [`Action::Fill`] to obtain an identity, which subsequently can be used.
/// On successful usage, use [`NextAction::approve()`], otherwise [`NextAction::reject()`].
pub fn action(action: Action<'_>) -> Result {
    let mut cmd = Command::new(cfg!(windows).then(|| "git.exe").unwrap_or("git"));
    cmd.arg("credential")
        .arg(action.as_str())
        .stdin(Stdio::piped())
        .stdout(if action.is_fill() {
            Stdio::piped()
        } else {
            Stdio::null()
        });
    let mut child = cmd.spawn()?;
    let mut stdin = child.stdin.take().expect("stdin to be configured");

    match action {
        Action::Fill(url) => encode_message(url, stdin)?,
        Action::Approve(last) | Action::Reject(last) => {
            stdin.write_all(&last)?;
            stdin.write_all(&[b'\n'])?
        }
    }

    let output = child.wait_with_output()?;
    if !output.status.success() {
        return Err(Error::CredentialsHelperFailed(output.status.code()));
    }
    let stdout = output.stdout;
    if stdout.is_empty() {
        Ok(None)
    } else {
        let kvs = decode_message(stdout.as_slice())?;
        let find = |name: &str| {
            kvs.iter()
                .find(|(k, _)| k == name)
                .ok_or_else(|| Error::KeyNotFound(name.into()))
                .map(|(_, n)| n.to_owned())
        };
        Ok(Some(Outcome {
            identity: git_sec::identity::Account {
                username: find("username")?,
                password: find("password")?,
            },
            next: NextAction {
                previous_output: stdout,
            },
        }))
    }
}

/// Encode `url` to `out` for consumption by a `git credentials` helper program.
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

/// Decode all lines in `input` as key-value pairs produced by a `git credentials` helper program.
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

#[cfg(test)]
mod tests {
    use super::*;
    type Result = std::result::Result<(), Box<dyn std::error::Error>>;

    mod encode_message {
        use bstr::ByteSlice;

        use super::*;

        #[test]
        fn from_url() -> super::Result {
            let mut out = Vec::new();
            encode_message("https://github.com/byron/gitoxide", &mut out)?;
            assert_eq!(out.as_bstr(), b"url=https://github.com/byron/gitoxide\n\n".as_bstr());
            Ok(())
        }

        mod invalid {
            use std::io;

            use super::*;

            #[test]
            fn contains_null() {
                assert_eq!(
                    encode_message("https://foo\u{0}", Vec::new()).err().map(|e| e.kind()),
                    Some(io::ErrorKind::Other)
                );
            }
            #[test]
            fn contains_newline() {
                assert_eq!(
                    encode_message("https://foo\n", Vec::new()).err().map(|e| e.kind()),
                    Some(io::ErrorKind::Other)
                );
            }
        }
    }

    mod decode_message {
        use super::*;

        #[test]
        fn typical_response() -> super::Result {
            assert_eq!(
                decode_message(
                    "protocol=https
host=example.com
username=bob
password=secr3t\n\n
this=is-skipped-past-empty-line"
                        .as_bytes()
                )?,
                vec![
                    ("protocol", "https"),
                    ("host", "example.com"),
                    ("username", "bob"),
                    ("password", "secr3t")
                ]
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect::<Vec<_>>()
            );
            Ok(())
        }
    }
}
