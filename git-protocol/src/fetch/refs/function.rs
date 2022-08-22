use super::Error;
use crate::fetch::refs::from_v2_refs;
use crate::fetch::{indicate_end_of_interaction, Command, LsRefsAction, Ref};
use bstr::BString;
use git_features::progress::Progress;
use git_transport::client::{Capabilities, Transport, TransportV2Ext};
use git_transport::Protocol;
use maybe_async::maybe_async;

/// Invoke an ls-refs command on `transport`  (assuming `protocol_version` 2 or panic), which requires a prior handshake that yielded
/// server `capabilities`. `prepare_ls_refs(arguments, features)` can be used to alter the _ls-refs_. `progress` is used to provide feedback.
#[maybe_async]
pub async fn refs(
    mut transport: impl Transport,
    protocol_version: Protocol,
    capabilities: &Capabilities,
    mut prepare_ls_refs: impl FnMut(
        &Capabilities,
        &mut Vec<BString>,
        &mut Vec<(&str, Option<&str>)>,
    ) -> std::io::Result<LsRefsAction>,
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
    let refs = match prepare_ls_refs(capabilities, &mut ls_args, &mut ls_features) {
        Ok(LsRefsAction::Skip) => Vec::new(),
        Ok(LsRefsAction::Continue) => {
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
