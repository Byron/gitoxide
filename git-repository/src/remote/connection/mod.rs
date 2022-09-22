use crate::Remote;

pub(crate) struct HandshakeWithRefs {
    outcome: git_protocol::fetch::handshake::Outcome,
    refs: Vec<git_protocol::fetch::Ref>,
}

/// A function that performs a given credential action.
pub type CredentialsFn<'a> = Box<dyn FnMut(git_credentials::helper::Action) -> git_credentials::protocol::Result + 'a>;

/// A type to represent an ongoing connection to a remote host, typically with the connection already established.
///
/// It can be used to perform a variety of operations with the remote without worrying about protocol details,
/// much like a remote procedure call.
pub struct Connection<'a, 'repo, T, P> {
    pub(crate) remote: &'a Remote<'repo>,
    pub(crate) credentials: Option<CredentialsFn<'a>>,
    pub(crate) transport: T,
    pub(crate) progress: P,
}

mod access {
    use crate::{
        remote::{connection::CredentialsFn, Connection},
        Remote,
    };
    use std::any::Any;

    /// Builder
    impl<'a, 'repo, T, P> Connection<'a, 'repo, T, P> {
        /// Set a custom credentials callback to provide credentials if the remotes require authentication.
        ///
        /// Otherwise we will use the git configuration to perform the same task as the `git credential` helper program,
        /// which is calling other helper programs in succession while resorting to a prompt to obtain credentials from the
        /// user.
        ///
        /// A custom function may also be used to prevent accessing resources with authentication.
        pub fn with_credentials(
            mut self,
            helper: impl FnMut(git_credentials::helper::Action) -> git_credentials::protocol::Result + 'a,
        ) -> Self {
            self.credentials = Some(Box::new(helper));
            self
        }
    }

    /// Access
    impl<'a, 'repo, T, P> Connection<'a, 'repo, T, P> {
        /// A utility to return a function that will use this repository's configuration to obtain credentials, similar to
        /// what `git credential` is doing.
        ///
        /// It's meant to be used by users of the [`with_credentials()`][Self::with_credentials()] builder to gain access to the
        /// default way of handling credentials, which they can call as fallback.
        pub fn configured_credentials(
            &self,
            url: git_url::Url,
        ) -> Result<CredentialsFn<'a>, crate::config::credential_helpers::Error> {
            let (mut cascade, _action_with_normalized_url, prompt_opts) =
                self.remote.repo.config_snapshot().credential_helpers(url)?;
            Ok(Box::new(move |action| cascade.invoke(action, prompt_opts.clone())) as CredentialsFn<'_>)
        }
        /// Drop the transport and additional state to regain the original remote.
        pub fn remote(&self) -> &Remote<'repo> {
            self.remote
        }

        /// Return the connection's transport.
        pub fn transport(&self) -> &T {
            &self.transport
        }
    }

    /// Access to the transport if it can be downcast to a particular type.
    impl<'a, 'repo, T, P> Connection<'a, 'repo, T, P>
    where
        T: crate::protocol::transport::client::Transport + 'static,
    {
        /// Try to cast our transport `T` into `U`, and pass it to `f` to allow any kind of configuration.
        ///
        /// Note that if the case fails and `f` is not called at all, `false` is returned.
        pub fn configure_transport<U: 'static, E>(
            &mut self,
            f: impl FnOnce(&mut U) -> Result<(), E>,
        ) -> Result<bool, E> {
            let transport = (&mut self.transport) as &mut dyn Any;
            match transport.downcast_mut::<U>() {
                Some(transport) => f(transport).map(|_| true),
                None => Ok(false),
            }
        }
    }
}

///
pub mod ref_map;

///
#[cfg(feature = "blocking-network-client")]
pub mod fetch;
