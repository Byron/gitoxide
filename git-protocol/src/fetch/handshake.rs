use crate::credentials;
use git_features::progress::Progress;
use git_transport::{client, Service};

use crate::handshake::{Error, Outcome};
use maybe_async::maybe_async;

/// Perform a handshake with the server on the other side of `transport`, with `authenticate` being used if authentication
/// turns out to be required. `extra_parameters` are the parameters `(name, optional value)` to add to the handshake,
/// each time it is performed in case authentication is required.
/// `progress` is used to inform about what's currently happening.
#[allow(clippy::result_large_err)]
#[maybe_async]
pub async fn upload_pack<AuthFn, T>(
    transport: T,
    authenticate: AuthFn,
    extra_parameters: Vec<(String, Option<String>)>,
    progress: &mut impl Progress,
) -> Result<Outcome, Error>
where
    AuthFn: FnMut(credentials::helper::Action) -> credentials::protocol::Result,
    T: client::Transport,
{
    crate::handshake(transport, Service::UploadPack, authenticate, extra_parameters, progress).await
}
