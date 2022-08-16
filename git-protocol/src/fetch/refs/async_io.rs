use futures_io::AsyncBufRead;
use futures_lite::AsyncBufReadExt;

use crate::fetch::{refs, refs::parse::Error, Ref};

/// Parse refs from the given input line by line. Protocol V2 is required for this to succeed.
pub async fn from_v2_refs(in_refs: &mut (dyn AsyncBufRead + Unpin)) -> Result<Vec<Ref>, Error> {
    let mut out_refs = Vec::new();
    let mut line = String::new();
    loop {
        line.clear();
        let bytes_read = in_refs.read_line(&mut line).await?;
        if bytes_read == 0 {
            break;
        }
        out_refs.push(refs::shared::parse_v2(&line)?);
    }
    Ok(out_refs)
}

/// Parse refs from the return stream of the handshake as well as the server capabilities, also received as part of the
/// handshake.
/// Together they form a complete set of refs.
///
/// # Note
///
/// Symbolic refs are shoe-horned into server capabilities whereas refs (without symbolic ones) are sent automatically as
/// part of the handshake. Both symbolic and peeled refs need to be combined to fit into the [`Ref`] type provided here.
pub async fn from_v1_refs_received_as_part_of_handshake_and_capabilities<'a>(
    in_refs: &mut (dyn AsyncBufRead + Unpin),
    capabilities: impl Iterator<Item = git_transport::client::capabilities::Capability<'a>>,
) -> Result<Vec<Ref>, refs::parse::Error> {
    let mut out_refs = refs::shared::from_capabilities(capabilities)?;
    let number_of_possible_symbolic_refs_for_lookup = out_refs.len();
    let mut line = String::new();
    loop {
        line.clear();
        let bytes_read = in_refs.read_line(&mut line).await?;
        if bytes_read == 0 {
            break;
        }
        refs::shared::parse_v1(number_of_possible_symbolic_refs_for_lookup, &mut out_refs, &line)?;
    }
    Ok(out_refs.into_iter().map(Into::into).collect())
}
