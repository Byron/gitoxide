use std::process::{self, Command, Stdio};

use bstr::{BString, ByteSlice};

use crate::client::git;
use crate::{
    client::{self, MessageKind, RequestWriter, SetServiceResponse, WriteMode},
    Protocol, Service,
};

// from https://github.com/git/git/blob/20de7e7e4f4e9ae52e6cc7cfaa6469f186ddb0fa/environment.c#L115:L115
const ENV_VARS_TO_REMOVE: &[&str] = &[
    "GIT_ALTERNATE_OBJECT_DIRECTORIES",
    "GIT_CONFIG",
    "GIT_CONFIG_PARAMETERS",
    "GIT_OBJECT_DIRECTORY",
    "GIT_DIR",
    "GIT_WORK_TREE",
    "GIT_IMPLICIT_WORK_TREE",
    "GIT_GRAFT_FILE",
    "GIT_INDEX_FILE",
    "GIT_NO_REPLACE_OBJECTS",
    "GIT_REPLACE_REF_BASE",
    "GIT_PREFIX",
    "GIT_INTERNAL_SUPER_PREFIX",
    "GIT_SHALLOW_FILE",
    "GIT_COMMON_DIR",
];

/// A utility to spawn a helper process to actually transmit data, possibly over `ssh`.
///
/// It can only be instantiated using the local [`connect()`] or [ssh connect][crate::client::ssh::connect()].
pub struct SpawnProcessOnDemand {
    desired_version: Protocol,
    url: git_url::Url,
    pub(crate) path: BString,
    ssh_program: Option<String>,
    ssh_args: Vec<String>,
    ssh_env: Vec<(&'static str, String)>,
    connection: Option<git::Connection<process::ChildStdout, process::ChildStdin>>,
    child: Option<process::Child>,
}

impl Drop for SpawnProcessOnDemand {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            child.wait().ok();
        }
    }
}

impl SpawnProcessOnDemand {
    pub(crate) fn new_ssh(
        url: git_url::Url,
        program: String,
        args: impl IntoIterator<Item = impl Into<String>>,
        env: impl IntoIterator<Item = (&'static str, impl Into<String>)>,
        path: BString,
        version: Protocol,
    ) -> SpawnProcessOnDemand {
        SpawnProcessOnDemand {
            url,
            path,
            ssh_program: Some(program),
            ssh_args: args.into_iter().map(|s| s.into()).collect(),
            ssh_env: env.into_iter().map(|(k, v)| (k, v.into())).collect(),
            child: None,
            connection: None,
            desired_version: version,
        }
    }
    pub(crate) fn new_local(path: BString, version: Protocol) -> SpawnProcessOnDemand {
        SpawnProcessOnDemand {
            url: git_url::Url {
                scheme: git_url::Scheme::File,
                user: None,
                host: None,
                port: None,
                path: path.clone(),
            },
            path,
            ssh_program: None,
            ssh_args: Vec::new(),
            ssh_env: Vec::new(),
            child: None,
            connection: None,
            desired_version: version,
        }
    }
}

impl client::Transport for SpawnProcessOnDemand {
    fn handshake(&mut self, service: Service) -> Result<SetServiceResponse<'_>, client::Error> {
        assert!(
            self.connection.is_none(),
            "cannot handshake twice with the same connection"
        );
        let mut cmd = match &self.ssh_program {
            Some(program) => Command::new(program),
            None => Command::new(service.as_str()),
        };
        for env_to_remove in ENV_VARS_TO_REMOVE {
            cmd.env_remove(env_to_remove);
        }
        cmd.envs(std::mem::take(&mut self.ssh_env));
        cmd.args(&mut self.ssh_args);
        cmd.stdin(Stdio::piped()).stdout(Stdio::piped());
        if self.ssh_program.is_some() {
            cmd.arg(service.as_str());
        }
        cmd.arg("--strict").arg("--timeout=0").arg(self.path.to_os_str_lossy());

        let mut child = cmd.spawn()?;
        self.connection = Some(git::Connection::new_for_spawned_process(
            child.stdout.take().expect("stdout configured"),
            child.stdin.take().expect("stdin configured"),
            self.desired_version,
            self.path.clone(),
        ));
        self.child = Some(child);
        let c = self
            .connection
            .as_mut()
            .expect("connection to be there right after setting it");
        c.handshake(service)
    }

    fn request(
        &mut self,
        write_mode: WriteMode,
        on_into_read: MessageKind,
    ) -> Result<RequestWriter<'_>, client::Error> {
        self.connection
            .as_mut()
            .expect("handshake() to have been called first")
            .request(write_mode, on_into_read)
    }

    fn close(&mut self) -> Result<(), client::Error> {
        if let Some(mut c) = self.connection.take() {
            c.close()
        } else {
            Ok(())
        }
    }

    fn to_url(&self) -> String {
        self.url.to_string()
    }

    fn desired_protocol_version(&self) -> Protocol {
        self.desired_version
    }

    fn is_stateful(&self) -> bool {
        true
    }
}

/// Connect to a locally readable repository at `path` using the given `desired_version`.
///
/// This will spawn a `git` process locally.
pub fn connect(
    path: impl Into<BString>,
    desired_version: Protocol,
) -> Result<SpawnProcessOnDemand, std::convert::Infallible> {
    Ok(SpawnProcessOnDemand::new_local(path.into(), desired_version))
}
