use bstr::{BString, ByteSlice};
use std::io;

use git_protocol::fetch::{self, Action, Arguments, LsRefsAction, Ref, Response};
use git_transport::client::Capabilities;

use crate::fixture_bytes;

#[cfg(feature = "blocking-client")]
type Cursor = std::io::Cursor<Vec<u8>>;
#[cfg(feature = "async-client")]
type Cursor = futures_lite::io::Cursor<Vec<u8>>;

#[derive(Default)]
pub struct CloneDelegate {
    pack_bytes: usize,
    abort_with: Option<std::io::Error>,
}

impl fetch::DelegateBlocking for CloneDelegate {
    fn prepare_fetch(
        &mut self,
        _version: git_transport::Protocol,
        _server: &Capabilities,
        _features: &mut Vec<(&str, Option<&str>)>,
        _refs: &[fetch::Ref],
    ) -> io::Result<Action> {
        match self.abort_with.take() {
            Some(err) => Err(err),
            None => Ok(Action::Continue),
        }
    }
    fn negotiate(
        &mut self,
        refs: &[Ref],
        arguments: &mut Arguments,
        _previous_response: Option<&Response>,
    ) -> io::Result<Action> {
        for r in refs {
            arguments.want(r.unpack().1);
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
    refs: Vec<fetch::Ref>,

    /// Refs advertised as `wanted-ref` -- should always match `want_refs`
    wanted_refs: Vec<fetch::Ref>,
}

impl fetch::DelegateBlocking for CloneRefInWantDelegate {
    fn prepare_ls_refs(
        &mut self,
        _server: &Capabilities,
        _arguments: &mut Vec<BString>,
        _features: &mut Vec<(&str, Option<&str>)>,
    ) -> io::Result<LsRefsAction> {
        Ok(LsRefsAction::Skip)
    }

    fn prepare_fetch(
        &mut self,
        _version: git_transport::Protocol,
        _server: &Capabilities,
        _features: &mut Vec<(&str, Option<&str>)>,
        refs: &[fetch::Ref],
    ) -> io::Result<Action> {
        self.refs = refs.to_owned();
        Ok(Action::Continue)
    }

    fn negotiate(&mut self, _refs: &[Ref], arguments: &mut Arguments, _prev: Option<&Response>) -> io::Result<Action> {
        for wanted_ref in &self.want_refs {
            arguments.want_ref(wanted_ref.as_ref())
        }

        Ok(Action::Cancel)
    }
}

#[derive(Default)]
pub struct LsRemoteDelegate {
    refs: Vec<fetch::Ref>,
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
        _features: &mut Vec<(&str, Option<&str>)>,
    ) -> std::io::Result<LsRefsAction> {
        match self.abort_with.take() {
            Some(err) => Err(err),
            None => Ok(LsRefsAction::Continue),
        }
    }
    fn prepare_fetch(
        &mut self,
        _version: git_transport::Protocol,
        _server: &Capabilities,
        _features: &mut Vec<(&str, Option<&str>)>,
        refs: &[fetch::Ref],
    ) -> io::Result<fetch::Action> {
        self.refs = refs.to_owned();
        Ok(fetch::Action::Cancel)
    }

    fn negotiate(
        &mut self,
        _refs: &[Ref],
        _arguments: &mut Arguments,
        _previous_response: Option<&Response>,
    ) -> io::Result<Action> {
        unreachable!("this must not be called after closing the connection in `prepare_fetch(…)`")
    }
}

#[cfg(feature = "blocking-client")]
mod blocking_io {
    use crate::fetch::{CloneDelegate, CloneRefInWantDelegate, LsRemoteDelegate};
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
            _progress: impl Progress,
            _refs: &[Ref],
            response: &Response,
        ) -> io::Result<()> {
            for wanted in response.wanted_refs() {
                self.wanted_refs.push(fetch::Ref::Direct {
                    path: wanted.path.clone(),
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
            _progress: impl Progress,
            _refs: &[Ref],
            _previous_response: &Response,
        ) -> io::Result<()> {
            unreachable!("Should not be called for ls-refs");
        }
    }
}

#[cfg(feature = "async-client")]
mod async_io {
    use crate::fetch::{CloneDelegate, CloneRefInWantDelegate, LsRemoteDelegate};
    use async_trait::async_trait;
    use futures_io::AsyncBufRead;
    use git_features::progress::Progress;
    use git_protocol::{
        fetch,
        fetch::{Ref, Response},
    };
    use std::io;

    #[async_trait(?Send)]
    impl fetch::Delegate for CloneDelegate {
        async fn receive_pack(
            &mut self,
            mut input: impl AsyncBufRead + Unpin + 'async_trait,
            _progress: impl Progress,
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
            _progress: impl Progress,
            _refs: &[Ref],
            response: &Response,
        ) -> io::Result<()> {
            for wanted in response.wanted_refs() {
                self.wanted_refs.push(fetch::Ref::Direct {
                    path: wanted.path.clone(),
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
            _progress: impl Progress,
            _refs: &[Ref],
            _previous_response: &Response,
        ) -> io::Result<()> {
            unreachable!("Should not be called for ls-refs");
        }
    }
}

pub fn oid(hex_sha: &str) -> git_hash::ObjectId {
    git_hash::ObjectId::from_hex(hex_sha.as_bytes()).expect("valid input")
}

#[cfg(feature = "async-client")]
pub fn transport<W: futures_io::AsyncWrite + Unpin>(
    out: W,
    path: &str,
    desired_version: git_transport::Protocol,
    mode: git_transport::client::git::ConnectMode,
) -> git_transport::client::git::Connection<Cursor, W> {
    let response = fixture_bytes(path);
    git_transport::client::git::Connection::new(
        Cursor::new(response),
        out,
        desired_version,
        b"does/not/matter".as_bstr().to_owned(),
        None::<(&str, _)>,
        mode,
    )
}

#[cfg(feature = "blocking-client")]
pub fn transport<W: std::io::Write>(
    out: W,
    path: &str,
    version: git_transport::Protocol,
    mode: git_transport::client::git::ConnectMode,
) -> git_transport::client::git::Connection<Cursor, W> {
    let response = fixture_bytes(path);
    git_transport::client::git::Connection::new(
        Cursor::new(response),
        out,
        version,
        b"does/not/matter".as_bstr().to_owned(),
        None::<(&str, _)>,
        mode,
    )
}

pub mod response;
mod v1;
mod v2;
