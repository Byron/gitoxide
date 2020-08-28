use crate::{
    client::{self, git, MessageKind, RequestWriter, SetServiceResponse, WriteMode},
    Service,
};
use quick_error::quick_error;
use std::process::Stdio;
use std::{
    path::{Path, PathBuf},
    process,
};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Tbd {
            display("tbd")
        }
    }
}

// from https://github.com/git/git/blob/20de7e7e4f4e9ae52e6cc7cfaa6469f186ddb0fa/environment.c#L115:L115
const ENV_VARS_TO_REMOVE: &'static [&'static str] = &[
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
    path: PathBuf,
    version: crate::Protocol,
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
        cmd.arg("--strict").arg("--timeout=0");
        let child = cmd.spawn()?;
        // self.connection = Some(git::Connection {})
        unimplemented!("invoke command")
    }

    fn request(&mut self, write_mode: WriteMode, on_drop: Vec<MessageKind>) -> Result<RequestWriter, client::Error> {
        unimplemented!()
    }
}

pub fn connect(path: &Path, version: crate::Protocol) -> Result<SpawnProcessOnDemand, std::convert::Infallible> {
    Ok(SpawnProcessOnDemand {
        path: path.to_owned(),
        version,
        connection: None,
    })
}
