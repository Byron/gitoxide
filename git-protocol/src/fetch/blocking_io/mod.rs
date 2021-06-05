///
pub mod delegate;
///
pub mod response;

mod error {
    use std::io;

    use quick_error::quick_error;

    use git_transport::client;

    use crate::fetch::refs;
    use crate::{credentials, fetch::blocking_io::response};

    quick_error! {
        /// The error used in [`fetch()`].
        #[derive(Debug)]
        #[allow(missing_docs)]
        pub enum Error {
            PackIo(err: io::Error) {
                display("Could not read streaming pack file")
                from()
                source(err)
            }
            Credentials(err: credentials::Error) {
                display("Failed to obtain, approve or reject credentials")
                from()
                source(err)
            }
            Transport(err: client::Error) {
                display("An error occurred on the transport layer while fetching data")
                from()
                source(err)
            }
            SymrefWithoutValue {
                display("A symref 'capability' is expected to have a value")
            }
            Ref(err: refs::Error) {
                display("A reference could not be parsed or invariants were not met")
                from()
                source(err)
            }
            Response(err: response::Error) {
                display("The server response could not be parsed")
                from()
                source(err)
            }
        }
    }
}
pub use error::Error;

mod fetch {
    use std::io;

    use git_features::{progress, progress::Progress};
    use git_transport::{
        client,
        client::{SetServiceResponse, TransportV2Ext},
        Service,
    };

    use crate::fetch::refs;
    use crate::fetch::refs::InternalRef;
    use crate::{
        credentials,
        fetch::{ Action, Arguments, Command, Delegate, Error, Ref, Response},
    };

    /// Perform a 'fetch' operation with the server using `transport`, with `delegate` handling all server interactions.
    ///
    /// * `authenticate(operation_to_perform)` is used to receive credentials for the connection and potentially store it
    ///   if the server indicates 'permission denied'. Note that not all transport support authentication or authorization.
    /// * `progress` is used to emit progress messages.
    ///
    /// _Note_ that depending on the `delegate`, the actual action performed can be `ls-refs`, `clone` or `fetch`.
    pub fn fetch<F>(
        mut transport: impl client::Transport,
        delegate: &mut impl Delegate,
        mut authenticate: F,
        mut progress: impl Progress,
    ) -> Result<(), Error>
    where
        F: FnMut(credentials::Action<'_>) -> credentials::Result,
    {
        let (protocol_version, mut parsed_refs, capabilities, call_ls_refs) = {
            progress.init(None, progress::steps());
            progress.set_name("handshake");
            progress.step();
            let result = transport.handshake(Service::UploadPack);
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
                    match transport.handshake(Service::UploadPack) {
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

            let mut parsed_refs = Vec::<InternalRef>::new();
            refs::from_capabilities(&mut parsed_refs, capabilities.iter())?;

            let call_ls_refs = match refs {
                Some(mut refs) => {
                    assert_eq!(
                        actual_protocol,
                        git_transport::Protocol::V1,
                        "Only V1 auto-responds with refs"
                    );
                    refs::from_v1_refs_received_as_part_of_handshake(&mut parsed_refs, &mut refs)?;
                    false
                }
                None => true,
            };
            (
                actual_protocol,
                parsed_refs.into_iter().map(Into::into).collect::<Vec<Ref>>(),
                capabilities,
                call_ls_refs,
            )
        }; // this scope is needed, see https://github.com/rust-lang/rust/issues/76149

        if transport.desired_protocol_version() != protocol_version {
            progress.info(format!(
                "server did not support protocol {} and downgraded to {}",
                transport.desired_protocol_version() as usize,
                protocol_version as usize,
            ));
        }

        if call_ls_refs {
            assert_eq!(
                protocol_version,
                git_transport::Protocol::V2,
                "Only V2 needs a separate request to get specific refs"
            );

            let ls_refs = Command::LsRefs;
            let mut ls_features = ls_refs.default_features(protocol_version, &capabilities);
            let mut ls_args = ls_refs.initial_arguments(&ls_features);
            delegate.prepare_ls_refs(&capabilities, &mut ls_args, &mut ls_features);
            ls_refs.validate_argument_prefixes_or_panic(protocol_version, &capabilities, &ls_args, &ls_features);

            progress.step();
            progress.set_name("list refs");
            let mut remote_refs = transport.invoke(
                ls_refs.as_str(),
                ls_features.into_iter(),
                if ls_args.is_empty() {
                    None
                } else {
                    Some(ls_args.into_iter())
                },
            )?;
            refs::from_v2_refs(&mut parsed_refs, &mut remote_refs)?;
        }

        let fetch = Command::Fetch;
        let mut fetch_features = fetch.default_features(protocol_version, &capabilities);
        let next = delegate.prepare_fetch(protocol_version, &capabilities, &mut fetch_features, &parsed_refs);
        fetch.validate_argument_prefixes_or_panic(protocol_version, &capabilities, &[], &fetch_features);

        if next == Action::Close {
            transport.close()?;
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
            let mut reader = arguments.send(&mut transport, action == Action::Close)?;
            if sideband_all {
                setup_remote_progress(&mut progress, &mut reader);
            }
            let response = Response::from_line_reader(protocol_version, &mut reader)?;
            previous_response = if response.has_pack() {
                progress.step();
                progress.set_name("receiving pack");
                if !sideband_all {
                    setup_remote_progress(&mut progress, &mut reader);
                }
                delegate.receive_pack(reader, progress, &parsed_refs, &response)?;
                break 'negotiation;
            } else {
                match action {
                    Action::Close => break 'negotiation,
                    Action::Continue => Some(response),
                }
            }
        }
        Ok(())
    }

    fn setup_remote_progress(
        progress: &mut impl Progress,
        reader: &mut Box<dyn git_transport::client::ExtendedBufRead + '_>,
    ) {
        reader.set_progress_handler(Some(Box::new({
            let mut remote_progress = progress.add_child("remote");
            move |is_err: bool, data: &[u8]| {
                crate::RemoteProgress::translate_to_progress(is_err, data, &mut remote_progress)
            }
        }) as git_transport::client::HandleProgress));
    }
}
pub use fetch::fetch;
