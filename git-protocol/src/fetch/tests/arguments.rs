use crate::fetch;
use bstr::ByteSlice;
use git_transport::Protocol;
use std::io;

fn new(protocol: Protocol, features: impl IntoIterator<Item = &'static str>) -> fetch::Arguments {
    fetch::Arguments::new(
        protocol,
        features
            .into_iter()
            .chain(if protocol == Protocol::V1 {
                Some("multi_ack_detailed")
            } else {
                None
            })
            .map(|n| (n, None))
            .collect(),
    )
    .expect("all required features")
}

fn transport(out: &mut Vec<u8>) -> git_transport::client::git::Connection<io::Cursor<Vec<u8>>, &mut Vec<u8>> {
    git_transport::client::git::Connection::new(
        io::Cursor::new(Vec::new()),
        out,
        Protocol::V1, // does not matter
        b"does/not/matter".as_bstr().to_owned(),
        None::<(&str, _)>,
        git_transport::client::git::ConnectMode::Process, // avoid header to be sent
    )
}

fn id(hex: &str) -> git_object::owned::Id {
    git_object::owned::Id::from_40_bytes_in_hex(hex.as_bytes()).expect("expect valid hex id")
}

mod v1 {
    use crate::fetch::tests::arguments::{id, new, transport};
    use bstr::ByteSlice;
    use git_transport::Protocol;

    #[test]
    fn haves_and_wants_for_clone() {
        let mut out = Vec::new();
        let mut t = transport(&mut out);
        let mut arguments = new(Protocol::V1, ["feature-a", "feature-b"].iter().cloned());

        arguments.want(id("7b333369de1221f9bfbbe03a3a13e9a09bc1c907").to_borrowed());
        arguments.want(id("ff333369de1221f9bfbbe03a3a13e9a09bc1ffff").to_borrowed());
        arguments.send(&mut t, true).expect("sending to buffer to work");
        assert_eq!(
            out.as_bstr(),
            b"0059want 7b333369de1221f9bfbbe03a3a13e9a09bc1c907\0feature-a feature-b multi_ack_detailed
0032want ff333369de1221f9bfbbe03a3a13e9a09bc1ffff
00000009done
"
            .as_bstr()
        );
    }

    #[test]
    fn haves_and_wants_for_fetch() {
        let mut out = Vec::new();
        let mut t = transport(&mut out);
        let mut arguments = new(Protocol::V1, ["feature-a"].iter().cloned());

        arguments.want(id("7b333369de1221f9bfbbe03a3a13e9a09bc1c907").to_borrowed());
        arguments.have(id("0000000000000000000000000000000000000000").to_borrowed());
        arguments.send(&mut t, false).expect("sending to buffer to work");

        arguments.have(id("1111111111111111111111111111111111111111").to_borrowed());
        arguments.send(&mut t, true).expect("sending to buffer to work");
        assert_eq!(
            out.as_bstr(),
            b"004fwant 7b333369de1221f9bfbbe03a3a13e9a09bc1c907\0feature-a multi_ack_detailed
00000032have 0000000000000000000000000000000000000000
000000000032have 1111111111111111111111111111111111111111
0009done
"
            .as_bstr()
        );
    }
}

mod v2 {
    use crate::fetch::tests::arguments::{id, new, transport};
    use bstr::ByteSlice;
    use git_transport::Protocol;

    #[test]
    fn haves_and_wants_for_clone() {
        let mut out = Vec::new();
        let mut t = transport(&mut out);
        let mut arguments = new(Protocol::V2, ["feature-a", "feature-b"].iter().cloned());

        arguments.want(id("7b333369de1221f9bfbbe03a3a13e9a09bc1c907").to_borrowed());
        arguments.want(id("ff333369de1221f9bfbbe03a3a13e9a09bc1ffff").to_borrowed());
        arguments.send(&mut t, true).expect("sending to buffer to work");
        assert_eq!(
            out.as_bstr(),
            b"0012command=fetch
000efeature-a
000efeature-b
0001000ethin-pack
0010include-tag
000eofs-delta
0032want 7b333369de1221f9bfbbe03a3a13e9a09bc1c907
0032want ff333369de1221f9bfbbe03a3a13e9a09bc1ffff
0009done
0000"
                .as_bstr()
        );
    }

    #[test]
    fn haves_and_wants_for_fetch() {
        let mut out = Vec::new();
        let mut t = transport(&mut out);
        let mut arguments = new(Protocol::V2, ["feature-a"].iter().cloned());

        arguments.want(id("7b333369de1221f9bfbbe03a3a13e9a09bc1c907").to_borrowed());
        arguments.have(id("0000000000000000000000000000000000000000").to_borrowed());
        arguments.send(&mut t, false).expect("sending to buffer to work");

        arguments.have(id("1111111111111111111111111111111111111111").to_borrowed());
        arguments.send(&mut t, true).expect("sending to buffer to work");
        assert_eq!(
            out.as_bstr(),
            b"0012command=fetch
000efeature-a
0001000ethin-pack
0010include-tag
000eofs-delta
0032want 7b333369de1221f9bfbbe03a3a13e9a09bc1c907
0032have 0000000000000000000000000000000000000000
00000012command=fetch
000efeature-a
0001000ethin-pack
0010include-tag
000eofs-delta
0032have 1111111111111111111111111111111111111111
0009done
0000"
                .as_bstr()
        );
    }
}
