use crate::remote::Connection;
use git_protocol::transport::client::Transport;

struct Delegate {
    refs: Vec<git_protocol::fetch::Ref>,
}

mod delegate {
    use super::Delegate;
    use git_protocol::fetch::Action;
    use git_protocol::transport;

    impl git_protocol::fetch::DelegateBlocking for Delegate {
        fn prepare_fetch(
            &mut self,
            _version: transport::Protocol,
            _server: &transport::client::Capabilities,
            _features: &mut Vec<(&str, Option<&str>)>,
            refs: &[git_protocol::fetch::Ref],
        ) -> std::io::Result<Action> {
            self.refs = refs.into();
            Ok(Action::Cancel)
        }

        fn negotiate(
            &mut self,
            _refs: &[git_protocol::fetch::Ref],
            _arguments: &mut git_protocol::fetch::Arguments,
            _previous_response: Option<&git_protocol::fetch::Response>,
        ) -> std::io::Result<Action> {
            unreachable!("not to be called due to Action::Close in `prepare_fetch`")
        }
    }

    #[cfg(feature = "blocking-network-client")]
    mod blocking_io {
        impl git_protocol::fetch::Delegate for super::Delegate {
            fn receive_pack(
                &mut self,
                _input: impl std::io::BufRead,
                _progress: impl git_features::progress::Progress,
                _refs: &[git_protocol::fetch::Ref],
                _previous_response: &git_protocol::fetch::Response,
            ) -> std::io::Result<()> {
                unreachable!("not called for ls-refs")
            }
        }
    }

    #[cfg(feature = "async-network-client")]
    mod async_io {
        use git_protocol::async_trait::async_trait;
        use git_protocol::futures_io::AsyncBufRead;

        #[async_trait(? Send)]
        impl git_protocol::fetch::Delegate for super::Delegate {
            async fn receive_pack(
                &mut self,
                _input: impl AsyncBufRead + Unpin + 'async_trait,
                _progress: impl git_features::progress::Progress,
                _refs: &[git_protocol::fetch::Ref],
                _previous_response: &git_protocol::fetch::Response,
            ) -> std::io::Result<()> {
                unreachable!("not called for ls-refs")
            }
        }
    }
}

impl<'repo, T> Connection<'repo, T>
where
    T: Transport,
{
    /// List all references on the remote that have been filtered through our remote's [`refspecs`][crate::Remote::refspecs()]
    /// for _fetching_.
    ///
    /// This comes in the form of information of all matching tips on the remote and the object they point to, along with
    /// with the local tracking branch of these tips (if available).
    ///
    /// Note that this doesn't fetch the objects mentioned in the tips nor does it make any change to underlying repository.
    pub fn refs(&mut self) -> ! {
        todo!()
    }
}
