use crate::{
    credentials,
    fetch::{refs, Action, Arguments, Command, Delegate, Error, LsRefsAction, Response},
};
use git_features::{progress, progress::Progress};
use git_transport::{
    client,
    client::{SetServiceResponse, TransportV2Ext},
    Service,
};
use maybe_async::maybe_async;
use std::io;

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
    mut authenticate: F,
    mut progress: impl Progress,
) -> Result<(), Error>
where
    F: FnMut(credentials::Action<'_>) -> credentials::Result,
    D: Delegate,
    T: client::Transport,
{
    let (protocol_version, parsed_refs, capabilities) = {
        progress.init(None, progress::steps());
        progress.set_name("handshake");
        progress.step();

        let extra_parameters = delegate.handshake_extra_parameters();
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
            Err(client::Error::Io { ref err }) if err.kind() == io::ErrorKind::PermissionDenied => {
                drop(result); // needed to workaround this: https://github.com/rust-lang/rust/issues/76149
                let url = transport.to_url();
                progress.set_name("authentication");
                let credentials::Outcome { identity, next } =
                    authenticate(credentials::Action::Fill(&url))?.expect("FILL provides an identity");
                transport.set_identity(identity)?;
                progress.step();
                progress.set_name("handshake (authenticated)");
                match transport.handshake(Service::UploadPack, &extra_parameters).await {
                    Ok(v) => {
                        authenticate(next.approve())?;
                        Ok(v)
                    }
                    // Still no permission? Reject the credentials.
                    Err(client::Error::Io { err }) if err.kind() == io::ErrorKind::PermissionDenied => {
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
                    refs::from_v1_refs_received_as_part_of_handshake_and_capabilities(&mut refs, capabilities.iter())
                        .await?,
                )
            }
            None => None,
        };
        (actual_protocol, parsed_refs, capabilities)
    }; // this scope is needed, see https://github.com/rust-lang/rust/issues/76149

    let parsed_refs = match parsed_refs {
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
            let next = delegate.prepare_ls_refs(&capabilities, &mut ls_args, &mut ls_features);
            // User has requested to skip ls-refs, perhaps because they're using want-ref
            if next == LsRefsAction::Skip {
                vec![]
            } else {
                ls_refs.validate_argument_prefixes_or_panic(protocol_version, &capabilities, &ls_args, &ls_features);

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
        }
    };

    let fetch = Command::Fetch;
    let mut fetch_features = fetch.default_features(protocol_version, &capabilities);
    let next = delegate.prepare_fetch(protocol_version, &capabilities, &mut fetch_features, &parsed_refs);
    fetch.validate_argument_prefixes_or_panic(protocol_version, &capabilities, &[], &fetch_features);

    if next == Action::Cancel {
        // An empty request marks the (early) end of the interaction.
        transport
            .request(client::WriteMode::Binary, client::MessageKind::Flush)?
            .into_read()
            .await?;
        return Ok(());
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
        let action = delegate.negotiate(&parsed_refs, &mut arguments, previous_response.as_ref());
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
            delegate.receive_pack(reader, progress, &parsed_refs, &response).await?;
            break 'negotiation;
        } else {
            match action {
                Action::Cancel => break 'negotiation,
                Action::Continue => Some(response),
            }
        }
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
