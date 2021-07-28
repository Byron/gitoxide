use bstr::ByteSlice;
use git_transport::client::Capabilities;

#[test]
fn from_bytes() -> crate::Result {
    let (caps, delim_pos) = Capabilities::from_bytes(&b"7814e8a05a59c0cf5fb186661d1551c75d1299b5 HEAD\0multi_ack thin-pack side-band side-band-64k ofs-delta shallow deepen-since deepen-not deepen-relative no-progress include-tag multi_ack_detailed symref=HEAD:refs/heads/master object-format=sha1 agent=git/2.28.0"[..])?;
    assert_eq!(delim_pos, 45);
    assert_eq!(
        caps.iter().map(|c| c.name().to_owned()).collect::<Vec<_>>(),
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
    let object_format = caps.capability("object-format").expect("cap exists");
    assert_eq!(
        object_format.supports("sha1").expect("there is a value"),
        true,
        "sha1 is supported"
    );
    assert_eq!(
        object_format.supports("sha2").expect("there is a value"),
        false,
        "sha2 is not supported"
    );
    assert_eq!(
        caps.iter()
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
