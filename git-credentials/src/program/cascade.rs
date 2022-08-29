use crate::program::Cascade;
use crate::protocol::Context;
use crate::{helper, protocol, Program};
use bstr::ByteSlice;

impl Default for Cascade {
    fn default() -> Self {
        Cascade {
            programs: Vec::new(),
            stderr: true,
            #[cfg(feature = "prompt")]
            prompt: true,
        }
    }
}

/// Initialization
impl Cascade {
    /// Return an instance configured to run the `git credential-<platform>` program for the current platform first, followed
    /// by additional programs pushed onto [`programs`][Self::programs].
    ///
    /// It's the basis for adding more programs according to the caller which run in succession.
    pub fn platform_builtin() -> Self {
        let programs = if cfg!(target_os = "macos") {
            Some("osxkeychain")
        } else if cfg!(target_os = "linux") {
            Some("libsecret")
        } else if cfg!(target_os = "windows") {
            Some("wincred")
        } else {
            None
        }
        .map(|name| vec![Program::from_custom_definition(name)])
        .unwrap_or_default();

        Cascade {
            programs,
            stderr: true,
            #[cfg(feature = "prompt")]
            prompt: true,
        }
    }
}

/// Builder
impl Cascade {
    /// Disable prompting to assure we only interact with stored or already present credentials.
    ///
    /// Note that this is only meaningful with the `prompt` feature enabled.
    #[cfg_attr(not(feature = "prompt"), allow(unused_mut))]
    pub fn disable_prompt(mut self) -> Self {
        #[cfg(feature = "prompt")]
        {
            self.prompt = false;
        }
        self
    }
    /// Extend the list of programs to run `programs`.
    pub fn extend(mut self, programs: impl IntoIterator<Item = Program>) -> Self {
        self.programs.extend(programs);
        self
    }
}

/// Finalize
impl Cascade {
    /// Invoke the cascade by `invoking` each program with `action`.
    ///
    /// When _getting_ credentials, all programs are asked until the credentials are complete, stopping the cascade.
    /// When _storing_ or _erasing_ all programs are instructed in order.
    pub fn invoke(&mut self, mut action: helper::Action) -> protocol::Result {
        fn store_url_parts(ctx: &mut Context) -> Result<(), protocol::Error> {
            let url = git_url::parse(ctx.url.as_ref().ok_or(protocol::Error::UrlMissing)?.as_ref())?;
            ctx.protocol = Some(url.scheme.as_str().into());
            ctx.host = url.host().map(ToOwned::to_owned).map(|mut host| {
                if let Some(port) = url.port {
                    use std::fmt::Write;
                    write!(host, ":{}", port).expect("infallible");
                }
                host
            });
            let path = url.path.trim_with(|b| b == '/');
            ctx.path = (!path.is_empty()).then(|| path.into());
            Ok(())
        }
        action.context_mut().map(store_url_parts).transpose()?;

        for program in &mut self.programs {
            program.stderr = self.stderr;
            match helper::invoke::raw(program, &action) {
                Ok(None) => {}
                Ok(Some(stdout)) => {
                    let ctx = Context::from_bytes(&stdout)?;
                    if let Some(dst_ctx) = action.context_mut() {
                        if let Some(src) = ctx.path {
                            dst_ctx.path = Some(src);
                        }
                        for (src, dst) in [
                            (ctx.protocol, &mut dst_ctx.protocol),
                            (ctx.host, &mut dst_ctx.host),
                            (ctx.username, &mut dst_ctx.username),
                            (ctx.password, &mut dst_ctx.password),
                        ] {
                            if let Some(src) = src {
                                *dst = Some(src);
                            }
                        }
                        if let Some(src) = ctx.url {
                            dst_ctx.url = Some(src);
                            store_url_parts(dst_ctx)?;
                        }
                        if dst_ctx.username.is_some() && dst_ctx.password.is_some() {
                            break;
                        }
                        if ctx.quit.unwrap_or_default() {
                            dst_ctx.quit = ctx.quit;
                            break;
                        }
                    }
                }
                Err(helper::Error::CredentialsHelperFailed { .. }) => continue, // ignore helpers that we can't call
                Err(err) if action.context().is_some() => return Err(err.into()), // communication errors are fatal when getting credentials
                Err(_) => {} // for other actions, ignore everything, try the operation
            }
        }

        #[cfg(feature = "prompt")]
        if self.prompt {
            if let Some(ctx) = action.context_mut() {
                if let username @ None = &mut ctx.username {
                    *username = git_prompt::openly("Username: ")?.into();
                }
                if let password @ None = &mut ctx.password {
                    *password = git_prompt::securely("Password: ")?.into();
                }
            }
        }

        protocol::helper_outcome_to_result(
            action.context().map(|ctx| helper::Outcome {
                username: ctx.username.clone(),
                password: ctx.password.clone(),
                quit: ctx.quit.unwrap_or(false),
                next: ctx.to_owned().into(),
            }),
            action,
        )
    }
}
