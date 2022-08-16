use crate::fetch::Ref;
use git_transport::client::Capabilities;

/// The result of the [`handshake()`][super::handshake()] function.
pub struct Outcome {
    /// The protocol version the server responded with. It might have downgraded the desired version.
    pub server_protocol_version: git_transport::Protocol,
    /// The references reported as part of the Protocol::V1 handshake, or `None` otherwise as V2 requires a separate request.
    pub refs: Option<Vec<Ref>>,
    /// The server capabilities.
    pub capabilities: Capabilities,
}

mod error {
    use crate::credentials;
    use crate::fetch::refs;
    use git_transport::client;

    /// The error returned by [`handshake()`][super::handshake()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        Credentials(#[from] credentials::helper::Error),
        #[error(transparent)]
        Transport(#[from] client::Error),
        #[error("The transport didn't accept the advertised server version {actual_version:?} and closed the connection client side")]
        TransportProtocolPolicyViolation { actual_version: git_transport::Protocol },
        #[error(transparent)]
        ParseRefs(#[from] refs::parse::Error),
    }
}
pub use error::Error;

pub(crate) mod function {
    use super::{Error, Outcome};
    use crate::credentials;
    use crate::fetch::refs;
    use git_features::progress;
    use git_features::progress::Progress;
    use git_transport::client::SetServiceResponse;
    use git_transport::{client, Service};
    use maybe_async::maybe_async;

    /// Perform a handshake with the server on the other side of `transport`, with `authenticate` being used if authentication
    /// turns out to be required. `extra_parameters` are the parameters `(name, optional value)` to add to the handshake,
    /// each time it is performed in case authentication is required.
    /// `progress` is used to inform about what's currently happening.
    #[maybe_async]
    pub async fn handshake<AuthFn, T>(
        mut transport: T,
        mut authenticate: AuthFn,
        extra_parameters: Vec<(String, Option<String>)>,
        progress: &mut impl Progress,
    ) -> Result<Outcome, Error>
    where
        AuthFn: FnMut(credentials::helper::Action<'_>) -> credentials::helper::Result,
        T: client::Transport,
    {
        let (server_protocol_version, refs, capabilities) = {
            progress.init(None, progress::steps());
            progress.set_name("handshake");
            progress.step();

            let extra_parameters: Vec<_> = extra_parameters
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_ref().map(|s| s.as_str())))
                .collect();
            let supported_versions: Vec<_> = transport.supported_protocol_versions().into();

            let result = transport.handshake(Service::UploadPack, &extra_parameters).await;
            let SetServiceResponse {
                actual_protocol,
                capabilities,
                refs,
            } = match result {
                Ok(v) => Ok(v),
                Err(client::Error::Io { ref err }) if err.kind() == std::io::ErrorKind::PermissionDenied => {
                    drop(result); // needed to workaround this: https://github.com/rust-lang/rust/issues/76149
                    let url = transport.to_url();
                    progress.set_name("authentication");
                    let credentials::helper::Outcome { identity, next } =
                        authenticate(credentials::helper::Action::Fill(url.as_str().into()))?
                            .expect("FILL provides an identity");
                    transport.set_identity(identity)?;
                    progress.step();
                    progress.set_name("handshake (authenticated)");
                    match transport.handshake(Service::UploadPack, &extra_parameters).await {
                        Ok(v) => {
                            authenticate(next.approve())?;
                            Ok(v)
                        }
                        // Still no permission? Reject the credentials.
                        Err(client::Error::Io { err }) if err.kind() == std::io::ErrorKind::PermissionDenied => {
                            authenticate(next.reject())?;
                            Err(client::Error::Io { err })
                        }
                        // Otherwise, do nothing, as we don't know if it actually got to try the credentials.
                        // If they were previously stored, they remain. In the worst case, the user has to enter them again
                        // next time they try.
                        Err(err) => Err(err),
                    }
                }
                Err(err) => Err(err),
            }?;

            if !supported_versions.is_empty() && !supported_versions.contains(&actual_protocol) {
                return Err(Error::TransportProtocolPolicyViolation {
                    actual_version: actual_protocol,
                });
            }

            let parsed_refs = match refs {
                Some(mut refs) => {
                    assert_eq!(
                        actual_protocol,
                        git_transport::Protocol::V1,
                        "Only V1 auto-responds with refs"
                    );
                    Some(
                        refs::from_v1_refs_received_as_part_of_handshake_and_capabilities(
                            &mut refs,
                            capabilities.iter(),
                        )
                        .await?,
                    )
                }
                None => None,
            };
            (actual_protocol, parsed_refs, capabilities)
        }; // this scope is needed, see https://github.com/rust-lang/rust/issues/76149

        Ok(Outcome {
            server_protocol_version,
            refs,
            capabilities,
        })
    }
}
