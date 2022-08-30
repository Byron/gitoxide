use crate::program::Cascade;
use crate::protocol::Context;
use crate::{helper, protocol, Program};

impl Default for Cascade {
    fn default() -> Self {
        Cascade {
            programs: Vec::new(),
            stderr: true,
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

        Cascade { programs, stderr: true }
    }
}

/// Builder
impl Cascade {
    /// Extend the list of programs to run `programs`.
    pub fn extend(mut self, programs: impl IntoIterator<Item = Program>) -> Self {
        self.programs.extend(programs);
        self
    }
}

/// Finalize
impl Cascade {
    /// Invoke the cascade by `invoking` each program with `action`, and configuring potential prompts with `prompt` options.
    /// The latter can also be used to disable the prompt entirely when setting the `mode` to [`Disable`][git_prompt::Mode::Disable];=.
    ///
    /// When _getting_ credentials, all programs are asked until the credentials are complete, stopping the cascade.
    /// When _storing_ or _erasing_ all programs are instructed in order.
    pub fn invoke(&mut self, mut action: helper::Action, mut prompt: git_prompt::Options<'_>) -> protocol::Result {
        action.context_mut().map(Context::destructure_url).transpose()?;

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
                            dst_ctx.destructure_url()?;
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

        if prompt.mode != git_prompt::Mode::Disable {
            if let Some(ctx) = action.context_mut() {
                if ctx.username.is_none() {
                    let message = ctx.to_prompt("Username");
                    prompt.mode = git_prompt::Mode::Visible;
                    ctx.username = git_prompt::ask(&message, &prompt)
                        .map_err(|err| protocol::Error::Prompt {
                            prompt: message,
                            source: err,
                        })?
                        .into();
                }
                if ctx.password.is_none() {
                    let message = ctx.to_prompt("Password");
                    prompt.mode = git_prompt::Mode::Hidden;
                    ctx.password = git_prompt::ask(&message, &prompt)
                        .map_err(|err| protocol::Error::Prompt {
                            prompt: message,
                            source: err,
                        })?
                        .into();
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
