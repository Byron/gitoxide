use crate::{client, Protocol};
use bstr::{BString, ByteSlice, ByteVec};
use quick_error::quick_error;
use std::borrow::Cow;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        UnsupportedSshCommand(command: String) {
            display("The ssh command '{}' is not currently supported", command)
        }
    }
}

pub fn connect(
    host: &str,
    path: BString,
    version: crate::Protocol,
    user: Option<&str>,
    port: Option<u16>,
) -> Result<client::file::SpawnProcessOnDemand, Error> {
    let ssh_cmd_line = std::env::var("GIT_SSH_COMMAND").unwrap_or_else(|_| "ssh".into());
    let mut ssh_cmd_line = ssh_cmd_line.split(' ');
    let ssh_cmd = ssh_cmd_line.next().expect("there is always a single item");

    type EnvVar = (&'static str, String);
    let args_and_env: Option<(Vec<Cow<str>>, Vec<EnvVar>)> = match ssh_cmd {
        "ssh" | "ssh.exe" => {
            if version != Protocol::V1 {
                let mut args = vec![Cow::from("-o"), "SendEnv=GIT_PROTOCOL".into()];
                if let Some(port) = port {
                    args.push(format!("-p={}", port).into());
                }
                Some((args, vec![("GIT_PROTOCOL", format!("version={}", version as usize))]))
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

    let path = match git_url::expand_path::parse(path.as_slice().as_bstr()) {
        Ok((user, mut path)) => match user {
            Some(git_url::expand_path::ForUser::Current) => {
                path.insert(0, b'~');
                path
            }
            Some(git_url::expand_path::ForUser::Name(mut user)) => {
                user.insert(0, b'~');
                user.append(path.as_vec_mut());
                user
            }
            None => path,
        },
        Err(_) => path,
    };

    let url = git_url::Url {
        scheme: git_url::Scheme::Ssh,
        user: user.map(Into::into),
        host: Some(host.clone()),
        port,
        path: path.clone(),
    };
    Ok(match args_and_env {
        Some((args, envs)) => client::file::SpawnProcessOnDemand::new_ssh(
            url,
            ssh_cmd.into(),
            ssh_cmd_line.map(Cow::from).chain(args).chain(Some(host.into())),
            envs,
            path,
        ),
        None => client::file::SpawnProcessOnDemand::new_ssh(
            url,
            ssh_cmd.into(),
            ssh_cmd_line.chain(Some(host.as_str())),
            None::<(&str, String)>,
            path,
        ),
    })
}

#[cfg(test)]
mod tests {
    use crate::{client::ssh::connect, Protocol};

    #[test]
    fn connect_with_tilde_in_path() {
        let url = git_url::parse(b"ssh://host.xy/~/repo").expect("valid url");
        let cmd = connect("host", url.path, Protocol::V1, None, None).expect("parse success");
        assert_eq!(
            cmd.path, "~/repo",
            "the path is prepared to be substituted by the remote shell"
        );
    }

    #[test]
    fn connect_with_tilde_and_user_in_path() {
        let url = git_url::parse(b"ssh://host.xy/~username/repo").expect("valid url");
        let cmd = connect("host", url.path, Protocol::V1, None, None).expect("parse success");
        assert_eq!(
            cmd.path, "~username/repo",
            "the path is prepared to be substituted by the remote shell or git-upload-pack"
        );
    }
}
