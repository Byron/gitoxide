mod error {
    use crate::handshake::refs::parse;

    /// The error returned by [ls_refs()][crate::ls_refs()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        Io(#[from] std::io::Error),
        #[error(transparent)]
        Transport(#[from] git_transport::client::Error),
        #[error(transparent)]
        Parse(#[from] parse::Error),
    }
}
pub use error::Error;

/// What to do after [`DelegateBlocking::prepare_ls_refs`].
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub enum Action {
    /// Continue by sending a 'ls-refs' command.
    Continue,
    /// Skip 'ls-refs' entirely.
    ///
    /// This is valid if the 'ref-in-want' capability is taken advantage of. The delegate must then send 'want-ref's in
    /// [`DelegateBlocking::negotiate`].
    Skip,
}

pub(crate) mod function {
    use bstr::BString;
    use git_features::progress::Progress;
    use git_transport::{
        client::{Capabilities, Transport, TransportV2Ext},
        Protocol,
    };
    use maybe_async::maybe_async;
    use std::borrow::Cow;

    use super::{Action, Error};
    use crate::handshake::{refs::from_v2_refs, Ref};
    use crate::indicate_end_of_interaction;
    use crate::Command;

    /// Invoke an ls-refs command on `transport`  (assuming `protocol_version` 2 or panic), which requires a prior handshake that yielded
    /// server `capabilities`. `prepare_ls_refs(capabilities, arguments, features)` can be used to alter the _ls-refs_. `progress` is used to provide feedback.
    /// Note that `prepare_ls_refs()` is expected to add the `(agent, Some(name))` to the list of `features`.
    #[maybe_async]
    pub async fn ls_refs(
        mut transport: impl Transport,
        protocol_version: Protocol,
        capabilities: &Capabilities,
        prepare_ls_refs: impl FnOnce(
            &Capabilities,
            &mut Vec<BString>,
            &mut Vec<(&str, Option<Cow<'static, str>>)>,
        ) -> std::io::Result<Action>,
        progress: &mut impl Progress,
    ) -> Result<Vec<Ref>, Error> {
        assert_eq!(
            protocol_version,
            Protocol::V2,
            "Only V2 needs a separate request to get specific refs"
        );

        let ls_refs = Command::LsRefs;
        let mut ls_features = ls_refs.default_features(protocol_version, capabilities);
        let mut ls_args = ls_refs.initial_arguments(&ls_features);
        if capabilities
            .capability("ls-refs")
            .and_then(|cap| cap.supports("unborn"))
            .unwrap_or_default()
        {
            ls_args.push("unborn".into());
        }
        let refs = match prepare_ls_refs(capabilities, &mut ls_args, &mut ls_features) {
            Ok(Action::Skip) => Vec::new(),
            Ok(Action::Continue) => {
                ls_refs.validate_argument_prefixes_or_panic(protocol_version, capabilities, &ls_args, &ls_features);

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
                from_v2_refs(&mut remote_refs).await?
            }
            Err(err) => {
                indicate_end_of_interaction(transport).await?;
                return Err(err.into());
            }
        };
        Ok(refs)
    }
}
