use std::ffi::OsStr;

use bstr::{BString, ByteSlice, ByteVec};

use crate::{
    client::{ssh, ssh::ProgramKind},
    Protocol,
};

impl ProgramKind {
    /// Provide the name of the executable that belongs to this kind, or `None` if the kind is `Simple`.
    pub fn exe(&self) -> Option<&'static OsStr> {
        Some(OsStr::new(match self {
            ProgramKind::Ssh => "ssh",
            ProgramKind::Plink => "plink",
            ProgramKind::Putty => "putty",
            ProgramKind::TortoisePlink => "tortoiseplink.exe",
            ProgramKind::Simple => return None,
        }))
    }

    /// Prepare all information needed to invoke the ssh command
    pub(crate) fn prepare_invocation(
        &self,
        ssh_cmd: &OsStr,
        url: &git_url::Url,
        desired_version: Protocol,
        disallow_shell: bool,
    ) -> Result<git_command::Prepare, ssh::invocation::Error> {
        let mut prepare = git_command::prepare(ssh_cmd).with_shell();
        if disallow_shell {
            prepare.use_shell = false;
        }
        let host = url.host().expect("present in ssh urls");
        match self {
            ProgramKind::Ssh => {
                if desired_version != Protocol::V1 {
                    prepare = prepare
                        .args(["-o", "SendEnv=GIT_PROTOCOL"])
                        .env("GIT_PROTOCOL", format!("version={}", desired_version as usize))
                }
                if let Some(port) = url.port {
                    prepare = prepare.arg(format!("-p{}", port));
                }
            }
            ProgramKind::Plink | ProgramKind::Putty | ProgramKind::TortoisePlink => {
                if *self == ProgramKind::TortoisePlink {
                    prepare = prepare.arg("-batch");
                }
                if let Some(port) = url.port {
                    prepare = prepare.arg(format!("-P{}", port));
                }
            }
            ProgramKind::Simple => {
                if url.port.is_some() {
                    return Err(ssh::invocation::Error {
                        command: ssh_cmd.into(),
                        function: "setting the port",
                    });
                }
            }
        };
        let host_as_ssh_arg = match url.user() {
            Some(user) => format!("{user}@{host}"),
            None => host.into(),
        };

        // Try to force ssh to yield english messages (for parsing later)
        Ok(prepare.arg(host_as_ssh_arg).env("LANG", "C").env("LC_ALL", "C"))
    }

    /// Note that the caller has to assure that the ssh program is launched in English by setting the locale.
    pub(crate) fn line_to_permission_err(&self, line: BString) -> Result<std::io::Error, BString> {
        let is_perm_err = match self {
            ProgramKind::Ssh | ProgramKind::Simple => {
                line.contains_str(b"Permission denied") || line.contains_str(b"permission denied")
            }
            ProgramKind::Plink | ProgramKind::Putty | ProgramKind::TortoisePlink => line.contains_str(b"publickey"),
        };
        if is_perm_err {
            Ok(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                Vec::from(line).into_string_lossy(),
            ))
        } else {
            Err(line)
        }
    }
}

impl<'a> From<&'a OsStr> for ProgramKind {
    fn from(v: &'a OsStr) -> Self {
        let p = std::path::Path::new(v);
        match p.file_stem().and_then(|s| s.to_str()) {
            None => ProgramKind::Simple,
            Some(stem) => {
                if stem.eq_ignore_ascii_case("ssh") {
                    ProgramKind::Ssh
                } else if stem.eq_ignore_ascii_case("plink") {
                    ProgramKind::Plink
                } else if stem.eq_ignore_ascii_case("putty") {
                    ProgramKind::Putty
                } else if stem.eq_ignore_ascii_case("tortoiseplink") {
                    ProgramKind::TortoisePlink
                } else {
                    ProgramKind::Simple
                }
            }
        }
    }
}
