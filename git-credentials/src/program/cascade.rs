use crate::helper::Action;
use crate::program::Cascade;
use crate::{helper, protocol, Program};

/// Initialization
impl Cascade {
    /// Return an instance configured to run the `git credential-<platform>` program for the current platform first, followed
    /// by additional programs pushed onto [`programs`][Self::prorgams].
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

        Cascade { programs }
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
    /// Invoke the cascade by `invoking` each program with `action`.
    ///
    /// When _getting_ credentials, all programs are asked until the credentials are complete, stopping the cascade.
    /// When _storing_ or _erasing_ all programs are instructed in order.
    pub fn invoke(&mut self, mut action: helper::Action) -> protocol::Result {
        let mut fill_ctx = action
            .expects_output()
            .then(|| action.context().map(ToOwned::to_owned))
            .flatten();
        for program in &mut self.programs {
            match helper::invoke(program, &action) {
                Ok(None) => {}
                Ok(Some(outcome)) => {
                    if let Some(fill_ctx) = fill_ctx.as_mut() {
                        let action_needs_update = if let v @ None = &mut fill_ctx.username {
                            *v = outcome.username;
                            true
                        } else if let v @ None = &mut fill_ctx.password {
                            *v = outcome.password;
                            true
                        } else if let v @ None = &mut fill_ctx.quit {
                            if outcome.quit {
                                *v = outcome.quit.into();
                                break;
                            }
                            true
                        } else {
                            false
                        };
                        if action_needs_update {
                            action = Action::Get(fill_ctx.clone());
                        }
                    }
                }
                Err(helper::Error::CredentialsHelperFailed { .. }) => continue, // ignore helpers that we can't call
                Err(err) if fill_ctx.is_some() => return Err(err.into()), // communication errors are fatal when getting credentials
                Err(_) => {} // for other actions, ignore everything, try the operation
            }
        }

        protocol::helper_outcome_to_result(
            fill_ctx.map(|ctx| helper::Outcome {
                username: ctx.username.clone(),
                password: ctx.password.clone(),
                quit: ctx.quit.unwrap_or(false),
                next: ctx.into(),
            }),
            action,
        )
    }
}
