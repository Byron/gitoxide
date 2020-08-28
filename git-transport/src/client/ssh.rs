use crate::{client, Protocol};
use bstr::BString;
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

    let args_and_env: Option<(Vec<Cow<str>>, Vec<(&'static str, String)>)> = match ssh_cmd {
        "ssh" | "ssh.exe" => {
            if version != Protocol::V1 {
                let mut args = vec![Cow::from("-o"), format!("SendEnv=GIT_PROTOCOL").into()];
                if let Some(port) = port {
                    args.push(format!("-p={}", port).into());
                }
                Some((
                    args,
                    vec![("GIT_PROTOCOL", format!("version={}", version as usize).into())],
                ))
            } else {
                None
            }
        }
        _ => return Err(Error::UnsupportedSshCommand(ssh_cmd.into())),
    };

    let host = match user {
        Some(user) => format!("{}@{}", user, host),
        None => host.into(),
    };
    Ok(match args_and_env {
        Some((args, envs)) => client::file::SpawnProcessOnDemand::new_ssh(
            ssh_cmd.into(),
            ssh_cmd_line.map(|s| Cow::from(s)).chain(args).chain(Some(host.into())),
            envs,
            path,
        ),
        None => client::file::SpawnProcessOnDemand::new_ssh(
            ssh_cmd.into(),
            ssh_cmd_line.chain(Some(host.as_str())),
            None::<(&str, String)>,
            path,
        ),
    })
}
