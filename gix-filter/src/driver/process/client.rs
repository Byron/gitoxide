use crate::driver::process;
use crate::driver::process::{Client, PacketlineReader};
use bstr::{BString, ByteVec};
use std::collections::HashSet;
use std::io::Write;
use std::str::FromStr;

///
pub mod handshake {
    /// The error returned by [Client::handshake()][super::Client::handshake()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("Failed to read or write to the process")]
        Io(#[from] std::io::Error),
        #[error("{msg} '{actual}'")]
        Protocol { msg: String, actual: String },
        #[error("The server sent the '{name}' capability which isn't among the ones we desire can support")]
        UnsupportedCapability { name: String },
    }
}

///
pub mod invoke {
    /// The error returned by [Client::invoke()][super::Client::invoke()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("Failed to read or write to the process")]
        Io(#[from] std::io::Error),
    }
}

/// Protocol implementation
impl Client {
    /// Given a spawned `process` as created from `cmd`, use the 'long-running-process' protocol to send `welcome-prefix` and supported
    /// `versions`, along with the `desired_capabilities`, and perform the handshake to negotiate a version to use along with
    /// obtaining supported capabilities, which may be a sub-set of the desired capabilities.
    pub fn handshake(
        mut process: std::process::Child,
        welcome_prefix: &str,
        versions: &[usize],
        desired_capabilities: &[&str],
    ) -> Result<Self, handshake::Error> {
        let mut out = gix_packetline::Writer::new(process.stdin.take().expect("configured stdin when spawning"));
        out.write_all(format!("{welcome_prefix}-client").as_bytes())?;
        for version in versions {
            out.write_all(format!("version={version}").as_bytes())?;
        }
        gix_packetline::encode::flush_to_write(out.inner_mut())?;
        out.flush()?;

        let mut input = gix_packetline::StreamingPeekableIter::new(
            process.stdout.take().expect("configured stdout when spawning"),
            &[gix_packetline::PacketLineRef::Flush],
        );
        let mut read = input.as_read();
        let mut buf = String::new();
        read.read_line_to_string(&mut buf)?;
        if buf
            .strip_prefix(welcome_prefix)
            .map_or(true, |rest| rest.trim_end() != "-server")
        {
            return Err(handshake::Error::Protocol {
                msg: format!("Wanted '{welcome_prefix}-server, got "),
                actual: buf,
            });
        }

        let chosen_version;
        buf.clear();
        read.read_line_to_string(&mut buf)?;
        match buf
            .strip_prefix("version=")
            .and_then(|version| usize::from_str(version.trim_end()).ok())
        {
            Some(version) => {
                chosen_version = version;
            }
            None => {
                return Err(handshake::Error::Protocol {
                    msg: "Needed 'version=<integer>', got ".into(),
                    actual: buf,
                })
            }
        }

        if !versions.contains(&chosen_version) {
            return Err(handshake::Error::Protocol {
                msg: format!("Server offered {chosen_version}, we only support "),
                actual: versions.iter().map(ToString::to_string).collect::<Vec<_>>().join(", "),
            });
        }

        if read.read_line_to_string(&mut buf)? != 0 {
            return Err(handshake::Error::Protocol {
                msg: "expected flush packet, got".into(),
                actual: buf,
            });
        }
        for capability in desired_capabilities {
            out.write_all(format!("capability={capability}").as_bytes())?;
        }
        gix_packetline::encode::flush_to_write(out.inner_mut())?;
        out.flush()?;

        read.reset_with(&[gix_packetline::PacketLineRef::Flush]);
        let mut capabilities = HashSet::new();
        loop {
            buf.clear();
            let num_read = read.read_line_to_string(&mut buf)?;
            if num_read == 0 {
                break;
            }
            match buf.strip_prefix("capability=") {
                Some(cap) => {
                    let cap = cap.trim_end();
                    if !desired_capabilities.contains(&cap) {
                        return Err(handshake::Error::UnsupportedCapability { name: cap.into() });
                    }
                    capabilities.insert(cap.to_owned());
                }
                None => continue,
            }
        }

        drop(read);
        Ok(Client {
            child: process,
            out: input,
            input: out,
            capabilities,
            version: chosen_version,
        })
    }

    /// Invoke `command` and send all `meta` data before sending all `content` in full.
    pub fn invoke<'a>(
        &mut self,
        command: &str,
        meta: impl IntoIterator<Item = (&'a str, BString)>,
        mut content: impl std::io::Read,
    ) -> Result<process::Status, invoke::Error> {
        self.input.write_all(format!("command={command}").as_bytes())?;
        let mut buf = BString::default();
        for (key, value) in meta {
            buf.clear();
            buf.push_str(key);
            buf.push(b'=');
            buf.push_str(&value);
            self.input.write_all(&buf)?;
        }
        gix_packetline::encode::flush_to_write(self.input.inner_mut())?;
        std::io::copy(&mut content, &mut self.input)?;
        gix_packetline::encode::flush_to_write(self.input.inner_mut())?;
        self.input.flush()?;
        Ok(self.read_status()?)
    }

    /// Return a `Read` implementation that reads the server process output until the next flush package, and validates
    /// the status. If the status indicates failure, the last read will also fail.
    pub fn as_read(&mut self) -> impl std::io::Read + '_ {
        self.out.reset_with(&[gix_packetline::PacketLineRef::Flush]);
        ReadProcessOutputAndStatus {
            inner: self.out.as_read(),
        }
    }

    /// Read a `status=` line from the process output until it is exhausted.
    /// Note that the last sent status line wins and no status line means that the `Previous` still counts.
    pub fn read_status(&mut self) -> std::io::Result<process::Status> {
        read_status(&mut self.out.as_read())
    }
}

fn read_status(read: &mut PacketlineReader<'_>) -> std::io::Result<process::Status> {
    let mut status = process::Status::Previous;
    let mut buf = String::new();
    loop {
        buf.clear();
        let num_read = read.read_line_to_string(&mut buf)?;
        if num_read == 0 {
            break;
        }
        if let Some(name) = buf.strip_prefix("status=") {
            status = process::Status::Named(name.trim_end().into());
        }
    }
    read.reset_with(&[gix_packetline::PacketLineRef::Flush]);
    Ok(status)
}

struct ReadProcessOutputAndStatus<'a> {
    inner: PacketlineReader<'a>,
}

impl<'a> std::io::Read for ReadProcessOutputAndStatus<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let num_read = self.inner.read(buf)?;
        if num_read == 0 {
            self.inner.reset_with(&[gix_packetline::PacketLineRef::Flush]);
            let status = read_status(&mut self.inner)?;
            if status.is_success() {
                Ok(0)
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "Process indicated error after reading: {}",
                        status.message().unwrap_or_default()
                    ),
                ))
            }
        } else {
            Ok(num_read)
        }
    }
}

/// Access
impl Client {
    /// Return the list of capabilities reported by the serving process.
    pub fn capabilities(&self) -> &HashSet<String> {
        &self.capabilities
    }

    /// Return the negotiated version of the protocol.
    ///
    /// Note that it is the highest one that both the client and the server support.
    pub fn version(&self) -> usize {
        self.version
    }
}

/// Lifecycle
impl Client {
    /// Return the child handle of the running process.
    ///
    /// Note that this will naturally close input and output handles, which is a signal for the child process to shutdown.
    pub fn into_child(self) -> std::process::Child {
        self.child
    }
}
