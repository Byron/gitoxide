use std::{borrow::Cow, io};

use bstr::{BString, ByteSlice};
use gix_protocol::{
    fetch::{self, Action, Arguments, Response},
    handshake, ls_refs,
};
use gix_transport::client::Capabilities;

use crate::fixture_bytes;

#[cfg(feature = "blocking-client")]
type Cursor = std::io::Cursor<Vec<u8>>;
#[cfg(feature = "async-client")]
type Cursor = futures_lite::io::Cursor<Vec<u8>>;

#[allow(clippy::result_large_err)]
fn helper_unused(_action: gix_credentials::helper::Action) -> gix_credentials::protocol::Result {
    panic!("Call to credentials helper is unexpected")
}

#[derive(Default)]
pub struct CloneDelegate {
    pack_bytes: usize,
    abort_with: Option<std::io::Error>,
}

impl fetch::DelegateBlocking for CloneDelegate {
    fn prepare_fetch(
        &mut self,
        _version: gix_transport::Protocol,
        _server: &Capabilities,
        _features: &mut Vec<(&str, Option<Cow<'_, str>>)>,
        _refs: &[handshake::Ref],
    ) -> io::Result<Action> {
        if _refs.is_empty() {
            return Ok(Action::Cancel);
        }
        match self.abort_with.take() {
            Some(err) => Err(err),
            None => Ok(Action::Continue),
        }
    }
    fn negotiate(
        &mut self,
        refs: &[handshake::Ref],
        arguments: &mut Arguments,
        _previous_response: Option<&Response>,
    ) -> io::Result<Action> {
        for r in refs {
            if let Some(id) = r.unpack().1 {
                arguments.want(id);
            }
        }
        Ok(Action::Cancel)
    }
}

/// A delegate which bypasses refs negotiation entirely via `ref-in-want`.
#[derive(Default)]
pub struct CloneRefInWantDelegate {
    /// Statically-known refs we want.
    want_refs: Vec<BString>,

    /// Number of bytes received of the final packfile.
    pack_bytes: usize,

    /// Refs advertised by `ls-refs` -- should always be empty, as we skip `ls-refs`.
    refs: Vec<handshake::Ref>,

    /// Refs advertised as `wanted-ref` -- should always match `want_refs`
    wanted_refs: Vec<handshake::Ref>,
}

impl fetch::DelegateBlocking for CloneRefInWantDelegate {
    fn prepare_ls_refs(
        &mut self,
        _server: &Capabilities,
        _arguments: &mut Vec<BString>,
        _features: &mut Vec<(&str, Option<Cow<'_, str>>)>,
    ) -> io::Result<ls_refs::Action> {
        Ok(ls_refs::Action::Skip)
    }

    fn prepare_fetch(
        &mut self,
        _version: gix_transport::Protocol,
        _server: &Capabilities,
        _features: &mut Vec<(&str, Option<Cow<'_, str>>)>,
        refs: &[handshake::Ref],
    ) -> io::Result<Action> {
        self.refs = refs.to_owned();
        Ok(Action::Continue)
    }

    fn negotiate(
        &mut self,
        _refs: &[handshake::Ref],
        arguments: &mut Arguments,
        _prev: Option<&Response>,
    ) -> io::Result<Action> {
        for wanted_ref in &self.want_refs {
            arguments.want_ref(wanted_ref.as_ref())
        }

        Ok(Action::Cancel)
    }
}

#[derive(Default)]
pub struct LsRemoteDelegate {
    refs: Vec<handshake::Ref>,
    abort_with: Option<std::io::Error>,
}

impl fetch::DelegateBlocking for LsRemoteDelegate {
    fn handshake_extra_parameters(&self) -> Vec<(String, Option<String>)> {
        vec![("value-only".into(), None), ("key".into(), Some("value".into()))]
    }
    fn prepare_ls_refs(
        &mut self,
        _server: &Capabilities,
        _arguments: &mut Vec<BString>,
        _features: &mut Vec<(&str, Option<Cow<'_, str>>)>,
    ) -> std::io::Result<ls_refs::Action> {
        match self.abort_with.take() {
            Some(err) => Err(err),
            None => Ok(ls_refs::Action::Continue),
        }
    }
    fn prepare_fetch(
        &mut self,
        _version: gix_transport::Protocol,
        _server: &Capabilities,
        _features: &mut Vec<(&str, Option<Cow<'_, str>>)>,
        refs: &[handshake::Ref],
    ) -> io::Result<fetch::Action> {
        self.refs = refs.to_owned();
        Ok(fetch::Action::Cancel)
    }
    fn negotiate(
        &mut self,
        _refs: &[handshake::Ref],
        _arguments: &mut Arguments,
        _previous_response: Option<&Response>,
    ) -> io::Result<Action> {
        unreachable!("this must not be called after closing the connection in `prepare_fetch(…)`")
    }
}

#[cfg(feature = "blocking-client")]
mod blocking_io {
    use gix_features::progress::NestedProgress;
    use std::io;

    use gix_protocol::{fetch, fetch::Response, handshake, handshake::Ref};

    use crate::fetch::{CloneDelegate, CloneRefInWantDelegate, LsRemoteDelegate};

    impl fetch::Delegate for CloneDelegate {
        fn receive_pack(
            &mut self,
            mut input: impl io::BufRead,
            _progress: impl NestedProgress,
            _refs: &[Ref],
            _previous_response: &Response,
        ) -> io::Result<()> {
            self.pack_bytes = io::copy(&mut input, &mut io::sink())? as usize;
            Ok(())
        }
    }

    impl fetch::Delegate for CloneRefInWantDelegate {
        fn receive_pack(
            &mut self,
            mut input: impl io::BufRead,
            _progress: impl NestedProgress,
            _refs: &[Ref],
            response: &Response,
        ) -> io::Result<()> {
            for wanted in response.wanted_refs() {
                self.wanted_refs.push(handshake::Ref::Direct {
                    full_ref_name: wanted.path.clone(),
                    object: wanted.id,
                });
            }
            self.pack_bytes = io::copy(&mut input, &mut io::sink())? as usize;
            Ok(())
        }
    }

    impl fetch::Delegate for LsRemoteDelegate {
        fn receive_pack(
            &mut self,
            _input: impl io::BufRead,
            _progress: impl NestedProgress,
            _refs: &[Ref],
            _previous_response: &Response,
        ) -> io::Result<()> {
            unreachable!("Should not be called for ls-refs");
        }
    }
}

#[cfg(feature = "async-client")]
mod async_io {
    use std::io;

    use async_trait::async_trait;
    use futures_io::AsyncBufRead;
    use gix_features::progress::NestedProgress;
    use gix_protocol::{fetch, fetch::Response, handshake, handshake::Ref};

    use crate::fetch::{CloneDelegate, CloneRefInWantDelegate, LsRemoteDelegate};

    #[async_trait(?Send)]
    impl fetch::Delegate for CloneDelegate {
        async fn receive_pack(
            &mut self,
            mut input: impl AsyncBufRead + Unpin + 'async_trait,
            _progress: impl NestedProgress,
            _refs: &[Ref],
            _previous_response: &Response,
        ) -> io::Result<()> {
            self.pack_bytes = futures_lite::io::copy(&mut input, &mut futures_lite::io::sink()).await? as usize;
            Ok(())
        }
    }

    #[async_trait(?Send)]
    impl fetch::Delegate for CloneRefInWantDelegate {
        async fn receive_pack(
            &mut self,
            mut input: impl AsyncBufRead + Unpin + 'async_trait,
            _progress: impl NestedProgress,
            _refs: &[Ref],
            response: &Response,
        ) -> io::Result<()> {
            for wanted in response.wanted_refs() {
                self.wanted_refs.push(handshake::Ref::Direct {
                    full_ref_name: wanted.path.clone(),
                    object: wanted.id,
                });
            }
            self.pack_bytes = futures_lite::io::copy(&mut input, &mut futures_lite::io::sink()).await? as usize;
            Ok(())
        }
    }

    #[async_trait(?Send)]
    impl fetch::Delegate for LsRemoteDelegate {
        async fn receive_pack(
            &mut self,
            _input: impl AsyncBufRead + Unpin + 'async_trait,
            _progress: impl NestedProgress,
            _refs: &[Ref],
            _previous_response: &Response,
        ) -> io::Result<()> {
            unreachable!("Should not be called for ls-refs");
        }
    }
}

pub fn oid(hex_sha: &str) -> gix_hash::ObjectId {
    gix_hash::ObjectId::from_hex(hex_sha.as_bytes()).expect("valid input")
}

#[cfg(feature = "async-client")]
pub fn transport<W: futures_io::AsyncWrite + Unpin>(
    out: W,
    path: &str,
    desired_version: gix_transport::Protocol,
    mode: gix_transport::client::git::ConnectMode,
) -> gix_transport::client::git::Connection<Cursor, W> {
    let response = fixture_bytes(path);
    gix_transport::client::git::Connection::new(
        Cursor::new(response),
        out,
        desired_version,
        b"does/not/matter".as_bstr().to_owned(),
        None::<(&str, _)>,
        mode,
        false,
    )
}

#[cfg(feature = "blocking-client")]
pub fn transport<W: std::io::Write>(
    out: W,
    path: &str,
    version: gix_transport::Protocol,
    mode: gix_transport::client::git::ConnectMode,
) -> gix_transport::client::git::Connection<Cursor, W> {
    let response = fixture_bytes(path);
    gix_transport::client::git::Connection::new(
        Cursor::new(response),
        out,
        version,
        b"does/not/matter".as_bstr().to_owned(),
        None::<(&str, _)>,
        mode,
        false,
    )
}

pub mod response;
mod v1;
mod v2;
