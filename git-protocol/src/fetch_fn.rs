use git_features::progress::Progress;
use git_transport::{client, client::TransportV2Ext};
use maybe_async::maybe_async;

use crate::fetch::handshake;
use crate::{
    credentials,
    fetch::{refs, Action, Arguments, Command, Delegate, Error, LsRefsAction, Response},
};

/// A way to indicate how to treat the connection underlying the transport, potentially allowing to reuse it.
pub enum FetchConnection {
    /// Use this variant if server should be informed that the operation is completed and no further commands will be issued
    /// at the end of the fetch operation or after deciding that no fetch operation should happen after references were listed.
    ///
    /// When indicating the end-of-fetch, this flag is only relevant in protocol V2.
    /// Generally it only applies when using persistent transports.
    ///
    /// In most explicit client side failure modes the end-of-operation' notification will be sent to the server automatically.
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

impl Default for FetchConnection {
    fn default() -> Self {
        FetchConnection::TerminateOnSuccessfulCompletion
    }
}

/// Perform a 'fetch' operation with the server using `transport`, with `delegate` handling all server interactions.
/// **Note** that `delegate` has blocking operations and thus this entire call should be on an executor which can handle
/// that. This could be the current thread blocking, or another thread.
///
/// * `authenticate(operation_to_perform)` is used to receive credentials for the connection and potentially store it
///   if the server indicates 'permission denied'. Note that not all transport support authentication or authorization.
/// * `progress` is used to emit progress messages.
///
/// _Note_ that depending on the `delegate`, the actual action performed can be `ls-refs`, `clone` or `fetch`.
#[maybe_async]
pub async fn fetch<F, D, T>(
    mut transport: T,
    mut delegate: D,
    authenticate: F,
    mut progress: impl Progress,
    fetch_mode: FetchConnection,
) -> Result<(), Error>
where
    F: FnMut(credentials::helper::Action<'_>) -> credentials::helper::Result,
    D: Delegate,
    T: client::Transport,
{
    let handshake::Outcome {
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

    let refs = match refs {
        Some(refs) => refs,
        None => {
            assert_eq!(
                protocol_version,
                git_transport::Protocol::V2,
                "Only V2 needs a separate request to get specific refs"
            );

            let ls_refs = Command::LsRefs;
            let mut ls_features = ls_refs.default_features(protocol_version, &capabilities);
            let mut ls_args = ls_refs.initial_arguments(&ls_features);
            match delegate.prepare_ls_refs(&capabilities, &mut ls_args, &mut ls_features) {
                Ok(LsRefsAction::Skip) => Vec::new(),
                Ok(LsRefsAction::Continue) => {
                    ls_refs.validate_argument_prefixes_or_panic(
                        protocol_version,
                        &capabilities,
                        &ls_args,
                        &ls_features,
                    );

                    progress.step();
                    progress.set_name("list refs");
                    let mut remote_refs = transport
                        .invoke(
                            ls_refs.as_str(),
                            ls_features.into_iter(),
                            if ls_args.is_empty() {
                                None
                            } else {
                                Some(ls_args.into_iter())
                            },
                        )
                        .await?;
                    refs::from_v2_refs(&mut remote_refs).await?
                }
                Err(err) => {
                    indicate_end_of_interaction(transport).await?;
                    return Err(err.into());
                }
            }
        }
    };

    let fetch = Command::Fetch;
    let mut fetch_features = fetch.default_features(protocol_version, &capabilities);
    match delegate.prepare_fetch(protocol_version, &capabilities, &mut fetch_features, &refs) {
        Ok(Action::Cancel) => {
            return if matches!(protocol_version, git_transport::Protocol::V1)
                || matches!(fetch_mode, FetchConnection::TerminateOnSuccessfulCompletion)
            {
                indicate_end_of_interaction(transport).await
            } else {
                Ok(())
            };
        }
        Ok(Action::Continue) => {
            fetch.validate_argument_prefixes_or_panic(protocol_version, &capabilities, &[], &fetch_features);
        }
        Err(err) => {
            indicate_end_of_interaction(transport).await?;
            return Err(err.into());
        }
    }

    Response::check_required_features(protocol_version, &fetch_features)?;
    let sideband_all = fetch_features.iter().any(|(n, _)| *n == "sideband-all");
    let mut arguments = Arguments::new(protocol_version, fetch_features);
    let mut previous_response = None::<Response>;
    let mut round = 1;
    'negotiation: loop {
        progress.step();
        progress.set_name(format!("negotiate (round {})", round));
        round += 1;
        let action = delegate.negotiate(&refs, &mut arguments, previous_response.as_ref())?;
        let mut reader = arguments.send(&mut transport, action == Action::Cancel).await?;
        if sideband_all {
            setup_remote_progress(&mut progress, &mut reader);
        }
        let response = Response::from_line_reader(protocol_version, &mut reader).await?;
        previous_response = if response.has_pack() {
            progress.step();
            progress.set_name("receiving pack");
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
    if matches!(protocol_version, git_transport::Protocol::V2)
        && matches!(fetch_mode, FetchConnection::TerminateOnSuccessfulCompletion)
    {
        indicate_end_of_interaction(transport).await?;
    }
    Ok(())
}

#[maybe_async]
async fn indicate_end_of_interaction(mut transport: impl client::Transport) -> Result<(), Error> {
    // An empty request marks the (early) end of the interaction. Only relevant in stateful transports though.
    if transport.connection_persists_across_multiple_requests() {
        transport
            .request(client::WriteMode::Binary, client::MessageKind::Flush)?
            .into_read()
            .await?;
    }
    Ok(())
}

fn setup_remote_progress(
    progress: &mut impl Progress,
    reader: &mut Box<dyn git_transport::client::ExtendedBufRead + Unpin + '_>,
) {
    reader.set_progress_handler(Some(Box::new({
        let mut remote_progress = progress.add_child("remote");
        move |is_err: bool, data: &[u8]| {
            crate::RemoteProgress::translate_to_progress(is_err, data, &mut remote_progress)
        }
    }) as git_transport::client::HandleProgress));
}
