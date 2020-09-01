use super::{refs::extract_symrefs, Capabilities, Ref};
use git_transport::client;
use std::convert::TryInto;

#[test]
fn extract_symbolic_references_from_capabilities() -> Result<(), client::Error> {
    let (caps, _) = client::Capabilities::from_bytes(
        b"\0unrelated symref=HEAD:refs/heads/main symref=ANOTHER:refs/heads/foo agent=git/2.28.0",
    )?;
    let mut caps: Capabilities = caps.try_into().expect("this is a working example");
    let mut out = Vec::new();
    extract_symrefs(&mut out, std::mem::take(&mut caps.symrefs)).expect("a working example");

    assert_eq!(
        caps.available.into_iter().collect::<Vec<_>>(),
        vec![("agent".into(), Some("git/2.28.0".into())), ("unrelated".into(), None)]
    );
    assert_eq!(
        out,
        vec![
            Ref::SymbolicForLookup {
                path: "HEAD".into(),
                target: "refs/heads/main".into()
            },
            Ref::SymbolicForLookup {
                path: "ANOTHER".into(),
                target: "refs/heads/foo".into()
            }
        ]
    );
    Ok(())
}
