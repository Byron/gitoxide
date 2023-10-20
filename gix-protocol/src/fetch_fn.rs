use std::borrow::Cow;

use gix_features::progress::NestedProgress;
use gix_transport::client;
use maybe_async::maybe_async;

use crate::{
    credentials,
    fetch::{Action, Arguments, Delegate, Error, Response},
    indicate_end_of_interaction, Command,
};

/// A way to indicate how to treat the connection underlying the transport, potentially allowing to reuse it.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FetchConnection {
    /// Use this variant if server should be informed that the operation is completed and no further commands will be issued
    /// at the end of the fetch operation or after deciding that no fetch operation should happen after references were listed.
    ///
    /// When indicating the end-of-fetch, this flag is only relevant in protocol V2.
    /// Generally it only applies when using persistent transports.
    ///
    /// In most explicit client side failure modes the end-of-operation' notification will be sent to the server automatically.
    #[default]
    TerminateOnSuccessfulCompletion,

    /// Indicate that persistent transport connections can be reused by _not_ sending an 'end-of-operation' notification to the server.
    /// This is useful if multiple `fetch(…)` calls are used in succession.
    ///
    /// Note that this has no effect in case of non-persistent connections, like the ones over HTTP.
    ///
    /// As an optimization, callers can use `AllowReuse` here as the server will also know the client is done
    /// if the connection is closed.
    AllowReuse,
}

/// Perform a 'fetch' operation with the server using `transport`, with `delegate` handling all server interactions.
/// **Note** that `delegate` has blocking operations and thus this entire call should be on an executor which can handle
/// that. This could be the current thread blocking, or another thread.
///
/// * `authenticate(operation_to_perform)` is used to receive credentials for the connection and potentially store it
///   if the server indicates 'permission denied'. Note that not all transport support authentication or authorization.
/// * `progress` is used to emit progress messages.
/// * `name` is the name of the git client to present as `agent`, like `"my-app (v2.0)"`".
/// * If `trace` is `true`, all packetlines received or sent will be passed to the facilities of the `gix-trace` crate.
///
/// _Note_ that depending on the `delegate`, the actual action performed can be `ls-refs`, `clone` or `fetch`.
///
/// # WARNING - Do not use!
///
/// As it will hang when having multiple negotiation rounds.
#[allow(clippy::result_large_err)]
#[maybe_async]
// TODO: remove this without losing test coverage - we have the same but better in `gix` and it's
//       not really worth it to maintain the delegates here.
pub async fn fetch<F, D, T, P>(
    mut transport: T,
    mut delegate: D,
    authenticate: F,
    mut progress: P,
    fetch_mode: FetchConnection,
    agent: impl Into<String>,
    trace: bool,
) -> Result<(), Error>
where
    F: FnMut(credentials::helper::Action) -> credentials::protocol::Result,
    D: Delegate,
    T: client::Transport,
    P: NestedProgress + 'static,
    P::SubProgress: 'static,
{
    let crate::handshake::Outcome {
        server_protocol_version: protocol_version,
        refs,
        capabilities,
    } = crate::fetch::handshake(
        &mut transport,
        authenticate,
        delegate.handshake_extra_parameters(),
        &mut progress,
    )
    .await?;

    let agent = crate::agent(agent);
    let refs = match refs {
        Some(refs) => refs,
        None => {
            crate::ls_refs(
                &mut transport,
                &capabilities,
                |a, b, c| {
                    let res = delegate.prepare_ls_refs(a, b, c);
                    c.push(("agent", Some(Cow::Owned(agent.clone()))));
                    res
                },
                &mut progress,
                trace,
            )
            .await?
        }
    };

    let fetch = Command::Fetch;
    let mut fetch_features = fetch.default_features(protocol_version, &capabilities);
    match delegate.prepare_fetch(protocol_version, &capabilities, &mut fetch_features, &refs) {
        Ok(Action::Cancel) => {
            return if matches!(protocol_version, gix_transport::Protocol::V1)
                || matches!(fetch_mode, FetchConnection::TerminateOnSuccessfulCompletion)
            {
                indicate_end_of_interaction(transport, trace).await.map_err(Into::into)
            } else {
                Ok(())
            };
        }
        Ok(Action::Continue) => {
            fetch.validate_argument_prefixes_or_panic(protocol_version, &capabilities, &[], &fetch_features);
        }
        Err(err) => {
            indicate_end_of_interaction(transport, trace).await?;
            return Err(err.into());
        }
    }

    Response::check_required_features(protocol_version, &fetch_features)?;
    let sideband_all = fetch_features.iter().any(|(n, _)| *n == "sideband-all");
    fetch_features.push(("agent", Some(Cow::Owned(agent))));
    let mut arguments = Arguments::new(protocol_version, fetch_features, trace);
    let mut previous_response = None::<Response>;
    let mut round = 1;
    'negotiation: loop {
        progress.step();
        progress.set_name(format!("negotiate (round {round})"));
        round += 1;
        let action = delegate.negotiate(&refs, &mut arguments, previous_response.as_ref())?;
        let mut reader = arguments.send(&mut transport, action == Action::Cancel).await?;
        if sideband_all {
            setup_remote_progress(&mut progress, &mut reader);
        }
        let response = Response::from_line_reader(
            protocol_version,
            &mut reader,
            true,  /* hack, telling us we don't want this delegate approach anymore */
            false, /* just as much of a hack which causes us to expect a pack immediately */
        )
        .await?;
        previous_response = if response.has_pack() {
            progress.step();
            progress.set_name("receiving pack".into());
            if !sideband_all {
                setup_remote_progress(&mut progress, &mut reader);
            }
            delegate.receive_pack(reader, progress, &refs, &response).await?;
            break 'negotiation;
        } else {
            match action {
                Action::Cancel => break 'negotiation,
                Action::Continue => Some(response),
            }
        }
    }
    if matches!(protocol_version, gix_transport::Protocol::V2)
        && matches!(fetch_mode, FetchConnection::TerminateOnSuccessfulCompletion)
    {
        indicate_end_of_interaction(transport, trace).await?;
    }
    Ok(())
}

fn setup_remote_progress<P>(progress: &mut P, reader: &mut Box<dyn gix_transport::client::ExtendedBufRead + Unpin + '_>)
where
    P: NestedProgress,
    P::SubProgress: 'static,
{
    reader.set_progress_handler(Some(Box::new({
        let mut remote_progress = progress.add_child("remote");
        move |is_err: bool, data: &[u8]| {
            crate::RemoteProgress::translate_to_progress(is_err, data, &mut remote_progress);
            gix_transport::packetline::read::ProgressAction::Continue
        }
    }) as gix_transport::client::HandleProgress));
}
