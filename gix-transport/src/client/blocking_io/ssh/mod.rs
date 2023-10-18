use std::process::Stdio;

use crate::{client::blocking_io, Protocol};

/// The error used in [`connect()`].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("The scheme in \"{}\" is not usable for an ssh connection", .0.to_bstring())]
    UnsupportedScheme(gix_url::Url),
    #[error("Host name '{host}' could be mistaken for a command-line argument")]
    AmbiguousHostName { host: String },
}

impl crate::IsSpuriousError for Error {}

/// The kind of SSH programs we have built-in support for.
///
/// Various different programs exists with different capabilities, and we have a few built in.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ProgramKind {
    /// The standard linux ssh program
    Ssh,
    /// The `(plink|putty).exe` binaries, typically only on windows.
    Plink,
    /// The `putty.exe` binary, typically only on windows.
    Putty,
    /// The `tortoiseplink.exe` binary, only on windows.
    TortoisePlink,
    /// A minimal ssh client that supports on options.
    Simple,
}

mod program_kind;

///
pub mod invocation {
    use std::ffi::OsString;

    /// The error returned when producing ssh invocation arguments based on a selected invocation kind.
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("Host name '{host}' could be mistaken for a command-line argument")]
        AmbiguousHostName { host: String },
        #[error("The 'Simple' ssh variant doesn't support {function}")]
        Unsupported {
            /// The simple command that should have been invoked.
            command: OsString,
            /// The function that was unsupported
            function: &'static str,
        },
    }
}

///
pub mod connect {
    use std::ffi::{OsStr, OsString};

    use crate::client::ssh::ProgramKind;

    /// The options for use when [connecting][super::connect()] via the `ssh` protocol.
    #[derive(Debug, Clone, Default)]
    pub struct Options {
        /// The program or script to use.
        /// If unset, it defaults to `ssh` or `ssh.exe`, or the program implied by `kind` if that one is set.
        pub command: Option<OsString>,
        /// If `true`, a shell must not be used to execute `command`.
        /// This defaults to `false`, and a shell can then be used if `command` seems to require it, but won't be
        /// used unnecessarily.
        pub disallow_shell: bool,
        /// The ssh variant further identifying `program`. This determines which arguments will be used
        /// when invoking the program.
        /// If unset, the `program` basename determines the variant, or an invocation of the `command` itself.
        pub kind: Option<ProgramKind>,
    }

    impl Options {
        /// Return the configured ssh command, defaulting to `ssh` if neither the `command` nor the `kind` fields are set.
        pub fn ssh_command(&self) -> &OsStr {
            self.command
                .as_deref()
                .or_else(|| self.kind.and_then(|kind| kind.exe()))
                .unwrap_or_else(|| OsStr::new("ssh"))
        }
    }
}

/// Connect to `host` using the ssh program to obtain data from the repository at `path` on the remote.
///
/// The optional `user` identifies the user's account to which to connect, while `port` allows to specify non-standard
/// ssh ports.
///
/// The `desired_version` is the preferred protocol version when establishing the connection, but note that it can be
/// downgraded by servers not supporting it.
/// If `trace` is `true`, all packetlines received or sent will be passed to the facilities of the `gix-trace` crate.
#[allow(clippy::result_large_err)]
pub fn connect(
    url: gix_url::Url,
    desired_version: Protocol,
    options: connect::Options,
    trace: bool,
) -> Result<blocking_io::file::SpawnProcessOnDemand, Error> {
    if url.scheme != gix_url::Scheme::Ssh || url.host().is_none() {
        return Err(Error::UnsupportedScheme(url));
    }
    let ssh_cmd = options.ssh_command();
    let mut kind = options.kind.unwrap_or_else(|| ProgramKind::from(ssh_cmd));
    if options.kind.is_none() && kind == ProgramKind::Simple {
        kind = if std::process::Command::from(
            gix_command::prepare(ssh_cmd)
                .stderr(Stdio::null())
                .stdout(Stdio::null())
                .stdin(Stdio::null())
                .with_shell()
                .arg("-G")
                .arg(url.host_argument_safe().ok_or_else(|| Error::AmbiguousHostName {
                    host: url.host().expect("set in ssh urls").into(),
                })?),
        )
        .status()
        .ok()
        .map_or(false, |status| status.success())
        {
            ProgramKind::Ssh
        } else {
            ProgramKind::Simple
        };
    }

    let path = gix_url::expand_path::for_shell(url.path.clone());
    Ok(blocking_io::file::SpawnProcessOnDemand::new_ssh(
        url,
        ssh_cmd,
        path,
        kind,
        options.disallow_shell,
        desired_version,
        trace,
    ))
}

#[cfg(test)]
mod tests;
