use crate::Protocol;
use bstr::BString;

/// The way to connect to a process speaking the `git` protocol.
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ConnectMode {
    /// A git daemon.
    Daemon,
    /// A spawned `git` process to upload a pack to the client.
    Process,
}

/// A TCP connection to either a `git` daemon or a spawned `git` process.
///
/// When connecting to a daemon, additional context information is sent with the first line of the handshake. Otherwise that
/// context is passed using command line arguments to a [spawned `git` process][crate::client::file::SpawnProcessOnDemand].
pub struct Connection<R, W> {
    pub(in crate::client) writer: W,
    pub(in crate::client) line_provider: git_packetline::StreamingPeekableIter<R>,
    pub(in crate::client) path: BString,
    pub(in crate::client) virtual_host: Option<(String, Option<u16>)>,
    pub(in crate::client) desired_version: Protocol,
    supported_versions: [Protocol; 1],
    pub(in crate::client) mode: ConnectMode,
}

impl<R, W> Connection<R, W> {
    /// Return the inner reader and writer
    pub fn into_inner(self) -> (R, W) {
        (self.line_provider.into_inner(), self.writer)
    }
}

pub(crate) mod message {
    use crate::{Protocol, Service};
    use bstr::{BString, ByteVec};

    pub fn connect(
        service: Service,
        version: Protocol,
        path: &[u8],
        virtual_host: Option<&(String, Option<u16>)>,
        extra_parameters: &[(&str, Option<&str>)],
    ) -> BString {
        let mut out = bstr::BString::from(service.as_str());
        out.push(b' ');
        let path = git_url::expand_path::for_shell(path.into());
        out.extend_from_slice(&path);
        out.push(0);
        if let Some((host, port)) = virtual_host {
            out.push_str("host=");
            out.extend_from_slice(host.as_bytes());
            if let Some(port) = port {
                out.push_byte(b':');
                out.push_str(&format!("{}", port));
            }
            out.push(0);
        }
        // We only send the version when needed, as otherwise a V2 server who is asked for V1 will respond with 'version 1'
        // as extra lines in the reply, which we don't want to handle. Especially since an old server will not respond with that
        // line (is what I assume, at least), so it's an optional part in the response to understand and handle. There is no value
        // in that, so let's help V2 servers to respond in a way that assumes V1.
        let needs_null_prefix = if version != Protocol::V1 {
            out.push(0);
            out.push_str(format!("version={}", version as usize));
            out.push(0);
            false
        } else {
            true
        };

        if !extra_parameters.is_empty() {
            if needs_null_prefix {
                out.push(0);
            }
            for (key, value) in extra_parameters {
                match value {
                    Some(value) => out.push_str(format!("{}={}", key, value)),
                    None => out.push_str(key),
                }
                out.push(0);
            }
        }
        out
    }
    #[cfg(test)]
    mod tests {
        use crate::{client::git, Protocol, Service};

        #[test]
        fn version_1_without_host_and_version() {
            assert_eq!(
                git::message::connect(Service::UploadPack, Protocol::V1, b"hello/world", None, &[]),
                "git-upload-pack hello/world\0"
            )
        }
        #[test]
        fn version_2_without_host_and_version() {
            assert_eq!(
                git::message::connect(Service::UploadPack, Protocol::V2, b"hello\\world", None, &[]),
                "git-upload-pack hello\\world\0\0version=2\0"
            )
        }
        #[test]
        fn version_2_without_host_and_version_and_exta_parameters() {
            assert_eq!(
                git::message::connect(
                    Service::UploadPack,
                    Protocol::V2,
                    b"/path/project.git",
                    None,
                    &[("key", Some("value")), ("value-only", None)]
                ),
                "git-upload-pack /path/project.git\0\0version=2\0key=value\0value-only\0"
            )
        }
        #[test]
        fn with_host_without_port() {
            assert_eq!(
                git::message::connect(
                    Service::UploadPack,
                    Protocol::V1,
                    b"hello\\world",
                    Some(&("host".into(), None)),
                    &[]
                ),
                "git-upload-pack hello\\world\0host=host\0"
            )
        }
        #[test]
        fn with_host_without_port_and_extra_parameters() {
            assert_eq!(
                git::message::connect(
                    Service::UploadPack,
                    Protocol::V1,
                    b"hello\\world",
                    Some(&("host".into(), None)),
                    &[("key", Some("value")), ("value-only", None)]
                ),
                "git-upload-pack hello\\world\0host=host\0\0key=value\0value-only\0"
            )
        }
        #[test]
        fn with_host_with_port() {
            assert_eq!(
                git::message::connect(
                    Service::UploadPack,
                    Protocol::V1,
                    b"hello\\world",
                    Some(&("host".into(), Some(404))),
                    &[]
                ),
                "git-upload-pack hello\\world\0host=host:404\0"
            )
        }
    }
}

#[cfg(feature = "async-client")]
mod async_io;

#[cfg(feature = "blocking-client")]
mod blocking_io;
#[cfg(feature = "blocking-client")]
pub use blocking_io::connect;
