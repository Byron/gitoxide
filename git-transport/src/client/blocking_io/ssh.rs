use std::borrow::Cow;

use bstr::BString;

use crate::{client::blocking_io, Protocol};

/// The error used in [`connect()`].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("The ssh command {0:?} is not currently supported")]
    UnsupportedSshCommand(String),
}

/// Connect to `host` using the ssh program to obtain data from the repository at `path` on the remote.
///
/// The optional `user` identifies the user's account to which to connect, while `port` allows to specify non-standard
/// ssh ports.
///
/// The `desired_version` is the preferred protocol version when establishing the connection, but note that it can be
/// downgraded by servers not supporting it.
///
/// # Environment Variables
///
/// Use `GIT_SSH_COMMAND` to override the `ssh` program to execute. This can be a script dealing with using the correct
/// ssh key, for example.
pub fn connect(
    host: &str,
    path: BString,
    desired_version: crate::Protocol,
    user: Option<&str>,
    port: Option<u16>,
) -> Result<blocking_io::file::SpawnProcessOnDemand, Error> {
    let ssh_cmd_line = std::env::var("GIT_SSH_COMMAND").unwrap_or_else(|_| "ssh".into());
    let mut ssh_cmd_line = ssh_cmd_line.split(' ');
    let ssh_cmd = ssh_cmd_line.next().expect("there is always a single item");

    type EnvVar = (&'static str, String);
    let args_and_env: Option<(Vec<Cow<'_, str>>, Vec<EnvVar>)> = match ssh_cmd {
        "ssh" | "ssh.exe" => {
            if desired_version != Protocol::V1 {
                let mut args = vec![Cow::from("-o"), "SendEnv=GIT_PROTOCOL".into()];
                if let Some(port) = port {
                    args.push(format!("-p{}", port).into());
                }
                Some((
                    args,
                    vec![("GIT_PROTOCOL", format!("version={}", desired_version as usize))],
                ))
            } else {
                None
            }
        }
        _ => return Err(Error::UnsupportedSshCommand(ssh_cmd.into())),
    };

    let host = match user.as_ref() {
        Some(user) => format!("{}@{}", user, host),
        None => host.into(),
    };

    let path = git_url::expand_path::for_shell(path);
    let url = git_url::Url::from_parts(
        git_url::Scheme::Ssh,
        user.map(Into::into),
        Some(host.clone()),
        port,
        path.clone(),
    )
    .expect("valid url");
    Ok(match args_and_env {
        Some((args, envs)) => blocking_io::file::SpawnProcessOnDemand::new_ssh(
            url,
            ssh_cmd.into(),
            ssh_cmd_line.map(Cow::from).chain(args).chain(Some(host.into())),
            envs,
            path,
            desired_version,
        ),
        None => blocking_io::file::SpawnProcessOnDemand::new_ssh(
            url,
            ssh_cmd.into(),
            ssh_cmd_line.chain(Some(host.as_str())),
            None::<(&str, String)>,
            path,
            desired_version,
        ),
    })
}

#[cfg(test)]
mod tests {
    use bstr::ByteSlice;

    use crate::{client::blocking_io::ssh::connect, Protocol};

    #[test]
    fn connect_with_tilde_in_path() {
        for (url, expected) in &[
            ("ssh://host.xy/~/repo", "~/repo"),
            ("ssh://host.xy/~username/repo", "~username/repo"),
        ] {
            let url = git_url::parse((*url).into()).expect("valid url");
            let cmd = connect("host", url.path, Protocol::V1, None, None).expect("parse success");
            assert_eq!(
                cmd.path,
                expected.as_bytes().as_bstr(),
                "the path is prepared to be substituted by the remote shell"
            );
        }
    }
}
