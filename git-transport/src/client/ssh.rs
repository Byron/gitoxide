use crate::client::git;
use crate::Protocol;
use quick_error::quick_error;
use std::borrow::Cow;
use std::{path::Path, process};

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
    _path: &Path,
    version: crate::Protocol,
    _user: Option<&str>,
    port: Option<u16>,
) -> Result<git::Connection<process::ChildStdout, process::ChildStdin>, Error> {
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

    let mut cmd = std::process::Command::new(ssh_cmd);
    cmd.args(ssh_cmd_line);
    if let Some((args, envs)) = args_and_env {
        cmd.args(args.iter().map(|s| s.as_ref()));
        cmd.envs(envs);
    }
    cmd.arg(host);

    unimplemented!("file connection")
}
