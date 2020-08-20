use bstr::ByteSlice;
use git_transport::client::Capabilities;
use std::convert::TryFrom;

#[test]
fn from_bytes() -> crate::Result {
    let c = Capabilities::try_from(&b"7814e8a05a59c0cf5fb186661d1551c75d1299b5 HEAD\0multi_ack thin-pack side-band side-band-64k ofs-delta shallow deepen-since deepen-not deepen-relative no-progress include-tag multi_ack_detailed symref=HEAD:refs/heads/master object-format=sha1 agent=git/2.28.0"[..])?;
    assert_eq!(
        c.iter().map(|s| s.name().to_owned()).collect::<Vec<_>>(),
        vec![
            "multi_ack",
            "thin-pack",
            "side-band",
            "side-band-64k",
            "ofs-delta",
            "shallow",
            "deepen-since",
            "deepen-not",
            "deepen-relative",
            "no-progress",
            "include-tag",
            "multi_ack_detailed",
            "symref",
            "object-format",
            "agent"
        ]
        .into_iter()
        .map(|s| s.as_bytes().as_bstr())
        .collect::<Vec<_>>()
    );
    assert_eq!(
        c.iter()
            .filter_map(|c| c.value().map(|v| v.to_owned()))
            .collect::<Vec<_>>(),
        vec![
            b"HEAD:refs/heads/master".as_bstr(),
            b"sha1".as_bstr(),
            b"git/2.28.0".as_bstr()
        ]
    );
    Ok(())
}
