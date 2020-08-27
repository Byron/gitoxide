mod connect_message {
    use git_transport::{client::git, Protocol, Service};

    #[test]
    fn version_1_without_host_and_version() {
        assert_eq!(
            git::message::connect(Service::UploadPack, Protocol::V1, b"hello/world", None),
            "git-upload-pack hello/world\0"
        )
    }
    #[test]
    fn version_2_without_host_and_version() {
        assert_eq!(
            git::message::connect(Service::UploadPack, Protocol::V2, b"hello\\world", None),
            "git-upload-pack hello\\world\0\0version=2\0"
        )
    }
    #[test]
    fn with_host_without_port() {
        assert_eq!(
            git::message::connect(
                Service::UploadPack,
                Protocol::V1,
                b"hello\\world",
                Some(&("host".into(), None))
            ),
            "git-upload-pack hello\\world\0host=host\0"
        )
    }
    #[test]
    fn with_host_with_port() {
        assert_eq!(
            git::message::connect(
                Service::UploadPack,
                Protocol::V1,
                b"hello\\world",
                Some(&("host".into(), Some(404)))
            ),
            "git-upload-pack hello\\world\0host=host:404\0"
        )
    }
}

use crate::fixture_bytes;
use bstr::ByteSlice;
use git_transport::{
    client::{self, git, ExtendedBufRead, Transport, TransportV2Ext},
    Protocol, Service,
};
use std::{
    cell::RefCell,
    io::{BufRead, Read, Write},
    ops::Deref,
    rc::Rc,
};

#[test]
fn handshake_v1_and_request() -> crate::Result {
    let mut out = Vec::new();
    let server_response = fixture_bytes("v1/clone.response");
    let mut c = git::Connection::new(
        server_response.as_slice(),
        &mut out,
        Protocol::V1,
        "/foo.git",
        Some(("example.org", None)),
    );
    let mut res = c.handshake(Service::UploadPack)?;
    assert_eq!(res.actual_protocol, Protocol::V1);
    assert_eq!(
        res.capabilities
            .iter()
            .flat_map(|c| c.value().map(ToOwned::to_owned))
            .collect::<Vec<_>>(),
        vec![
            b"HEAD:refs/heads/master".as_bstr(),
            b"sha1".as_bstr(),
            b"git/2.28.0".as_bstr()
        ]
    );
    let refs = res
        .refs
        .as_mut()
        .expect("v1 protocol provides refs")
        .lines()
        .flat_map(Result::ok)
        .collect::<Vec<_>>();
    assert_eq!(
        refs,
        vec![
            "808e50d724f604f69ab93c6da2919c014667bedb HEAD",
            "808e50d724f604f69ab93c6da2919c014667bedb refs/heads/master"
        ]
    );
    drop(res);

    let writer = c.request(client::WriteMode::Binary, Vec::new())?;
    assert_eq!(writer.into_read().lines().next().expect("exactly one line")?, "NAK");

    let mut writer = c.request(
        client::WriteMode::OneLFTerminatedLinePerWriteCall,
        vec![client::MessageKind::Flush, client::MessageKind::Text(b"done")],
    )?;

    writer.write_all(b"hello")?;
    writer.write_all(b"world")?;

    let mut reader = writer.into_read();
    let messages = Rc::new(RefCell::new(Vec::<String>::new()));
    reader.set_progress_handler(Some(Box::new({
        let sb = messages.clone();
        move |is_err, data| {
            assert!(!is_err);
            sb.deref()
                .borrow_mut()
                .push(std::str::from_utf8(data).expect("valid utf8").to_owned())
        }
    })));
    let mut pack = Vec::new();
    reader.read_to_end(&mut pack)?;
    assert_eq!(pack.len(), 876, "we receive the whole pack…");

    drop(reader);
    let sidebands = Rc::try_unwrap(messages).expect("no other handle").into_inner();
    assert_eq!(sidebands.len(), 6, "…along with some status messages");
    assert_eq!(
        out.as_slice().as_bstr(),
        b"002egit-upload-pack /foo.git\0host=example.org\0000ahello\n\
        000aworld\n\
        00000009done\n"
            .as_bstr(),
        "it sends the correct request"
    );
    Ok(())
}

#[test]
fn handshake_v2_and_request() -> crate::Result {
    let mut out = Vec::new();
    let input = fixture_bytes("v2/clone.response");
    let mut c = git::Connection::new(
        input.as_slice(),
        &mut out,
        Protocol::V2,
        "/bar.git",
        Some(("example.org", None)),
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

    c.invoke(
        "ls-refs",
        [("agent", "git/2.28.0"), ("object-format", "sha1")].iter().cloned(),
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
        out.as_slice().as_bstr(),
        b"0039git-upload-pack /bar.git\0host=example.org\0\0version=2\0".as_bstr(),
        "it sends the correct request, including the adjusted version"
    );
    Ok(())
}
