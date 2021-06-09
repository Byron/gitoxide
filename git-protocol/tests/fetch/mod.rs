use bstr::ByteSlice;

use git_protocol::fetch::{self, Action, Arguments, Ref, Response};
use git_transport::client::Capabilities;

use crate::fixture_bytes;

#[cfg(feature = "blocking-client")]
type Cursor = std::io::Cursor<Vec<u8>>;
#[cfg(feature = "async-client")]
type Cursor = futures_lite::io::Cursor<Vec<u8>>;

#[derive(Default)]
pub struct CloneDelegate {
    pack_bytes: usize,
}

impl fetch::DelegateWithoutIO for CloneDelegate {
    fn negotiate(&mut self, refs: &[Ref], arguments: &mut Arguments, _previous_result: Option<&Response>) -> Action {
        for r in refs {
            arguments.want(r.unpack().1);
        }
        Action::Close
    }
}

#[derive(Default)]
pub struct LsRemoteDelegate {
    refs: Vec<fetch::Ref>,
}

impl fetch::DelegateWithoutIO for LsRemoteDelegate {
    fn prepare_fetch(
        &mut self,
        _version: git_transport::Protocol,
        _server: &Capabilities,
        _features: &mut Vec<(&str, Option<&str>)>,
        refs: &[fetch::Ref],
    ) -> fetch::Action {
        self.refs = refs.to_owned();
        fetch::Action::Close
    }

    fn negotiate(&mut self, _refs: &[Ref], _arguments: &mut Arguments, _previous_result: Option<&Response>) -> Action {
        unreachable!("this must not be called after closing the connection in `prepare_fetch(…)`")
    }
}

#[cfg(feature = "blocking-client")]
mod blocking_io {
    use crate::fetch::{CloneDelegate, LsRemoteDelegate};
    use git_features::progress::Progress;
    use git_protocol::{
        fetch,
        fetch::{Ref, Response},
    };
    use std::io;

    impl fetch::Delegate for CloneDelegate {
        fn receive_pack(
            &mut self,
            mut input: impl io::BufRead,
            _progress: impl Progress,
            _refs: &[Ref],
            _previous: &Response,
        ) -> io::Result<()> {
            self.pack_bytes = io::copy(&mut input, &mut io::sink())? as usize;
            Ok(())
        }
    }

    impl fetch::Delegate for LsRemoteDelegate {
        fn receive_pack(
            &mut self,
            _input: impl io::BufRead,
            _progress: impl Progress,
            _refs: &[Ref],
            _previous: &Response,
        ) -> io::Result<()> {
            unreachable!("Should not be called for ls-refs");
        }
    }
}

#[cfg(feature = "async-client")]
mod async_io {
    use crate::fetch::{CloneDelegate, LsRemoteDelegate};
    use async_trait::async_trait;
    use futures_io::AsyncBufRead;
    use git_features::progress::Progress;
    use git_protocol::{
        fetch,
        fetch::{Ref, Response},
    };
    use std::io;

    #[async_trait]
    impl fetch::Delegate for CloneDelegate {
        async fn receive_pack(
            &mut self,
            mut input: impl AsyncBufRead + Unpin + 'async_trait + Send,
            _progress: impl Progress,
            _refs: &[Ref],
            _previous: &Response,
        ) -> io::Result<()> {
            self.pack_bytes = futures_lite::io::copy(&mut input, &mut futures_lite::io::sink()).await? as usize;
            Ok(())
        }
    }

    #[async_trait]
    impl fetch::Delegate for LsRemoteDelegate {
        async fn receive_pack(
            &mut self,
            _input: impl AsyncBufRead + Unpin + 'async_trait + Send,
            _progress: impl Progress,
            _refs: &[Ref],
            _previous: &Response,
        ) -> io::Result<()> {
            unreachable!("Should not be called for ls-refs");
        }
    }
}

pub fn oid(hex_sha: &str) -> git_hash::ObjectId {
    git_hash::ObjectId::from_hex(hex_sha.as_bytes()).expect("valid input")
}

pub fn transport<'a>(
    out: &'a mut Vec<u8>,
    path: &str,
    version: git_transport::Protocol,
) -> git_transport::client::git::Connection<Cursor, &'a mut Vec<u8>> {
    let response = fixture_bytes(path);
    git_transport::client::git::Connection::new(
        Cursor::new(response),
        out,
        version,
        b"does/not/matter".as_bstr().to_owned(),
        None::<(&str, _)>,
        git_transport::client::git::ConnectMode::Process,
    )
}

pub mod response;
mod v1;
mod v2;
