use std::{
    cell::RefCell,
    io::{BufRead, Read},
    ops::Deref,
    rc::Rc,
};

use bstr::ByteSlice;

use git_transport::{
    client::{git, Transport},
    Protocol, Service,
};

use crate::fixture_bytes;

#[test]
fn handshake_v2_and_request() -> crate::Result {
    use git_transport::client::TransportV2Ext;
    let mut out = Vec::new();
    let input = fixture_bytes("v2/clone.response");
    let mut c = git::Connection::new(
        input.as_slice(),
        &mut out,
        Protocol::V2,
        "/bar.git",
        Some(("example.org", None)),
        git::ConnectMode::Daemon,
    );
    assert!(
        c.is_stateful(),
        "tcp connections are stateful despite the protocol version"
    );
    let res = c.handshake(Service::UploadPack)?;
    assert_eq!(res.actual_protocol, Protocol::V2);
    assert!(
        res.refs.is_none(),
        "V2 needs a separate trip for getting refs (with additional capabilities)"
    );
    assert_eq!(
        res.capabilities
            .iter()
            .map(|c| (c.name().to_owned(), c.value().map(ToOwned::to_owned)))
            .collect::<Vec<_>>(),
        [
            ("agent", Some("git/2.28.0")),
            ("ls-refs", None),
            ("fetch", Some("shallow")),
            ("server-option", None),
            ("object-format", Some("sha1"))
        ]
        .iter()
        .map(|(k, v)| (k.as_bytes().into(), v.map(|v| v.as_bytes().into())))
        .collect::<Vec<_>>()
    );
    drop(res);

    let res = c.invoke(
        "ls-refs",
        [("agent", Some("git/2.28.0")), ("object-format", Some("sha1"))]
            .iter()
            .cloned(),
        Some(
            [
                "peel",
                "symrefs",
                "ref-prefix HEAD",
                "ref-prefix refs/heads/",
                "ref-prefix refs/tags",
            ]
            .iter()
            .map(|s| s.as_bytes().as_bstr().to_owned()),
        ),
    )?;

    assert_eq!(
        res.lines().collect::<Result<Vec<_>, _>>()?,
        vec![
            "808e50d724f604f69ab93c6da2919c014667bedb HEAD symref-target:refs/heads/master".to_string(),
            "808e50d724f604f69ab93c6da2919c014667bedb refs/heads/master".into()
        ]
    );

    let mut res = c.invoke(
        "fetch",
        [
            ("agent", Some("git/2.28.0")),
            ("something-without-value", None),
            ("object-format", Some("sha1")),
        ]
        .iter()
        .cloned(),
        Some(
            [
                "thin-pack",
                "ofs-delta",
                "want 808e50d724f604f69ab93c6da2919c014667bedb",
                "done",
            ]
            .iter()
            .map(|s| s.as_bytes().as_bstr().to_owned()),
        ),
    )?;

    let mut line = String::new();
    res.read_line(&mut line)?;
    assert_eq!(line, "packfile\n");

    let messages = Rc::new(RefCell::new(Vec::<String>::new()));
    res.set_progress_handler(Some(Box::new({
        let sb = messages.clone();
        move |is_err, data| {
            assert!(!is_err);
            sb.deref()
                .borrow_mut()
                .push(std::str::from_utf8(data).expect("valid utf8").to_owned())
        }
    })));

    let mut pack = Vec::new();
    res.read_to_end(&mut pack)?;
    assert_eq!(pack.len(), 876);

    drop(res);
    let messages = Rc::try_unwrap(messages).expect("no other handle").into_inner();
    assert_eq!(messages.len(), 4);
    c.close()?;

    assert_eq!(
        out.as_slice().as_bstr(),
        b"0039git-upload-pack /bar.git\0host=example.org\0\0version=2\00014command=ls-refs
0015agent=git/2.28.0
0017object-format=sha1
00010009peel
000csymrefs
0014ref-prefix HEAD
001bref-prefix refs/heads/
0019ref-prefix refs/tags
00000012command=fetch
0015agent=git/2.28.0
001csomething-without-value
0017object-format=sha1
0001000ethin-pack
000eofs-delta
0032want 808e50d724f604f69ab93c6da2919c014667bedb
0009done
00000000"
            .as_bstr(),
        "it sends the correct request, including the adjusted version"
    );
    Ok(())
}
