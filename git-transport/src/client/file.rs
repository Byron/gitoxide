use crate::{
    client::{self, git, MessageKind, RequestWriter, SetServiceResponse, WriteMode},
    Service,
};
use bstr::{BString, ByteSlice};
use quick_error::quick_error;
use std::process::{self, Stdio};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Tbd {
            display("tbd")
        }
    }
}

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

pub struct SpawnProcessOnDemand {
    path: BString,
    connection: Option<git::Connection<process::ChildStdout, process::ChildStdin>>,
}

impl client::Transport for SpawnProcessOnDemand {
    fn handshake(&mut self, service: Service) -> Result<SetServiceResponse, client::Error> {
        assert!(
            self.connection.is_none(),
            "cannot handshake twice with the same connection"
        );
        let mut cmd = std::process::Command::new(service.as_str());
        for env_to_remove in ENV_VARS_TO_REMOVE {
            cmd.env_remove(env_to_remove);
        }
        cmd.stderr(Stdio::null()).stdout(Stdio::piped()).stdin(Stdio::piped());
        cmd.arg("--strict").arg("--timeout=0").arg(self.path.to_os_str_lossy());

        let child = cmd.spawn()?;
        self.connection = Some(git::Connection::new_for_spawned_process(
            child.stdout.expect("stdout configured"),
            child.stdin.expect("stdin configured"),
            self.path.clone(),
        ));
        let c = self
            .connection
            .as_mut()
            .expect("connection to be there right after setting it");
        c.handshake(service)
    }

    fn request(&mut self, write_mode: WriteMode, on_drop: Vec<MessageKind>) -> Result<RequestWriter, client::Error> {
        self.connection
            .as_mut()
            .expect("handshake() to have been called first")
            .request(write_mode, on_drop)
    }
}

pub fn connect(path: impl Into<BString>) -> Result<SpawnProcessOnDemand, std::convert::Infallible> {
    Ok(SpawnProcessOnDemand {
        path: path.into(),
        connection: None,
    })
}
